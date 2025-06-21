#![no_std]
#![feature(fn_align)]

use cfg_asm::cfg_naked_asm;

use core::{arch::naked_asm, usize};

pub use entry_macro::entry;

#[cfg(not(target_arch = "aarch64"))]
compile_error!("Only target_arch = \"aarch64\" is supported.");

pub trait Entry {
    unsafe extern "C" fn entry() -> !;
}

#[macropol::macropol]
macro_rules! get_current_el {
    ($reg:literal) => {
        "mrs $reg, CurrentEL
         ubfm $reg, $reg, #0x2, #0x3"
    };
}

#[macropol::macropol]
macro_rules! get_cpu_id {
    ($reg:literal) => {
        "mrs $reg, MPIDR_EL1
         ubfm $reg, $reg, #0x0, 0x7"
    };
}

#[macropol::macropol]
macro_rules! init_el3 {
    (
        $reg:literal,
        sctlr = $sctlr:literal,
        scr = $scr:literal,
        spsr = $spsr:literal,
        vbar = $vbar:literal
    ) => {
        "ldr $reg, =$sctlr      // Set SCTLR_EL3
         msr SCTLR_EL3, $reg

         ldr $reg, =$scr        // Set SCR_EL3
         msr SCR_EL3, $reg

         ldr $reg, =$spsr       // Set SPSR_EL3 in case of eret inst is executed
         msr SPSR_EL3, $reg

         ldr $reg, =$vbar       // Set VBAR_EL3
         msr VBAR_EL3, $reg

         msr CPTR_EL3, xzr      // Do not trap FP / SIMD instructions"
    };
}

#[macropol::macropol]
macro_rules! init_el2 {
    (
        $reg:literal,
        sctlr = $sctlr:literal,
        hcr = $hcr:literal,
        spsr = $spsr:literal,
        vbar = $vbar:literal
    ) => {
        "ldr $reg, =$sctlr      // Set SCTLR_EL2
         msr SCTLR_EL2, $reg

         ldr $reg, =$hcr        // Set HCR_EL2
         msr HCR_EL2, $reg

         ldr $reg, =$spsr       // Set SPSR_EL2 in case of eret inst is executed
         msr SPSR_EL2, $reg

         mrs $reg, MPIDR_EL1    // Set VMPIDR_EL2
         msr VMPIDR_EL2, $reg

         ldr $reg, =$vbar       // Set VBAR_EL2
         msr VBAR_EL2, $reg

         msr CPTR_EL2, xzr      // Do not trap FP / SIMD instructions"
    };
}

#[macropol::macropol]
macro_rules! init_el1 {
    (
        $reg:literal,
        sctlr = $sctlr:literal,
        spsr = $spsr:literal,
        vbar = $vbar:literal,
        cpacr = $cpacr:literal
    ) => {
        "ldr $reg, =$sctlr      // Set SCTLR_EL1
         msr SCTLR_EL1, $reg

         ldr $reg, =$spsr       // Set SPSR_EL1 in case of eret inst is executed
         msr SPSR_EL1, $reg

         ldr $reg, =$vbar       // Set VBAR_EL1
         msr VBAR_EL1, $reg

         ldr $reg, =$cpacr      // Set CPACR_EL1
         msr CPACR_EL1, $reg"
    };
}

#[macropol::macropol]
macro_rules! init_stack {
    (
        $reg0:literal,
        $reg1:literal,
        stack_start = $stack_start:literal,
        stack_end = $stack_end:literal
    ) => {
        "ldr $reg0, =$stack_start   // Init stack with pattern
         ldr $reg1, =$stack_end
         1:                         // Start loop
         cmp $reg0, $reg1
         b.ge 2f                    // done
         str xzr, [$reg0], 0x8
         b 1b
         2:                         // End loop

         mov sp, x10                // Set SP"
    };
}

#[macropol::macropol]
macro_rules! zero_bss {
    (
        $reg0:literal,
        $reg1:literal,
        bss_start = $bss_start:literal,
        bss_end = $bss_end:literal
    ) => {
        "ldr $reg0, =$bss_start     // Init stack with pattern
         ldr $reg1, =$bss_end
         1:                         // Start loop
         cmp $reg0, $reg1
         b.ge 2f                    // done
         str xzr, [$reg0], 0x8
         b 1b
         2:                         // End loop"
    };
}

#[macropol::macropol]
macro_rules! hang {
    () => {
        "1:
         wfe
         b 1b"
    };
}

#[unsafe(naked)]
pub unsafe extern "C" fn start<EntryImpl: Entry>() -> ! {
    #![allow(named_asm_labels)]
    cfg_naked_asm!({
        get_current_el!("x9"),
        "cbz x9, hang",                 // Hang if we are already in EL0

        get_cpu_id!("x9"),
        "cbz x9, {pri_core_init}",      // Core #0 continue

        "sec_core_wait:",               // Other cores wait
        "wfe",
        "b sec_core_wait",

        "hang:",
        hang!(),
    },
    pri_core_init = sym primary_core_init::<EntryImpl>);
}

#[unsafe(naked)]
unsafe extern "C" fn primary_core_init<EntryImpl: Entry>() -> ! {
    #![allow(named_asm_labels)]
    cfg_naked_asm!({
        "msr DAIFSet, 0xF",             // Mask all exceptions

        get_current_el!("x9"),
        "cmp x9, #0x3",
        "b.eq in_el3",
        "cmp x9, #0x2",
        "b.eq in_el2",
        "cmp x9, #0x1",
        "b.eq in_el1",
        hang!(),                        // If CurrentEL != {EL3, EL2, EL1}, something is wrong

        "in_el3:",
        init_el3!("x9",
            sctlr = "{sctlr_el3}",
            scr = "{scr_el3}",
            spsr = "{spsr_el3}",
            vbar = "{vectors}"),

        "in_el2:",
        init_el2!("x9",
            sctlr = "{sctlr_el2}",
            hcr = "{hcr_el2}",
            spsr = "{spsr_el2}",
            vbar = "{vectors}"),

        "in_el1:",
        init_el1!("x9",
            sctlr = "{sctlr_el1}",
            spsr = "{spsr_el1}",
            vbar = "{vectors}",
            cpacr = "{cpacr_el1}"),

        "init_stack:",
        init_stack!("x9", "x10",
            stack_start = "__stack_start",
            stack_end = "__stack_end"),

        "zero_bss:",
        zero_bss!("x9", "x10",
            bss_start = "__bss_start",
            bss_end = "__bss_end"),

        "b {entry}",
    },
        sctlr_el3 = const SCTLR_EL3_RES1,               // Reserved 1 Bits must be set
        sctlr_el2 = const SCTLR_EL2_RES1,
        sctlr_el1 = const SCTLR_EL1_RES1,

        scr_el3 = const SCR_EL3_RES1 |                  // Reserved 1 Bits must be set
            (1 << SCR_EL3_EA_BIT) |                     // Take SError, FIQ, IRQ to EL3
            (1 << SCR_EL3_FIQ_BIT) |
            (1 << SCR_EL3_IRQ_BIT) |
            (1 << SCR_EL3_NS_BIT),                      // Lower ELs are non-secure

        hcr_el2 = const HCR_EL2_RES1 |                  // Reserved 1 Bits must be set
            (1 << HCR_EL2_AMO_BIT) |                    // Take SError, IRQ, FIQ to EL2
            (1 << HCR_EL2_IMO_BIT) |
            (1 << HCR_EL2_FMO_BIT),

        spsr_el3 = const (0xF << SPSR_EL3_DAIF_BIT) |   // Mask interrupts on eret
            (0b01101 << SPSR_EL3_M_BIT),                // Return to AArch64 EL3 and SP_EL3 on eret

        spsr_el2 = const (0xF << SPSR_EL2_DAIF_BIT) |   // Mask interrupts on eret
            (0b01001 << SPSR_EL2_M_BIT),                // Return to AArch64 EL2 and SP_EL2 on eret

        spsr_el1 = const (0xF << SPSR_EL1_DAIF_BIT) |
            (0b00101 << SPSR_EL1_M_BIT),                // Return to AArch64 EL1 and SP_EL1 on eret

        cpacr_el1 = const (0x3 << CPACR_EL1_FPEN_BIT),  // Do not trap FP & SIMD instructions at EL1/EL0

        vectors = sym vectors,

        entry = sym EntryImpl::entry,
    )
}

#[unsafe(naked)]
#[repr(align(2048))]
unsafe extern "C" fn vectors() {
    cfg_naked_asm!({
        "b {excp_handler}",

        ".balign 0x80",
        "b {excp_handler}",

        ".balign 0x80",
        "b {excp_handler}",

        ".balign 0x80",
        "b {excp_handler}",

        ".balign 0x80",
        "b {excp_handler}",

        ".balign 0x80",
        "b {excp_handler}",

        ".balign 0x80",
        "b {excp_handler}",

        ".balign 0x80",
        "b {excp_handler}",

        ".balign 0x80",
        "b {excp_handler}",

        ".balign 0x80",
        "b {excp_handler}",

        ".balign 0x80",
        "b {excp_handler}",

        ".balign 0x80",
        "b {excp_handler}",

        ".balign 0x80",
        "b {excp_handler}",

        ".balign 0x80",
        "b {excp_handler}",

        ".balign 0x80",
        "b {excp_handler}",

        ".balign 0x80",
        "b {excp_handler}",
    },
        excp_handler = sym exception_handler)
}

#[unsafe(naked)]
unsafe extern "C" fn exception_handler() -> ! {
    cfg_naked_asm!({ hang!(), },)
}

const SCTLR_EL3_RES1: u64 = (1 << 29)
    | (1 << 28)
    | (1 << 23)
    | (1 << 22)
    | (1 << 18)
    | (1 << 16)
    | (1 << 11)
    | (1 << 5)
    | (1 << 4);

const SCTLR_EL2_RES1: u64 = (1 << 29)
    | (1 << 28)
    | (1 << 23)
    | (1 << 22)
    | (1 << 18)
    | (1 << 16)
    | (1 << 11)
    | (1 << 5)
    | (1 << 4);

const SCTLR_EL1_RES1: u64 = (1 << 29) | (1 << 28) | (1 << 23) | (1 << 22) | (1 << 20) | (1 << 11);

const SCR_EL3_RES1: u64 = (1 << 5) | (1 << 4);
const SCR_EL3_NS_BIT: usize = 0;
const SCR_EL3_IRQ_BIT: usize = 1;
const SCR_EL3_FIQ_BIT: usize = 2;
const SCR_EL3_EA_BIT: usize = 3;

const HCR_EL2_RES1: u64 = 0;
const HCR_EL2_FMO_BIT: usize = 3;
const HCR_EL2_IMO_BIT: usize = 4;
const HCR_EL2_AMO_BIT: usize = 5;

const SPSR_EL3_M_BIT: usize = 0;
const SPSR_EL3_DAIF_BIT: usize = 6;

const SPSR_EL2_M_BIT: usize = 0;
const SPSR_EL2_DAIF_BIT: usize = 6;

const SPSR_EL1_M_BIT: usize = 0;
const SPSR_EL1_DAIF_BIT: usize = 6;

const CPACR_EL1_FPEN_BIT: usize = 20;
