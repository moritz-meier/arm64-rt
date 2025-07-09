use core::{arch::asm, ops::RangeInclusive, usize};

use crate::{bitmask, read_bitfield};

mod dcache;
mod icache;

pub use dcache::*;
pub use icache::*;

#[derive(Clone, Copy)]
pub enum Cache {
    Instruction { idx: usize },
    DataOrUnified { idx: usize },
}

impl Cache {
    pub const I_L1: Self = Self::Instruction { idx: 0 };
    pub const I_L2: Self = Self::Instruction { idx: 1 };
    pub const I_L3: Self = Self::Instruction { idx: 2 };

    pub const D_L1: Self = Self::DataOrUnified { idx: 0 };
    pub const D_L2: Self = Self::DataOrUnified { idx: 1 };
    pub const D_L3: Self = Self::DataOrUnified { idx: 2 };
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CacheOp {
    Clean,
    Invalidate,
    CleanInvalidate,
}

#[derive(Clone, Copy)]
pub struct CacheInfo {
    pub cache: Cache,
    pub linesize: usize,
    pub num_ways: usize,
    pub num_sets: usize,
}

impl CacheInfo {
    pub fn get(cache: Cache) -> Option<Self> {
        let (idx, icache) = match cache {
            Cache::Instruction { idx } => (idx, true),
            Cache::DataOrUnified { idx } => (idx, false),
        };

        let Caches { impls, .. } = Caches::get();

        let Some(cache_impl) = impls.get(idx) else {
            return None;
        };

        if !cache_impl.contains(cache) {
            return None;
        }

        let id_aa64mmfr2_el1: u64;
        let ccsidr_el1: u64;
        let csselr_el1: u64 = ((idx as u64) << 1) | ({ if icache { 0b1 } else { 0b0 } } << 0);
        unsafe {
            asm!(
                "msr CSSELR_EL1, {csselr_el1}",
                "mrs {ccsidr_el1}, CCSIDR_EL1",
                "mrs {id_aa64mmfr2_el1}, ID_AA64MMFR2_EL1",
                csselr_el1 = in(reg) csselr_el1,
                ccsidr_el1 = lateout(reg) ccsidr_el1,
                id_aa64mmfr2_el1 = lateout(reg) id_aa64mmfr2_el1
            );
        }

        let feat_ccidx = read_bitfield!(id_aa64mmfr2_el1, msb: 23, lsb: 20);

        let (sets, ways, line) = if feat_ccidx > 0 {
            let sets = read_bitfield!(ccsidr_el1, msb: 55, lsb: 32);
            let ways = read_bitfield!(ccsidr_el1, msb: 23, lsb: 3);
            let line = read_bitfield!(ccsidr_el1, msb: 2, lsb: 0);

            (sets, ways, line)
        } else {
            let sets = read_bitfield!(ccsidr_el1, msb: 27, lsb: 13);
            let ways = read_bitfield!(ccsidr_el1, msb: 12, lsb: 3);
            let line = read_bitfield!(ccsidr_el1, msb: 2, lsb: 0);

            (sets, ways, line)
        };

        Some(CacheInfo {
            cache,
            linesize: 1 << (line + 4),
            num_ways: ways as usize + 1,
            num_sets: sets as usize + 1,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CacheImpl {
    NoCache,
    InstructionOnly,
    DataOnly,
    SeperateInstructionAndData,
    Unified,
}

impl CacheImpl {
    pub fn contains(self, cache: Cache) -> bool {
        match (self, cache) {
            (Self::InstructionOnly, Cache::Instruction { .. }) => true,
            (Self::DataOnly, Cache::DataOrUnified { .. }) => true,
            (Self::SeperateInstructionAndData, _) => true,
            (Self::Unified, Cache::DataOrUnified { .. }) => true,
            _ => false,
        }
    }
}

pub struct Caches {
    pub levels: RangeInclusive<usize>,
    pub impls: [CacheImpl; 7],

    pub level_of_unification_inner_shareable: usize,
    pub level_of_coherence: usize,
    pub level_of_unification_uniprocessor: usize,
    pub inner_cache_boundary: Option<usize>,
}

impl Caches {
    pub fn get() -> Self {
        let mut impls = [CacheImpl::NoCache; 7];

        let clidr_el1: u64 = unsafe {
            let value;
            asm!(
                "mrs {}, CLIDR_EL1",
                lateout(reg) value
            );
            value
        };

        let louis = read_bitfield!(clidr_el1, msb: 23, lsb: 21) as usize;
        let loc = read_bitfield!(clidr_el1, msb: 26, lsb: 24) as usize;
        let louu = read_bitfield!(clidr_el1, msb: 29, lsb: 27) as usize;
        let icb = read_bitfield!(clidr_el1, msb: 32, lsb: 30) as usize;

        let mut levels = 0;
        for level in 0..7 {
            let mask = bitmask!(msb: 2, lsb: 0) << level;
            let typ = (clidr_el1 & mask) >> level;
            if typ == 0b000 || typ > 0b100 {
                break;
            }

            levels += 1;

            impls[level] = match typ {
                0b000 => CacheImpl::NoCache,
                0b001 => CacheImpl::InstructionOnly,
                0b010 => CacheImpl::DataOnly,
                0b011 => CacheImpl::SeperateInstructionAndData,
                0b100 => CacheImpl::Unified,
                _ => CacheImpl::NoCache,
            };
        }

        return Self {
            levels: 1..=levels,
            impls,
            level_of_unification_inner_shareable: louis as usize,
            level_of_coherence: loc,
            level_of_unification_uniprocessor: louu,
            inner_cache_boundary: if icb > 0 { Some(icb) } else { None },
        };
    }
}
