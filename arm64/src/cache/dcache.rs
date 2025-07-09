use core::arch::asm;

use crate::{bitmask, dsb, isb, sysreg_read_bitfield, sysreg_write_bitfield};

use super::*;

pub struct DCache;

impl DCache {
    pub fn enable() {
        Self::op_all(CacheOp::Invalidate);

        let current_el: usize = sysreg_read_bitfield!("CurrentEL", msb: 3, lsb: 2);

        dsb!("sy");

        if current_el == 3 {
            sysreg_write_bitfield!("SCTLR_EL3", bit: 2, value: 0b1);
        }

        if current_el == 2 {
            sysreg_write_bitfield!("SCTLR_EL2", bit: 2, value: 0b1);
        }

        if current_el == 1 {
            sysreg_write_bitfield!("SCTLR_EL1", bit: 2, value: 0b1);
        }

        isb!("sy");
    }

    pub fn disable() {
        let current_el: usize = sysreg_read_bitfield!("CurrentEL", msb: 3, lsb: 2);

        dsb!("sy");

        if current_el == 3 {
            sysreg_write_bitfield!("SCTLR_EL3", bit: 2, value: 0b0);
        }

        if current_el == 2 {
            sysreg_write_bitfield!("SCTLR_EL2", bit: 2, value: 0b0);
        }

        if current_el == 1 {
            sysreg_write_bitfield!("SCTLR_EL1", bit: 2, value: 0b0);
        }

        isb!("sy");
    }

    pub fn op_all(op: CacheOp) {
        let Caches {
            level_of_coherence, ..
        } = Caches::get();

        for level in 0..level_of_coherence {
            let Some(info) = CacheInfo::get(Cache::DataOrUnified { idx: level }) else {
                return;
            };

            let way_shift = 32 - info.num_ways.next_power_of_two().ilog2();
            let set_shift = info.num_sets.next_power_of_two().ilog2();

            dsb!("sy");

            for set in 0..info.num_sets {
                for way in 0..info.num_ways {
                    let way_set_level =
                        (way as u32) << way_shift | (set as u32) << set_shift | (level as u32) << 1;
                    match op {
                        CacheOp::Clean => unsafe { asm!("dc isw, {:x}", in(reg) way_set_level) },
                        CacheOp::Invalidate => unsafe {
                            asm!("dc csw, {:x}", in(reg) way_set_level)
                        },
                        CacheOp::CleanInvalidate => unsafe {
                            asm!("dc cisw, {:x}", in(reg) way_set_level)
                        },
                    }
                }
            }
        }

        isb!("sy");
    }

    pub fn op_range<T>(op: CacheOp, range: RangeInclusive<*const T>) {
        let Some(info) = CacheInfo::get(Cache::D_L1) else {
            return;
        };

        let start = range.start().mask(!(info.linesize - 1));
        let end = unsafe {
            range
                .end()
                .mask(!(info.linesize - 1))
                .byte_add(info.linesize - 1)
        };

        dsb!("sy");

        let mut addr = start;
        loop {
            if addr >= end {
                break;
            }

            let mut op = op;
            if addr < *range.start() && op == CacheOp::Invalidate {
                op = CacheOp::CleanInvalidate;
            }

            let addr_end = unsafe { addr.byte_add(info.linesize - 1) };
            if addr_end > *range.end() && op == CacheOp::Invalidate {
                op = CacheOp::CleanInvalidate;
            }

            match op {
                CacheOp::Clean => unsafe { asm!("dc cvac, {:x}", in(reg) addr) },
                CacheOp::Invalidate => unsafe { asm!("dc ivac, {:x}", in(reg) addr) },
                CacheOp::CleanInvalidate => unsafe { asm!("dc civac, {:x}", in(reg) addr) },
            }

            addr = unsafe { addr.byte_add(info.linesize) };
        }

        isb!("sy")
    }
}
