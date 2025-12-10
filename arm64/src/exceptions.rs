use cfg_asm::cfg_naked_asm;
use core::arch::naked_asm;

#[allow(non_camel_case_types)]
pub struct ELx_SP_EL0;
#[allow(non_camel_case_types)]
pub struct ELx_SP_ELx;
#[allow(non_camel_case_types)]
pub struct ELy_AARCH64;
#[allow(non_camel_case_types)]
pub struct ELy_AARCH32;

pub trait Exceptions<EL> {
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

pub trait ExceptionVectors {
    unsafe extern "C" fn sync_excp_elx_sp_el0() -> !;
    unsafe extern "C" fn irq_elx_sp_el0() -> !;
    unsafe extern "C" fn fiq_elx_sp_el0() -> !;
    unsafe extern "C" fn serror_elx_sp_el0() -> !;

    unsafe extern "C" fn sync_excp_elx_sp_elx() -> !;
    unsafe extern "C" fn irq_elx_sp_elx() -> !;
    unsafe extern "C" fn fiq_elx_sp_elx() -> !;
    unsafe extern "C" fn serror_elx_sp_elx() -> !;

    unsafe extern "C" fn sync_excp_ely_aarch64() -> !;
    unsafe extern "C" fn irq_ely_aarch64() -> !;
    unsafe extern "C" fn fiq_ely_aarch64() -> !;
    unsafe extern "C" fn serror_ely_aarch64() -> !;

    unsafe extern "C" fn sync_excp_ely_aarch32() -> !;
    unsafe extern "C" fn irq_ely_aarch32() -> !;
    unsafe extern "C" fn fiq_ely_aarch32() -> !;
    unsafe extern "C" fn serror_ely_aarch32() -> !;
}

pub struct DefaultExceptions;

impl Exceptions<ELx_SP_EL0> for DefaultExceptions {}
impl Exceptions<ELx_SP_ELx> for DefaultExceptions {}
impl Exceptions<ELy_AARCH64> for DefaultExceptions {}
impl Exceptions<ELy_AARCH32> for DefaultExceptions {}

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
#[unsafe(link_section = ".text.vector_table")]
// #[repr(align(2048))]
pub unsafe extern "C" fn vector_table<ExcpVecs>() -> !
where
    ExcpVecs: ExceptionVectors,
{
    cfg_naked_asm!({
        ".balign 0x80",
        "b {sync_excp_elx_sp_el0}",

        ".balign 0x80",
        "b {irq_elx_sp_el0}",

        ".balign 0x80",
        "b {fiq_elx_sp_el0}",

        ".balign 0x80",
        "b {serror_elx_sp_el0}",

        ".balign 0x80",
        "b {sync_excp_elx_sp_elx}",

        ".balign 0x80",
        "b {irq_elx_sp_elx}",

        ".balign 0x80",
        "b {fiq_elx_sp_elx}",

        ".balign 0x80",
        "b {serror_elx_sp_elx}",

        ".balign 0x80",
        "b {sync_excp_ely_aarch64}",

        ".balign 0x80",
        "b {irq_ely_aarch64}",

        ".balign 0x80",
        "b {fiq_ely_aarch64}",

        ".balign 0x80",
        "b {serror_ely_aarch64}",

        ".balign 0x80",
        "b {sync_excp_ely_aarch32}",

        ".balign 0x80",
        "b {irq_ely_aarch32}",

        ".balign 0x80",
        "b {fiq_ely_aarch32}",

        ".balign 0x80",
        "b {serror_ely_aarch32}",
    },
        sync_excp_elx_sp_el0 = sym ExcpVecs::sync_excp_elx_sp_el0,
        irq_elx_sp_el0 = sym  ExcpVecs::irq_elx_sp_el0,
        fiq_elx_sp_el0 = sym  ExcpVecs::fiq_elx_sp_el0,
        serror_elx_sp_el0 = sym ExcpVecs::serror_elx_sp_el0,

        sync_excp_elx_sp_elx = sym ExcpVecs::sync_excp_elx_sp_elx,
        irq_elx_sp_elx = sym ExcpVecs::irq_elx_sp_elx,
        fiq_elx_sp_elx = sym ExcpVecs::fiq_elx_sp_elx,
        serror_elx_sp_elx = sym ExcpVecs::serror_elx_sp_elx,

        sync_excp_ely_aarch64 = sym ExcpVecs::sync_excp_ely_aarch64,
        irq_ely_aarch64 = sym ExcpVecs::irq_ely_aarch64,
        fiq_ely_aarch64 = sym ExcpVecs::fiq_ely_aarch64,
        serror_ely_aarch64 = sym ExcpVecs::serror_ely_aarch64,

        sync_excp_ely_aarch32 = sym ExcpVecs::sync_excp_ely_aarch32,
        irq_ely_aarch32 = sym ExcpVecs::irq_ely_aarch32,
        fiq_ely_aarch32 = sym ExcpVecs::fiq_ely_aarch32,
        serror_ely_aarch32 = sym ExcpVecs::serror_ely_aarch32,
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

macro_rules! excp_vector {
    ($excp_sym:expr) => {
        cfg_naked_asm!(
            {
                save_regs!(),

                "mov x0, sp",
                "bl {excp}",

                restore_regs!(),

                "eret",
            },
                excp = sym $excp_sym,
        )
    };
}

impl<T> ExceptionVectors for T
where
    T: Exceptions<ELx_SP_EL0>
        + Exceptions<ELx_SP_ELx>
        + Exceptions<ELy_AARCH64>
        + Exceptions<ELy_AARCH32>,
{
    #[unsafe(naked)]
    unsafe extern "C" fn sync_excp_elx_sp_el0() -> ! {
        excp_vector!(<T as Exceptions<ELx_SP_EL0>>::sync_excp)
    }

    #[unsafe(naked)]
    unsafe extern "C" fn irq_elx_sp_el0() -> ! {
        excp_vector!(<T as Exceptions<ELx_SP_EL0>>::irq)
    }

    #[unsafe(naked)]
    unsafe extern "C" fn fiq_elx_sp_el0() -> ! {
        excp_vector!(<T as Exceptions<ELx_SP_EL0>>::fiq)
    }

    #[unsafe(naked)]
    unsafe extern "C" fn serror_elx_sp_el0() -> ! {
        excp_vector!(<T as Exceptions<ELx_SP_EL0>>::serror)
    }

    #[unsafe(naked)]
    unsafe extern "C" fn sync_excp_elx_sp_elx() -> ! {
        excp_vector!(<T as Exceptions<ELx_SP_ELx>>::sync_excp)
    }

    #[unsafe(naked)]
    unsafe extern "C" fn irq_elx_sp_elx() -> ! {
        excp_vector!(<T as Exceptions<ELx_SP_ELx>>::irq)
    }

    #[unsafe(naked)]
    unsafe extern "C" fn fiq_elx_sp_elx() -> ! {
        excp_vector!(<T as Exceptions<ELx_SP_ELx>>::fiq)
    }

    #[unsafe(naked)]
    unsafe extern "C" fn serror_elx_sp_elx() -> ! {
        excp_vector!(<T as Exceptions<ELx_SP_ELx>>::serror)
    }

    #[unsafe(naked)]
    unsafe extern "C" fn sync_excp_ely_aarch64() -> ! {
        excp_vector!(<T as Exceptions<ELy_AARCH64>>::sync_excp)
    }

    #[unsafe(naked)]
    unsafe extern "C" fn irq_ely_aarch64() -> ! {
        excp_vector!(<T as Exceptions<ELy_AARCH64>>::irq)
    }

    #[unsafe(naked)]
    unsafe extern "C" fn fiq_ely_aarch64() -> ! {
        excp_vector!(<T as Exceptions<ELy_AARCH64>>::fiq)
    }

    #[unsafe(naked)]
    unsafe extern "C" fn serror_ely_aarch64() -> ! {
        excp_vector!(<T as Exceptions<ELy_AARCH64>>::serror)
    }

    #[unsafe(naked)]
    unsafe extern "C" fn sync_excp_ely_aarch32() -> ! {
        excp_vector!(<T as Exceptions<ELy_AARCH32>>::sync_excp)
    }

    #[unsafe(naked)]
    unsafe extern "C" fn irq_ely_aarch32() -> ! {
        excp_vector!(<T as Exceptions<ELy_AARCH32>>::irq)
    }

    #[unsafe(naked)]
    unsafe extern "C" fn fiq_ely_aarch32() -> ! {
        excp_vector!(<T as Exceptions<ELy_AARCH32>>::fiq)
    }

    #[unsafe(naked)]
    unsafe extern "C" fn serror_ely_aarch32() -> ! {
        excp_vector!(<T as Exceptions<ELy_AARCH32>>::serror)
    }
}
