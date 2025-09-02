use core::{arch::naked_asm, usize};

use cfg_asm::cfg_naked_asm;

pub use entry_macro::entry;

use crate::{exceptions::*, sys_regs::*};

pub trait Entry {
    unsafe extern "C" fn entry(info: EntryInfo) -> !;
}

#[repr(C)]
pub struct EntryInfo {
    pub param: u64,
    pub current_el: usize,
    pub cpu_idx: usize,
    pub num_cores: usize,
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
pub unsafe extern "C" fn start<EntryImpl: Entry, ExcpVecs: ExceptionVectors>() -> ! {
    cfg_naked_asm!({
        "mov x23, x0",                  // save param (fdt phys addr)

        "mrs x20, CurrentEL",           // Get CurrentEL
        "ubfm x20, x20, #0x2, #0x3",
        "cbz x20, 100f",                // Hang if we are already in EL0

        "mrs x21, MPIDR_EL1",           // Get cpu idx
        "ubfm x21, x21, #0x0, 0x7",

        "ldr x22, =__NUM_CPU",          // Get core count
        
        "cmp x21, x22",
        "b.hs 100f",                    // Ignore cores with idx >= num_cpu

        "bl {core_init}",               // Init cores

        #[cfg(feature = "cortex-a53")]  // Init impl specific stuff
        "bl {core_a53_init}",

        "cbz x21, 10f",                 // Primary core continue with Rust init
              
        "ldr x9, ={sec_core_lock}",     // Secondary cores wait
        "sevl",
        "2:",
        "wfe",
        "ldr x10, [x9]",
        "cbnz x10, 2b",

        "10:",
        "bl {rust_init}",               // Init Rust
            
        "ldr x9, ={sec_core_lock}",     // Unlock secondary cores
        "2:",
        "ldxr x10, [x9]",
        "mov x10, #0",
        "stxr w11, x10, [x9]",
        "cbnz x11, 2b",
        "dmb sy",
        "sev",

        "mov x0, x19",                  // Zero registers
        "mov x1, x20",
        "mov x2, x21",
        "mov x3, x22",
        "mov x4, #0",
        "mov x5, #0",
        "mov x6, #0",
        "mov x7, #0",

        "mov x8, #0",

        "mov x9, #0",
        "mov x10, #0",
        "mov x11, #0",
        "mov x12, #0",
        "mov x13, #0",
        "mov x14, #0",
        "mov x15, #0",

        "mov x16, #0",
        "mov x17, #0",

        "mov x18, #0",

        "mov x19, #0",
        "mov x20, #0",
        "mov x21, #0",
        "mov x22, #0",
        "mov x23, #0",
        "mov x24, #0",
        "mov x25, #0",
        "mov x26, #0",
        "mov x27, #0",
        "mov x28, #0",

        "bl {rust_entry}",              // Jump to Rust

        "100:",                         // Hang
        "wfe",
        "b 100b",
    },
    sec_core_lock = sym SEC_CORE_LOCK,
    core_init = sym core_init::<ExcpVecs>,
    core_a53_init = sym core_a53_init,
    rust_init = sym rust_init,
    rust_entry = sym rust_entry::<EntryImpl>);
}

#[unsafe(naked)]
unsafe extern "C" fn core_init<ExcpVecs: ExceptionVectors>() {
    cfg_naked_asm!({
        "msr DAIFSet, 0xF",             // Mask all exceptions

        "cmp x20, #0x3",                // Check current EL
        "b.eq 13f",
        "cmp x20, #0x2",
        "b.eq 12f",
        "cmp x20, #0x1",
        "b.eq 11f",
        "2:",                           // If CurrentEL != {EL3, EL2, EL1}, something is wrong
        "wfe",
        "b 2b",

        // Init EL3
        "13:",
        "ldr x9, ={sctlr_el3}",         // Set SCTLR_EL3
        "msr SCTLR_EL3, x9",

        "ldr x9, ={scr_el3}",           // Set SCR_EL3
        "msr SCR_EL3, x9",

        "ldr x9, ={spsr_el3}",          // Set SPSR_EL3 in case of eret inst is executed
        "msr SPSR_EL3, x9",

        "ldr x9, ={vectors}",           // Set VBAR_EL3
        "msr VBAR_EL3, x9",

        "msr CPTR_EL3, xzr",            // Do not trap to EL3: accesses to CPACR, CPACR_EL1, HCPTR, CPTR_EL2, Advanced SIMD and floating-point functionality",

        // Init EL2
        "12:",
        "ldr x9, ={sctlr_el2}",         // Set SCTLR_EL2
        "msr SCTLR_EL2, x9",

        "ldr x9, ={hcr_el2}",           // Set HCR_EL2
        "msr HCR_EL2, x9",

        "ldr x9, ={spsr_el2}",          // Set SPSR_EL2 in case of eret inst is executed
        "msr SPSR_EL2, x9",

        "mrs x9, MPIDR_EL1",            // Set VMPIDR_EL2
        "msr VMPIDR_EL2, x9",

        "ldr x9, ={vectors}",           // Set VBAR_EL2
        "msr VBAR_EL2, x9",

        "msr CPTR_EL2, xzr",            // Do not trap to EL2: accesses to CPACR, CPACR_EL1, Advanced SIMD and floating-point functionality"

        // Init EL1
        "11:",
        "ldr x9, ={sctlr_el1}",         // Set SCTLR_EL1
        "msr SCTLR_EL1, x9",

        "ldr x9, ={spsr_el1}",          // Set SPSR_EL1 in case of eret inst is executed
        "msr SPSR_EL1, x9",

        "ldr x9, ={vectors}",           // Set VBAR_EL1
        "msr VBAR_EL1, x9",

        "msr CPACR_EL1, xzr",           // Trap SIMD, FPU

        "ret",
    },

        sctlr_el3 = const SCTLR_EL3_INIT.raw_value(),
        sctlr_el2 = const SCTLR_EL2_INIT.raw_value(),
        sctlr_el1 = const SCTLR_EL1_INIT.raw_value(),

        scr_el3 = const SCR_EL3_INIT.raw_value(),
        hcr_el2 = const HCR_EL2_INIT.raw_value(),

        spsr_el3 = const SPSR_EL3_INIT.raw_value(),
        spsr_el2 = const SPSR_EL2_INIT.raw_value(),
        spsr_el1 = const SPSR_EL1_INIT.raw_value(),

        vectors = sym vector_table::<ExcpVecs>,
    )
}

#[unsafe(naked)]
unsafe extern "C" fn core_a53_init() {
    cfg_naked_asm!({
        "cmp x20, #0x3",                // Check current EL
        "b.eq 13f",
        "cmp x20, #0x2",
        "b.eq 12f",
        "cmp x20, #0x1",
        "b.eq 11f",
        "b 10f",

        "13:",                          // In EL3
        "mrs x9, S3_1_C15_C2_1",        // Set SMPEN in CPUECTLR_EL1
        "orr x9, x9, #0x40",
        "msr S3_1_C15_C2_1, x9",

        "12:",                          // In EL2
        "11:",                          // In EL1

        "10:",                          // Other

        "ret",
    },)
}

#[unsafe(naked)]
unsafe extern "C" fn rust_init() {
    cfg_naked_asm!({
        // Init stack
        "ldr x9, =__stack_start",
        "ldr x10, =__stack_end",

        "cmp x9, x10",
        "csel x9, x9, x10, lo",         // if stack_start > stack_end, set stack_start = stack_end

        "sub x11, x10, x9",             // stack_size = stack_end - stack_start
        "udiv x11, x11, x22",           // stack_size = stack_size / num_cpu

        "msub x10, x11, x21, x10",      // stack_end = stack_end - (stack_size * cpu_idx)
        "sub x9, x10, x11",             // stack_start = stack_end - stack_size

        "ldr x12, =0xfefefefedeadc0de", // stack pattern

        "2:",                           // loop
        "cmp x9, x10",
        "b.hs 3f",                      // done
        "str x12, [x9], 0x8",
        "b 2b",
        "3:",                           // end

        "msr spsel, #0x1",              // Use ELx stack
        "mov sp, x10",

        "cbnz x21, 3f",                 // Secondary cores skip

        // Zero bss
        "ldr x9, =__bss_start",
        "ldr x10, =__bss_end",
        "2:",                           // Start loop
        "cmp x9, x10",
        "b.hs 3f",                      // done
        "str xzr, [x9], 0x8",
        "b 2b",
        "3:",                           // end

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

const SCTLR_EL3_INIT: SCTLR_EL3 = SCTLR_EL3::DEFAULT;
const SCTLR_EL2_INIT: SCTLR_EL2 = SCTLR_EL2::DEFAULT;
const SCTLR_EL1_INIT: SCTLR_EL1 = SCTLR_EL1::DEFAULT;

const SCR_EL3_INIT: SCR_EL3 = SCR_EL3::DEFAULT
    .with_EA(true)
    .with_FIQ(true)
    .with_IRQ(true)
    .with_NS(true);

const HCR_EL2_INIT: HCR_EL2 = HCR_EL2::DEFAULT
    .with_AMO(true)
    .with_IMO(true)
    .with_FMO(true);

const SPSR_EL3_INIT: SPSR_EL3 = SPSR_EL3::DEFAULT
    .with_D(true)
    .with_A(true)
    .with_I(true)
    .with_F(true)
    .with_M(spsr_el3::M::AARCH64_EL3_SP_EL3);

const SPSR_EL2_INIT: SPSR_EL2 = SPSR_EL2::DEFAULT
    .with_D(true)
    .with_A(true)
    .with_I(true)
    .with_F(true)
    .with_M(spsr_el2::M::AARCH64_EL2_SP_EL2);

const SPSR_EL1_INIT: SPSR_EL1 = SPSR_EL1::DEFAULT
    .with_D(true)
    .with_A(true)
    .with_I(true)
    .with_F(true)
    .with_M(spsr_el1::M::AARCH64_EL1_SP_EL1);
