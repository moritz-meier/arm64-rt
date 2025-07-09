use core::{arch::asm, ops::RangeInclusive};

use crate::start;

use super::*;

pub struct ICache;

impl ICache {
    pub fn enable() {
        Self::invalidate_all();

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
                "orr x10, x10, #{sctlr_el3_i_mask}",
                "msr SCTLR_EL3, x10",
                "12:",
                "mrs x10, SCTLR_EL2",
                "orr x10, x10, #{sctlr_el2_i_mask}",
                "msr SCTLR_EL2, x10",
                "11:",
                "mrs x10, SCTLR_EL1",
                "orr x10, x10, #{sctlr_el1_i_mask}",
                "msr SCTLR_EL1, x10",
                "10:",
                "isb sy",

                sctlr_el3_i_mask = const SCTLR_EL3_I_MASK,
                sctlr_el2_i_mask = const SCTLR_EL2_I_MASK,
                sctlr_el1_i_mask = const SCTLR_EL1_I_MASK
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
                "bic x10, x10, #{sctlr_el3_i_mask}",
                "msr SCTLR_EL3, x10",
                "12:",
                "mrs x10, SCTLR_EL2",
                "bic x10, x10, #{sctlr_el2_i_mask}",
                "msr SCTLR_EL2, x10",
                "11:",
                "mrs x10, SCTLR_EL1",
                "bic x10, x10, #{sctlr_el1_i_mask}",
                "msr SCTLR_EL1, x10",
                "10:",
                "isb sy",

                sctlr_el3_i_mask = const SCTLR_EL3_I_MASK,
                sctlr_el2_i_mask = const SCTLR_EL2_I_MASK,
                sctlr_el1_i_mask = const SCTLR_EL1_I_MASK
            )
        }
    }

    pub fn invalidate_all() {
        unsafe { asm!("dsb sy", "ic iallu", "isb sy") }
    }

    pub fn invalidate<T>(range: RangeInclusive<*const T>) {
        let Some(info) = CacheInfo::get(Cache::I_L1) else {
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

            unsafe {
                asm!("ic ivau, {addr}", addr = in(reg) addr);
            }

            addr = unsafe { addr.byte_add(info.linesize) };
        }

        unsafe {
            asm!("isb sy");
        }
    }
}

const SCTLR_EL3_I_MASK: u64 = bitmask!(bit: 12);
const SCTLR_EL2_I_MASK: u64 = bitmask!(bit: 12);
const SCTLR_EL1_I_MASK: u64 = bitmask!(bit: 12);
