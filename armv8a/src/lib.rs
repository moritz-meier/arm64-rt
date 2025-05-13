#![no_std]

use cfg_asm::cfg_naked_asm;

use core::arch::naked_asm;

pub use entry_macro::entry;

#[cfg(not(target_arch = "aarch64"))]
compile_error!("Only target_arch = \"aarch64\" is supported.");

pub trait Entry {
    unsafe extern "C" fn entry() -> !;
}

#[unsafe(naked)]
pub unsafe extern "C" fn start<EntryImpl: Entry>() -> ! {
    cfg_naked_asm!({
            #[cfg(target_arch = "arm")]
            "nop {x}",
            #[cfg(target_arch = "aarch64")]
            "sdffh {x}",
    }, x = const 634)
}
