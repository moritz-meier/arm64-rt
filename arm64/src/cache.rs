use core::{ops::RangeInclusive, usize};

use arbitrary_int::*;

mod dcache;
mod icache;

pub use dcache::*;
pub use icache::*;

use crate::sys_regs::*;

#[derive(Clone, Copy)]
pub enum Cache {
    Instruction { idx: u8 },
    DataOrUnified { idx: u8 },
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
    pub linesize: u16,
    pub num_ways: u32,
    pub num_sets: u32,
}

impl CacheInfo {
    pub fn get(cache: Cache) -> Option<Self> {
        let (idx, icache) = match cache {
            Cache::Instruction { idx } => (idx, true),
            Cache::DataOrUnified { idx } => (idx, false),
        };

        let Caches { impls, .. } = Caches::get();

        let Some(cache_impl) = impls.get(idx as usize) else {
            return None;
        };

        if !cache_impl.contains(cache) {
            return None;
        }

        CSSELR_EL1.modify(|csselr_el1| csselr_el1.with_LEVEL(u3::from_u8(idx)).with_InD(icache));

        let feat_ccidx = ID_AA64MMFR2_EL1.read().CCIDX();
        if feat_ccidx.value() > 0 {
            let ccsidr_el1 = CCSIDR_EL1_CCIDX.read();
            let sets = ccsidr_el1.NUM_SETS().value();
            let ways = ccsidr_el1.ASSOCIATIVITY().value();
            let line = ccsidr_el1.LINE_SIZE().value();

            Some(CacheInfo {
                cache,
                linesize: 1 << (line + 4),
                num_ways: ways + 1,
                num_sets: sets + 1,
            })
        } else {
            let ccsidr_el1 = CCSIDR_EL1_NO_CCIDX.read();
            let sets = ccsidr_el1.NUM_SETS().value();
            let ways = ccsidr_el1.ASSOCIATIVITY().value();
            let line = ccsidr_el1.LINE_SIZE().value();

            Some(CacheInfo {
                cache,
                linesize: 1 << (line + 4),
                num_ways: ways as u32 + 1,
                num_sets: sets as u32 + 1,
            })
        }
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
    pub levels: RangeInclusive<u8>,
    pub impls: [CacheImpl; 7],

    pub level_of_unification_inner_shareable: u8,
    pub level_of_coherence: u8,
    pub level_of_unification_uniprocessor: u8,
    pub inner_cache_boundary: Option<u8>,
}

impl Caches {
    pub fn get() -> Self {
        let mut impls = [CacheImpl::NoCache; 7];

        let clidr_el1 = CLIDR_EL1.read();
        let louis = clidr_el1.LOUIS().value();
        let loc = clidr_el1.LOC().value();
        let louu = clidr_el1.LOUU().value();
        let icb = clidr_el1.ICB().value();

        let mut levels = 0;
        for level in 0..7 {
            let mask = 0b111 << level;
            let typ = (clidr_el1.raw_value() & mask) >> level;
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
            level_of_unification_inner_shareable: louis,
            level_of_coherence: loc,
            level_of_unification_uniprocessor: louu,
            inner_cache_boundary: if icb > 0 { Some(icb) } else { None },
        };
    }
}
