use core::arch::asm;

use crate::bitmask;

use super::*;
pub struct DCache;

impl DCache {
    pub fn enable() {
        Self::op_all(CacheOp::Invalidate);

        unsafe {
            asm!(
                "mrs x9, CurrentEL",
                "ubfm x9, x9, #2, #3",
                "dsb sy",
                "cmp x9, #0x3",
                "b.eq 13f",
                "cmp x9, #0x2",
                "b.eq 12f",
                "cmp x9, #0x1",
                "b.eq 11f",
                "b 10f",
                "13:",
                "mrs x10, SCTLR_EL3",
                "orr x10, x10, #{sctlr_el3_c_mask}",
                "msr SCTLR_EL3, x10",
                "12:",
                "mrs x10, SCTLR_EL2",
                "orr x10, x10, #{sctlr_el2_c_mask}",
                "msr SCTLR_EL2, x10",
                "11:",
                "mrs x10, SCTLR_EL1",
                "orr x10, x10, #{sctlr_el1_c_mask}",
                "msr SCTLR_EL1, x10",
                "10:",
                "isb sy",

                sctlr_el3_c_mask = const SCTLR_EL3_C_MASK,
                sctlr_el2_c_mask = const SCTLR_EL2_C_MASK,
                sctlr_el1_c_mask = const SCTLR_EL1_C_MASK
            )
        }
    }

    pub fn disable() {
        unsafe {
            asm!(
                "mrs x9, CurrentEL",
                "ubfm x9, x9, #2, #3",
                "dsb sy",
                "cmp x9, #0x3",
                "b.eq 13f",
                "cmp x9, #0x2",
                "b.eq 12f",
                "cmp x9, #0x1",
                "b.eq 11f",
                "b 10f",
                "13:",
                "mrs x10, SCTLR_EL3",
                "bic x10, x10, #{sctlr_el3_c_mask}",
                "msr SCTLR_EL3, x10",
                "12:",
                "mrs x10, SCTLR_EL2",
                "bic x10, x10, #{sctlr_el2_c_mask}",
                "msr SCTLR_EL2, x10",
                "11:",
                "mrs x10, SCTLR_EL1",
                "bic x10, x10, #{sctlr_el1_c_mask}",
                "msr SCTLR_EL1, x10",
                "10:",
                "isb sy",

                sctlr_el3_c_mask = const SCTLR_EL3_C_MASK,
                sctlr_el2_c_mask = const SCTLR_EL2_C_MASK,
                sctlr_el1_c_mask = const SCTLR_EL1_C_MASK
            )
        }
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

            unsafe {
                asm!("dsb sy");
            }

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

        unsafe { asm!("isb sy") }
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

        unsafe {
            asm!("dsb sy");
        }

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
                CacheOp::Clean => unsafe { asm!("dc cvac, {addr}", addr = in(reg) addr) },
                CacheOp::Invalidate => unsafe { asm!("dc ivac, {addr}", addr = in(reg) addr) },
                CacheOp::CleanInvalidate => unsafe {
                    asm!("dc civac, {addr}", addr = in(reg) addr)
                },
            }

            addr = unsafe { addr.byte_add(info.linesize) };
        }

        unsafe { asm!("isb sy") }
    }
}

const SCTLR_EL3_C_MASK: u64 = bitmask!(bit: 2);
const SCTLR_EL2_C_MASK: u64 = bitmask!(bit: 2);
const SCTLR_EL1_C_MASK: u64 = bitmask!(bit: 2);
