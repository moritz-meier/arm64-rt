#![no_std]
#![feature(fn_align)]

use cfg_asm::cfg_naked_asm;

use core::{arch::naked_asm, usize};

pub use entry_macro::entry;

#[cfg(not(target_arch = "aarch64"))]
compile_error!("Only target_arch = \"aarch64\" is supported.");

#[cfg(not(target_os = "none"))]
compile_error!("Only target_os = \"none\" is supported.");

#[cfg(not(target_endian = "little"))]
compile_error!("Only target_endian = \"little\" is supported.");

pub struct EntryInfo {
    pub param: u64,
    pub current_el: usize,
    pub cpu_idx: usize,
    pub num_cores: usize,
}

pub trait Entry {
    unsafe extern "C" fn entry(info: EntryInfo) -> !;
}

#[macropol::macropol]
macro_rules! get_current_el {
    ($reg:literal) => {
        "mrs $reg, CurrentEL
         ubfm $reg, $reg, #0x2, #0x3"
    };
}

#[macropol::macropol]
macro_rules! get_num_cpu {
    ($reg:literal) => {
        "ldr $reg, =__NUM_CPU"
    };
}

#[macropol::macropol]
macro_rules! get_cpu_idx {
    ($reg:literal) => {
        "mrs $reg, MPIDR_EL1
         ubfm $reg, $reg, #0x0, 0x7"
    };
}

#[macropol::macropol]
macro_rules! init_el3 {
    (
        sctlr = $sctlr:literal,
        scr = $scr:literal,
        spsr = $spsr:literal,
        vbar = $vbar:literal
    ) => {
        "ldr x9, =$sctlr        // Set SCTLR_EL3
         msr SCTLR_EL3, x9

         ldr x9, =$scr          // Set SCR_EL3
         msr SCR_EL3, x9

         ldr x9, =$spsr         // Set SPSR_EL3 in case of eret inst is executed
         msr SPSR_EL3, x9

         ldr x9, =$vbar         // Set VBAR_EL3
         msr VBAR_EL3, x9

         msr CPTR_EL3, xzr      // Do not trap to EL3: accesses to CPACR, CPACR_EL1, HCPTR, CPTR_EL2, Advanced SIMD and floating-point functionality"
    };
}

#[macropol::macropol]
macro_rules! init_el2 {
    (
        sctlr = $sctlr:literal,
        hcr = $hcr:literal,
        spsr = $spsr:literal,
        vbar = $vbar:literal
    ) => {
        "ldr x9, =$sctlr        // Set SCTLR_EL2
         msr SCTLR_EL2, x9

         ldr x9, =$hcr          // Set HCR_EL2
         msr HCR_EL2, x9

         ldr x9, =$spsr         // Set SPSR_EL2 in case of eret inst is executed
         msr SPSR_EL2, x9

         mrs x9, MPIDR_EL1      // Set VMPIDR_EL2
         msr VMPIDR_EL2, x9

         ldr x9, =$vbar         // Set VBAR_EL2
         msr VBAR_EL2, x9

         msr CPTR_EL2, xzr      // Do not trap to EL2: accesses to CPACR, CPACR_EL1, Advanced SIMD and floating-point functionality"
    };
}

#[macropol::macropol]
macro_rules! init_el1 {
    (
        sctlr = $sctlr:literal,
        spsr = $spsr:literal,
        vbar = $vbar:literal,
        cpacr = $cpacr:literal
    ) => {
        "ldr x9, =$sctlr        // Set SCTLR_EL1
         msr SCTLR_EL1, x9

         ldr x9, =$spsr         // Set SPSR_EL1 in case of eret inst is executed
         msr SPSR_EL1, x9

         ldr x9, =$vbar         // Set VBAR_EL1
         msr VBAR_EL1, x9

         ldr x9, =$cpacr        // Set CPACR_EL1
         msr CPACR_EL1, x9 "
    };
}

#[macropol::macropol]
macro_rules! init_stack {
    (
        cpu_idx = $cpu_idx:literal,
        num_cpu = $num_cpu:literal
    ) => {
        "ldr x9, =__stack_start
         ldr x10, =__stack_end

         cmp x9, x10
         csel x9, x9, x10, lo           // if stack_start > stack_end, set stack_start = stack_end

         sub x11, x10, x9               // stack_size = stack_end - stack_start
         udiv x11, x11, $num_cpu        // stack_size = stack_size / num_cpu
         
         msub x10, x11, $cpu_idx, x10   // stack_end = stack_end - (stack_size * cpu_idx)
         sub x9, x10, x11               // stack_start = stack_end - stack_size

         ldr x12, =0xfefefefedeadc0de   // stack pattern

         1:                             // Zero stack
         cmp x9, x10
         b.hs 2f                        // done
         str x12, [x9], 0x8
         b 1b
         2:

         msr spsel, #0x1                // Use ELx stack
         mov sp, x10"
    };
}

#[macropol::macropol]
macro_rules! zero_bss {
    () => {
        "ldr x9, =__bss_start
         ldr x10, =__bss_end
         1:                         // Start loop
         cmp x9, x10
         b.hs 2f                    // done
         str xzr, [x9], 0x8
         b 1b
         2:                         // End loop"
    };
}
#[macropol::macropol]
macro_rules! init_registers {
    (
        param = $param:literal,
        current_el = $current_el:literal,
        cpu_idx = $cpu_idx:literal,
        num_cores = $num_cores:literal
    ) => {
        "mov x0, $param
         mov x1, $current_el
         mov x2, $cpu_idx
         mov x3, $num_cores
         mov x4, #0
         mov x5, #0
         mov x6, #0
         mov x7, #0

         mov x8, #0

         mov x9, #0
         mov x10, #0
         mov x11, #0
         mov x12, #0
         mov x13, #0
         mov x14, #0
         mov x15, #0

         mov x16, #0
         mov x17, #0

         mov x18, #0

         mov x19, #0
         mov x20, #0
         mov x21, #0
         mov x22, #0
         mov x23, #0
         mov x24, #0
         mov x25, #0
         mov x26, #0
         mov x27, #0
         mov x28, #0"
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

/*
    Register allocation:
    x0-x7:      param/return values
    x8/:        indirect return value register
    x9-x15:     corruptable registers, caller saved
    x16-x17:    intra-procedure-call scratch registers
    x18:        platform register
    x19-x28:    callee-saved
        x19:        param
        x20:        current el
        x21:        cpu idx
        x22:        num cores
*/

static mut SEC_CORE_LOCK: usize = 1;

#[unsafe(naked)]
pub unsafe extern "C" fn start<EntryImpl: Entry>() -> ! {
    #![allow(named_asm_labels)]
    cfg_naked_asm!({
        "mov x23, x0",                  // save param (fdt phys addr)

        get_current_el!("x20"),
        "cbz x20, hang",                // Hang if we are already in EL0

        get_cpu_idx!("x21"),            // Check cores
        get_num_cpu!("x22"),
        
        "cmp x21, x22",
        "b.hs hang",                    // Ignore cores with idx >= num_cpu

        "bl {core_init}",               // Init cores

        "cbz x21, rust_init",           // Primary core continue

        "sec_core_wait:",               // Secondary cores wait
        "ldr x9, ={sec_core_lock}",
        "sevl",
        "1:",
        "wfe",
        "ldr x10, [x9]",
        "cbnz x10, 1b",

        "rust_init:",
        "bl {rust_init}",               // Init Rust

        "sec_core_unlock:",             // Unlock secondary cores
        "ldr x9, ={sec_core_lock}",
        "1:",
        "ldxr x10, [x9]",
        "mov x10, #0",
        "stxr w11, x10, [x9]",
        "cbnz x11, 1b",
        "dmb sy",
        "sev",

        init_registers!(
            param = "x19",
            current_el = "x20",
            cpu_idx = "x21",
            num_cores = "x22"
        ),

        "bl {rust_entry}",               // Jump to Rust

        "hang:",
        hang!(),
    },
    sec_core_lock = sym SEC_CORE_LOCK,
    core_init = sym core_init,
    rust_init = sym rust_init,
    rust_entry = sym rust_entry::<EntryImpl>);
}

#[unsafe(naked)]
unsafe extern "C" fn core_init() -> ! {
    #![allow(named_asm_labels)]
    cfg_naked_asm!({
        "msr DAIFSet, 0xF",             // Mask all exceptions

        "cmp x20, #0x3",                // Check current el
        "b.eq in_el3",
        "cmp x20, #0x2",
        "b.eq in_el2",
        "cmp x20, #0x1",
        "b.eq in_el1",
        hang!(),                        // If CurrentEL != {EL3, EL2, EL1}, something is wrong

        "in_el3:",
        init_el3!(
            sctlr = "{sctlr_el3}",
            scr = "{scr_el3}",
            spsr = "{spsr_el3}",
            vbar = "{vectors}"),

        "in_el2:",
        init_el2!(
            sctlr = "{sctlr_el2}",
            hcr = "{hcr_el2}",
            spsr = "{spsr_el2}",
            vbar = "{vectors}"),

        "in_el1:",
        init_el1!(
            sctlr = "{sctlr_el1}",
            spsr = "{spsr_el1}",
            vbar = "{vectors}",
            cpacr = "{cpacr_el1}"),

        "ret",
    },

        sctlr_el3 = const SCTLR_EL3_INIT,
        sctlr_el2 = const SCTLR_EL2_INIT,
        sctlr_el1 = const SCTLR_EL1_INIT,

        scr_el3 = const SCR_EL3_INIT,
        hcr_el2 = const HCR_EL2_INIT,

        spsr_el3 = const SPSR_EL3_INIT,
        spsr_el2 = const SPSR_EL2_INIT,
        spsr_el1 = const SPSR_EL1_INIT,

        vectors = sym vectors,

        cpacr_el1 = const CPACR_EL1_INIT,
    )
}

#[unsafe(naked)]
unsafe extern "C" fn rust_init() -> ! {
    #![allow(named_asm_labels)]
    cfg_naked_asm!({
        "init_stack:",
        init_stack!(cpu_idx = "x21", num_cpu = "x22"),

        "cbnz x21, 1f",                 // Secondary cores skip
        "zero_bss:",
        zero_bss!(),
        "1:",

        "ret",
    },)
}

unsafe extern "C" fn rust_entry<EntryImpl: Entry>(
    param: u64,
    current_el: u64,
    cpu_idx: u64,
    num_cores: u64,
) -> ! {
    unsafe {
        EntryImpl::entry(EntryInfo {
            param,
            current_el: current_el as usize,
            cpu_idx: cpu_idx as usize,
            num_cores: num_cores as usize,
        })
    }
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

const SCTLR_EL3_INIT: u64 = SCTLR_EL3_RES1; // Reserved 1 Bits must be set
const SCTLR_EL3_RES1: u64 = (1 << 29)
    | (1 << 28)
    | (1 << 23)
    | (1 << 22)
    | (1 << 18)
    | (1 << 16)
    | (1 << 11)
    | (1 << 5)
    | (1 << 4);

const SCTLR_EL2_INIT: u64 = SCTLR_EL2_RES1; // Reserved 1 Bits must be set
const SCTLR_EL2_RES1: u64 = (1 << 29)
    | (1 << 28)
    | (1 << 23)
    | (1 << 22)
    | (1 << 18)
    | (1 << 16)
    | (1 << 11)
    | (1 << 5)
    | (1 << 4);

const SCTLR_EL1_INIT: u64 = SCTLR_EL1_RES1; // Reserved 1 Bits must be set
const SCTLR_EL1_RES1: u64 = (1 << 29) | (1 << 28) | (1 << 23) | (1 << 22) | (1 << 20) | (1 << 11);

const SCR_EL3_RES1: u64 = (1 << 5) | (1 << 4);
const SCR_EL3_NS_BIT: usize = 0;
const SCR_EL3_IRQ_BIT: usize = 1;
const SCR_EL3_FIQ_BIT: usize = 2;
const SCR_EL3_EA_BIT: usize = 3;
const SCR_EL3_INIT: u64 = SCR_EL3_RES1                  // Reserved 1 Bits must be set
    | (1 << SCR_EL3_EA_BIT)                             // Take SError, FIQ, IRQ to EL3
    | (1 << SCR_EL3_FIQ_BIT)
    | (1 << SCR_EL3_IRQ_BIT)
    | (1 << SCR_EL3_NS_BIT); // Lower ELs are non-secure

const HCR_EL2_RES1: u64 = 0;
const HCR_EL2_FMO_BIT: usize = 3;
const HCR_EL2_IMO_BIT: usize = 4;
const HCR_EL2_AMO_BIT: usize = 5;
const HCR_EL2_INIT: u64 = HCR_EL2_RES1                  // Reserved 1 Bits must be set
    | (1 << HCR_EL2_AMO_BIT)                            // Take SError, IRQ, FIQ to EL2
    | (1 << HCR_EL2_IMO_BIT)
    | (1 << HCR_EL2_FMO_BIT);

const SPSR_EL3_M_BIT: usize = 0;
const SPSR_EL3_DAIF_BIT: usize = 6;
const SPSR_EL3_INIT: u64 = (0xF << SPSR_EL3_DAIF_BIT)   // Mask interrupts on eret
    | (0b01101 << SPSR_EL3_M_BIT); // Return to AArch64 EL3 and SP_EL3 on eret

const SPSR_EL2_M_BIT: usize = 0;
const SPSR_EL2_DAIF_BIT: usize = 6;
const SPSR_EL2_INIT: u64 = (0xF << SPSR_EL2_DAIF_BIT)   // Mask interrupts on eret
    | (0b01001 << SPSR_EL2_M_BIT); // Return to AArch64 EL2 and SP_EL2 on eret

const SPSR_EL1_M_BIT: usize = 0;
const SPSR_EL1_DAIF_BIT: usize = 6;
const SPSR_EL1_INIT: u64 = (0xF << SPSR_EL1_DAIF_BIT)   // Mask interrupts on eret
    | (0b00101 << SPSR_EL1_M_BIT); // Return to AArch64 EL1 and SP_EL1 on eret

const CPACR_EL1_FPEN_BIT: usize = 20;
const CPACR_EL1_INIT: u64 = 0b11 << CPACR_EL1_FPEN_BIT; // Enable FP and SIMD
