use cfg_asm::cfg_naked_asm;
use core::arch::naked_asm;

pub struct DefaultExceptions;

impl Exceptions<ELx_SP_EL0> for DefaultExceptions {}
impl Exceptions<ELx_SP_ELx> for DefaultExceptions {}
impl Exceptions<ELy_AARCH64> for DefaultExceptions {}
impl Exceptions<ELy_AARCH32> for DefaultExceptions {}

#[allow(non_camel_case_types)]
pub struct ELx_SP_EL0;
#[allow(non_camel_case_types)]
pub struct ELx_SP_ELx;
#[allow(non_camel_case_types)]
pub struct ELy_AARCH64;
#[allow(non_camel_case_types)]
pub struct ELy_AARCH32;

pub trait Exceptions<T> {
    fn sync_excp(_frame: &mut ExceptionFrame) {
        loop {}
    }
    fn irq(_frame: &mut ExceptionFrame) {
        loop {}
    }
    fn fiq(_frame: &mut ExceptionFrame) {
        loop {}
    }
    fn serror(_frame: &mut ExceptionFrame) {
        loop {}
    }
}

pub struct ExceptionFrame {
    pub x0: u64,
    pub x1: u64,
    pub x2: u64,
    pub x3: u64,
    pub x4: u64,
    pub x5: u64,
    pub x6: u64,
    pub x7: u64,
    pub x8: u64,
    pub x9: u64,
    pub x10: u64,
    pub x11: u64,
    pub x12: u64,
    pub x13: u64,
    pub x14: u64,
    pub x15: u64,
    pub x16: u64,
    pub x17: u64,
    pub x18: u64,
    pub fp: u64,
    pub lr: u64,
    pub xzr: u64,
}

#[unsafe(naked)]
#[repr(align(2048))]
pub unsafe extern "C" fn vectors<
    Impl: Exceptions<ELx_SP_EL0>
        + Exceptions<ELx_SP_ELx>
        + Exceptions<ELy_AARCH64>
        + Exceptions<ELy_AARCH32>,
>() -> ! {
    cfg_naked_asm!({
        ".balign 0x80",
        "b {sync_elx_sp_el0}",

        ".balign 0x80",
        "b {irq_elx_sp_el0}",

        ".balign 0x80",
        "b {fiq_elx_sp_el0}",

        ".balign 0x80",
        "b {serror_elx_sp_el0}",

        ".balign 0x80",
        "b {sync_elx_sp_elx}",

        ".balign 0x80",
        "b {irq_elx_sp_elx}",

        ".balign 0x80",
        "b {fiq_elx_sp_elx}",

        ".balign 0x80",
        "b {serror_elx_sp_elx}",

        ".balign 0x80",
        "b {sync_ely_aarch64}",

        ".balign 0x80",
        "b {irq_ely_aarch64}",

        ".balign 0x80",
        "b {fiq_ely_aarch64}",

        ".balign 0x80",
        "b {serror_ely_aarch64}",

        ".balign 0x80",
        "b {sync_ely_aarch32}",

        ".balign 0x80",
        "b {irq_ely_aarch32}",

        ".balign 0x80",
        "b {fiq_ely_aarch32}",

        ".balign 0x80",
        "b {serror_ely_aarch32}",
    },
        sync_elx_sp_el0 = sym sync_excp_entry::<ELx_SP_EL0, Impl>,
        irq_elx_sp_el0 = sym irq_entry::<ELx_SP_EL0, Impl>,
        fiq_elx_sp_el0 = sym fiq_entry::<ELx_SP_EL0, Impl>,
        serror_elx_sp_el0 = sym serror_entry::<ELx_SP_EL0, Impl>,

        sync_elx_sp_elx = sym sync_excp_entry::<ELx_SP_ELx, Impl>,
        irq_elx_sp_elx = sym irq_entry::<ELx_SP_ELx, Impl>,
        fiq_elx_sp_elx = sym fiq_entry::<ELx_SP_ELx, Impl>,
        serror_elx_sp_elx = sym serror_entry::<ELx_SP_ELx, Impl>,

        sync_ely_aarch64 = sym sync_excp_entry::<ELy_AARCH64, Impl>,
        irq_ely_aarch64 = sym irq_entry::<ELy_AARCH64, Impl>,
        fiq_ely_aarch64 = sym fiq_entry::<ELy_AARCH64, Impl>,
        serror_ely_aarch64 = sym serror_entry::<ELy_AARCH64, Impl>,

        sync_ely_aarch32 = sym sync_excp_entry::<ELy_AARCH32, Impl>,
        irq_ely_aarch32 = sym irq_entry::<ELy_AARCH32, Impl>,
        fiq_ely_aarch32 = sym fiq_entry::<ELy_AARCH32, Impl>,
        serror_ely_aarch32 = sym serror_entry::<ELy_AARCH32, Impl>,
    )
}

macro_rules! save_regs {
    () => {
        "stp x30, xzr, [sp, #-16]!
         stp x18, x29, [sp, #-16]!
         stp x16, x17, [sp, #-16]!
         stp x14, x15, [sp, #-16]!
         stp x12, x13, [sp, #-16]!
         stp x10, x11, [sp, #-16]!
         stp x8, x9, [sp, #-16]!
         stp x6, x7, [sp, #-16]!
         stp x4, x5, [sp, #-16]!
         stp x2, x3, [sp, #-16]!
         stp x0, x1, [sp, #-16]!"
    };
}

macro_rules! restore_regs {
    () => {
        "ldp x0, x1, [sp], #16
         ldp x2, x3, [sp], #16
         ldp x4, x5, [sp], #16
         ldp x6, x7, [sp], #16
         ldp x8, x9, [sp], #16
         ldp x10, x11, [sp], #16
         ldp x12, x13, [sp], #16
         ldp x14, x15, [sp], #16
         ldp x16, x17, [sp], #16
         ldp x18, x29, [sp], #16
         ldp x30, xzr, [sp], #16"
    };
}

#[unsafe(naked)]
unsafe extern "C" fn sync_excp_entry<T, Impl: Exceptions<T>>() {
    cfg_naked_asm!(
        {
            save_regs!(),

            "mov x0, sp",
            "bl {sync_excp}",

            restore_regs!(),

            "eret",
        },
            sync_excp = sym Impl::sync_excp
    )
}

#[unsafe(naked)]
unsafe extern "C" fn irq_entry<T, Impl: Exceptions<T>>() {
    cfg_naked_asm!(
        {
            save_regs!(),

            "mov x0, sp",
            "bl {irq}",

            restore_regs!(),

            "eret",
        },
            irq = sym Impl::irq
    )
}

#[unsafe(naked)]
unsafe extern "C" fn fiq_entry<T, Impl: Exceptions<T>>() {
    cfg_naked_asm!(
        {
            save_regs!(),

            "mov x0, sp",
            "bl {fiq}",

            restore_regs!(),

            "eret",
        },
            fiq = sym Impl::fiq
    )
}

#[unsafe(naked)]
unsafe extern "C" fn serror_entry<T, Impl: Exceptions<T>>() {
    cfg_naked_asm!(
        {
            save_regs!(),

            "mov x0, sp",
            "bl {serror}",

            restore_regs!(),

            "eret",
        },
            serror = sym Impl::serror
    )
}
