use core::{arch::asm, ops::RangeInclusive};

use crate::{dsb, isb, sysreg_read_bitfield, sysreg_write_bitfield};

use super::*;

pub struct ICache;

impl ICache {
    pub fn enable() {
        Self::invalidate_all();

        let current_el: usize = sysreg_read_bitfield!("CurrentEL", msb: 3, lsb: 2);

        dsb!("sy");

        if current_el <= 3 {
            sysreg_write_bitfield!("SCTLR_EL3", bit: 12, value: 0b1);
        }

        if current_el <= 2 {
            sysreg_write_bitfield!("SCTLR_EL2", bit: 12, value: 0b1);
        }

        if current_el == 1 {
            sysreg_write_bitfield!("SCTLR_EL1", bit: 12, value: 0b1);
        }

        isb!("sy");
    }

    pub fn disable() {
        let current_el: usize = sysreg_read_bitfield!("CurrentEL", msb: 3, lsb: 2);

        dsb!("sy");

        if current_el <= 3 {
            sysreg_write_bitfield!("SCTLR_EL3", bit: 12, value: 0b0);
        }

        if current_el <= 2 {
            sysreg_write_bitfield!("SCTLR_EL2", bit: 12, value: 0b0);
        }

        if current_el == 1 {
            sysreg_write_bitfield!("SCTLR_EL1", bit: 12, value: 0b0);
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

            unsafe {
                asm!("ic ivau, {:x}", in(reg) addr);
            }

            addr = unsafe { addr.byte_add(info.linesize) };
        }

        isb!("sy")
    }
}
