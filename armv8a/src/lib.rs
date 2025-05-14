#![no_std]

use cfg_asm::cfg_naked_asm;

use core::arch::naked_asm;

pub use entry_macro::entry;

#[cfg(not(target_arch = "aarch64"))]
compile_error!("Only target_arch = \"aarch64\" is supported.");

pub trait Entry {
    unsafe extern "C" fn entry() -> !;
}

#[macropol::macropol]
macro_rules! get_cpu_id {
    ($reg:literal) => {
        "mrs $reg, MPIDR_EL1
         ubfm $reg, $reg, #0, #7"
    };
}

macro_rules! reset_el3 {
    () => {
        "mrs x9, SCTLR_EL3
         bic x9, x9, #(1 << 0)
         bic x9, x9, #(1 << 2)
         bic x9, x9, #(1 << 12)
         msr SCTLR_EL3, x9
         dsb sy
         isb
         
         tlbi alle3
         ic iallu
         dsb sy
         isb
         "
    };
}

#[macropol::macropol]
macro_rules! inf_loop {
    () => {
        "1:
         wfi
         b 1b"
    };
}

#[unsafe(naked)]
pub unsafe extern "C" fn start<EntryImpl: Entry>() -> ! {
    #![allow(named_asm_labels)]
    cfg_naked_asm!({
        "msr DAIFSet, 0xf",         // Disable all Interrupts

        get_cpu_id!("x9"),          // Get CPU core id
        "cmp x9, #0",
        "beq pri_core",

        inf_loop!(),                // Other cores wait

        "pri_core:",                // Primary core
        "mrs x0, CurrentEL",        // Get Current EL
        "ubfm x0, x0, #2, #(2 + 2 - 1)",

        "cmp x0, #3",               // Jump to specific handler
        "beq in_el3",
        "cmp x0, #2",
        "beq in_el2",
        "cmp x0, #1",
        "beq in_el1",
        "blt in_el0",

        inf_loop!(),                // If current EL != {0, 1, 2, 3}, something is really wrong

        "in_el3:",                  // If we are in EL3
        reset_el3!(),

        "in_el2:",
        "",

        "in_el1:",
        "",

        "in_el0:",
        "",
    },);
}
