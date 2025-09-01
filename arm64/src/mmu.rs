mod translation_table;

use arbitrary_int::*;
pub use translation_table::*;

use crate::{dsb, isb, sys_regs::*};

pub struct MMU;

impl MMU {
    pub fn disable_el3() {
        SCTLR_EL3.modify(|sctlr_el3| sctlr_el3.with_M(false));

        isb!("sy")
    }

    pub fn disable_el2() {
        SCTLR_EL2.modify(|sctlr_el2| sctlr_el2.with_M(false));

        isb!("sy")
    }

    pub fn disable_el1() {
        SCTLR_EL1.modify(|sctlr_el1| sctlr_el1.with_M(false));

        isb!("sy")
    }

    pub fn enable_el3(table_paddr: u64) {
        Self::invalidate_tlb_el3_all();

        let id_aa64mmfr0_el1 = ID_AA64MMFR0_EL1.read();
        TCR_EL3.write(
            TCR_EL3::DEFAULT
                .with_T0SZ(u6::from_u8(0x10))
                .with_TG0(u2::from_u8(0x00))
                .with_PS(u3::from_u8(id_aa64mmfr0_el1.PARANGE().as_u8())),
        );

        MAIR_EL3.write(
            MAIR_EL3::DEFAULT
                .with_ATTR0(0b00000000)
                .with_ATTR1(0b01000100)
                .with_ATTR2(0b01001111)
                .with_ATTR3(0b11111111),
        );

        let table_paddr = u47::from_u64(
            (table_paddr & TTBR0_EL3::BADDR_mask()) >> *TTBR0_EL3::BADDR_BITS.start(),
        );

        TTBR0_EL3.write(TTBR0_EL3::DEFAULT.with_BADDR(table_paddr));

        SCTLR_EL3.modify(|sctlr_el3| sctlr_el3.with_M(true));

        isb!("sy")
    }

    pub fn enable_el2(table_paddr: u64) {
        Self::invalidate_tlb_el2_all();

        let id_aa64mmfr0_el1 = ID_AA64MMFR0_EL1.read();
        TCR_EL2.write(
            TCR_EL2::DEFAULT
                .with_T0SZ(u6::from_u8(0x10))
                .with_TG0(u2::from_u8(0x00))
                .with_PS(u3::from_u8(id_aa64mmfr0_el1.PARANGE().as_u8())),
        );

        MAIR_EL2.write(
            MAIR_EL2::DEFAULT
                .with_ATTR0(0b00000000)
                .with_ATTR1(0b01000100)
                .with_ATTR2(0b01001111)
                .with_ATTR3(0b11111111),
        );

        let table_paddr = u47::from_u64(
            (table_paddr & TTBR0_EL2::BADDR_mask()) >> *TTBR0_EL2::BADDR_BITS.start(),
        );

        TTBR0_EL2.write(TTBR0_EL2::DEFAULT.with_BADDR(table_paddr));

        SCTLR_EL2.modify(|sctlr_el2| sctlr_el2.with_M(true));

        isb!("sy")
    }

    pub fn enable_el1(table_paddr: u64) {
        // Self::invalidate_tlb_el1_all();

        TCR_EL1.write(
            TCR_EL1::DEFAULT
                .with_T0SZ(u6::from_u8(0x10))
                .with_TG0(u2::from_u8(0x00)),
        );

        MAIR_EL1.write(
            MAIR_EL1::DEFAULT
                .with_ATTR0(0b00000000)
                .with_ATTR1(0b01000100)
                .with_ATTR2(0b01001111)
                .with_ATTR3(0b11111111),
        );

        let table_paddr = u47::from_u64(
            (table_paddr & TTBR0_EL1::BADDR_mask()) >> *TTBR0_EL1::BADDR_BITS.start(),
        );

        TTBR0_EL1.write(TTBR0_EL1::DEFAULT.with_BADDR(table_paddr));

        SCTLR_EL1.modify(|sctlr_el1| sctlr_el1.with_M(true));

        isb!("sy")
    }

    pub fn invalidate_tlb_el3_all() {
        unsafe { core::arch::asm!("tlbi alle3is") }
        isb!("sy")
    }

    pub fn invalidate_tlb_el2_all() {
        unsafe { core::arch::asm!("tlbi alle2is") }
        isb!("sy")
    }

    pub fn invalidate_tlb_el1_all() {
        unsafe { core::arch::asm!("tlbi alle1is") }
        isb!("sy")
    }
}

pub trait TranslationLevel {
    const NUM: usize;
}

#[derive(Clone, Copy)]
pub struct Level0;
impl TranslationLevel for Level0 {
    const NUM: usize = 0;
}

#[derive(Clone, Copy)]
pub struct Level1;
impl TranslationLevel for Level1 {
    const NUM: usize = 1;
}

#[derive(Clone, Copy)]
pub struct Level2;
impl TranslationLevel for Level2 {
    const NUM: usize = 2;
}

#[derive(Clone, Copy)]
pub struct Level3;
impl TranslationLevel for Level3 {
    const NUM: usize = 3;
}

pub struct TableAttrs {
    non_secure: bool,
}

impl TableAttrs {
    pub const NON_SECURE: Self = Self { non_secure: true };
}

pub struct BlockAttrs {
    mem_typ: MemoryTyp,
    shareability: Shareability,
    access: Access,
    security: SecurityDomain,
}

impl BlockAttrs {
    pub const DEFAULT: Self = Self {
        mem_typ: MemoryTyp::Device_nGnRnE,
        shareability: Shareability::Non,
        access: Access::PrivReadWriteUnprivReadWrite,
        security: SecurityDomain::NonSecure,
    };
}

pub struct PageAttrs {
    mem_typ: MemoryTyp,
    shareability: Shareability,
    access: Access,
    security: SecurityDomain,
}

pub enum MemoryTyp {
    Device_nGnRnE,
    Normal_NonCacheable,
    Normal_InnerCacheable,
    Normal_InnerOuterCacheable,
}

pub enum Shareability {
    Non,
    Outer,
    Inner,
}

pub enum Access {
    PrivRead,
    PrivReadWrite,
    PrivReadUnprivRead,
    PrivReadWriteUnprivReadWrite,
}

pub enum SecurityDomain {
    NonSecure,
    Secure,
}
