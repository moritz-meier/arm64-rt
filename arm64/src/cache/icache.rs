use core::{arch::asm, ops::RangeInclusive};

use crate::{dsb, isb};

use super::*;

pub struct ICache;

impl ICache {
    pub fn enable() {
        Self::invalidate_all();

        let current_el = CURRENT_EL.read().EL().value();

        dsb!("sy");

        match current_el {
            3 => SCTLR_EL3.modify(|sctlr_el3| sctlr_el3.with_I(true)),
            2 => SCTLR_EL2.modify(|sctlr_el2| sctlr_el2.with_I(true)),
            1 => SCTLR_EL1.modify(|sctlr_el1| sctlr_el1.with_I(true)),
            _ => (),
        }

        isb!("sy");
    }

    pub fn disable() {
        let current_el = CURRENT_EL.read().EL().value();

        dsb!("sy");

        match current_el {
            3 => SCTLR_EL3.modify(|sctlr_el3| sctlr_el3.with_I(false)),
            2 => SCTLR_EL2.modify(|sctlr_el2| sctlr_el2.with_I(false)),
            1 => SCTLR_EL1.modify(|sctlr_el1| sctlr_el1.with_I(false)),
            _ => (),
        }

        isb!("sy");
    }

    pub fn invalidate_all() {
        dsb!("sy");

        unsafe { asm!("ic iallu") }

        isb!("sy");
    }

    pub fn invalidate<T>(range: RangeInclusive<*const T>) {
        let Some(info) = CacheInfo::get(Cache::I_L1) else {
            return;
        };

        let start = range.start().mask(!(info.linesize as usize - 1));
        let end = unsafe {
            range
                .end()
                .mask(!(info.linesize as usize - 1))
                .byte_add(info.linesize as usize - 1)
        };

        dsb!("sy");

        let mut addr = start;
        loop {
            if addr >= end {
                break;
            }

            unsafe {
                asm!("ic ivau, {:x}", in(reg) addr);
            }

            addr = unsafe { addr.byte_add(info.linesize as usize) };
        }

        isb!("sy")
    }
}
