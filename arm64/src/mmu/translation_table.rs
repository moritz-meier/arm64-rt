#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::{
    marker::{PhantomData, PhantomPinned},
    ptr::addr_of,
};

use arbitrary_int::*;
use bitbybit::*;

use super::*;

#[repr(align(4096), C)]
pub struct TranslationTable<L: TranslationLevel> {
    entries: [TranslationTableEntry<L>; 512],
    pin: PhantomPinned,
}

impl<L: TranslationLevel> TranslationTable<L> {
    pub const DEFAULT: Self = Self {
        entries: [TranslationTableEntry::INVALID; 512],
        pin: PhantomPinned,
    };

    pub fn base_addr(&self) -> *const u64 {
        addr_of!(self.entries) as *const u64
    }
}

impl TranslationTable<Level0> {
    pub fn unmap(&mut self, vaddr: u64) {
        let vaddr = VirtAddr::new_with_raw_value(vaddr);
        self.entries[vaddr.LEVEL0_IDX().as_usize()] = TranslationTableEntry::INVALID;
    }

    pub fn map_table(&mut self, vaddr: u64, table_paddr: u64, attrs: TableAttrs) {
        let vaddr = VirtAddr::new_with_raw_value(vaddr);
        self.entries[vaddr.LEVEL0_IDX().as_usize()] =
            TranslationTableEntry::<Level0>::table(table_paddr, attrs)
    }
}

impl TranslationTable<Level1> {
    pub fn unmap(&mut self, vaddr: u64) {
        let vaddr = VirtAddr::new_with_raw_value(vaddr);
        self.entries[vaddr.LEVEL1_IDX().as_usize()] = TranslationTableEntry::INVALID;
    }

    pub fn map_table(&mut self, vaddr: u64, table_paddr: u64, attrs: TableAttrs) {
        let vaddr = VirtAddr::new_with_raw_value(vaddr);
        self.entries[vaddr.LEVEL1_IDX().as_usize()] =
            TranslationTableEntry::<Level1>::table(table_paddr, attrs)
    }

    pub fn map_block(&mut self, vaddr: u64, paddr: u64, attrs: BlockAttrs) {
        let vaddr = VirtAddr::new_with_raw_value(vaddr);
        self.entries[vaddr.LEVEL1_IDX().as_usize()] =
            TranslationTableEntry::<Level1>::block(paddr, attrs)
    }
}

impl TranslationTable<Level2> {
    pub fn unmap(&mut self, vaddr: u64) {
        let vaddr = VirtAddr::new_with_raw_value(vaddr);
        self.entries[vaddr.LEVEL2_IDX().as_usize()] = TranslationTableEntry::INVALID;
    }

    pub fn map_table(&mut self, vaddr: u64, table_paddr: u64, attrs: TableAttrs) {
        let vaddr = VirtAddr::new_with_raw_value(vaddr);
        self.entries[vaddr.LEVEL2_IDX().as_usize()] =
            TranslationTableEntry::<Level2>::table(table_paddr, attrs)
    }

    pub fn map_block(&mut self, vaddr: u64, paddr: u64, attrs: BlockAttrs) {
        let vaddr = VirtAddr::new_with_raw_value(vaddr);
        self.entries[vaddr.LEVEL2_IDX().as_usize()] =
            TranslationTableEntry::<Level2>::block(paddr, attrs)
    }
}

impl TranslationTable<Level3> {
    pub fn unmap(&mut self, vaddr: u64) {
        let vaddr = VirtAddr::new_with_raw_value(vaddr);
        self.entries[vaddr.LEVEL3_IDX().as_usize()] = TranslationTableEntry::INVALID;
    }

    pub fn map_page(&mut self, vaddr: u64, paddr: u64, attrs: PageAttrs) {
        let vaddr = VirtAddr::new_with_raw_value(vaddr);
        self.entries[vaddr.LEVEL3_IDX().as_usize()] =
            TranslationTableEntry::<Level3>::page(paddr, attrs)
    }
}

#[repr(align(8), C)]
#[derive(Clone, Copy)]
union TranslationTableEntry<LEVEL> {
    block: BlockEntry,
    table: TableEntry,
    page: PageEntry,
    invalid: InvalidEntry,
    phantom: PhantomData<LEVEL>,
}

impl<L: TranslationLevel> TranslationTableEntry<L> {
    const INVALID: Self = Self {
        invalid: InvalidEntry::DEFAULT,
    };

    fn is_invalid(self) -> bool {
        if L::NUM < 3 {
            unsafe { !self.invalid.VALID() }
        } else {
            unsafe { !self.invalid.VALID() || !self.page.PAGE() }
        }
    }

    fn is_table(self) -> bool {
        if L::NUM < 3 {
            unsafe { self.table.VALID() && self.table.TABLE() }
        } else {
            false
        }
    }

    fn is_block(self) -> bool {
        if L::NUM < 3 {
            unsafe { self.block.VALID() && !self.table.TABLE() }
        } else {
            false
        }
    }

    fn is_page(self) -> bool {
        if L::NUM < 3 {
            false
        } else {
            unsafe { self.page.VALID() && self.page.PAGE() }
        }
    }
}

impl TranslationTableEntry<Level0> {
    fn table(paddr: u64, attrs: TableAttrs) -> Self {
        const PADDR_MASK: u64 = TableEntry::ADDR_mask();
        const PADDR_SHIFT: usize = *TableEntry::ADDR_BITS.start();
        let paddr = u36::from_u64((paddr & PADDR_MASK) >> PADDR_SHIFT);

        let ns = match attrs.security {
            SecurityDomain::NonSecure => true,
            SecurityDomain::Secure => false,
        };

        Self {
            table: TableEntry::DEFAULT.with_ADDR(paddr).with_NS(ns),
        }
    }
}

impl TranslationTableEntry<Level1> {
    fn table(paddr: u64, attrs: TableAttrs) -> Self {
        const PADDR_MASK: u64 = TableEntry::ADDR_mask();
        const PADDR_SHIFT: usize = *TableEntry::ADDR_BITS.start();
        let paddr = u36::from_u64((paddr & PADDR_MASK) >> PADDR_SHIFT);

        let ns = match attrs.security {
            SecurityDomain::NonSecure => true,
            SecurityDomain::Secure => false,
        };

        Self {
            table: TableEntry::DEFAULT.with_ADDR(paddr).with_NS(ns),
        }
    }

    fn block(paddr: u64, attrs: BlockAttrs) -> Self {
        const PADDR_MASK: u64 = BlockEntry::ADDR_LEVEL1_mask();
        const PADDR_SHIFT: usize = *BlockEntry::ADDR_LEVEL1_BITS.start();
        let paddr = u18::from_u64((paddr & PADDR_MASK) >> PADDR_SHIFT);

        let sh = match attrs.shareability {
            super::Shareability::Non => Shareability::Non,
            super::Shareability::Outer => Shareability::Outer,
            super::Shareability::Inner => Shareability::Inner,
        };

        let ap = match attrs.access {
            super::Access::PrivRead => Access::PrivRead,
            super::Access::PrivReadWrite => Access::PrivReadWrite,
            super::Access::PrivReadUnprivRead => Access::PrivReadUnprivRead,
            super::Access::PrivReadWriteUnprivReadWrite => Access::PrivReadWriteUnprivReadWrite,
        };

        let ns = match attrs.security {
            SecurityDomain::NonSecure => true,
            SecurityDomain::Secure => false,
        };

        let attr_idx = match attrs.mem_typ {
            MemoryTyp::Device_nGnRnE => 0,
            MemoryTyp::Normal_NonCacheable => 1,
            MemoryTyp::Normal_WriteThrough => 2,
            MemoryTyp::Normal_Cacheable => 3,
            MemoryTyp::Normal_InnerCacheable => 4,
            MemoryTyp::Normal_OuterCacheable => 5,
        };

        Self {
            block: BlockEntry::DEFAULT
                .with_ADDR_LEVEL1(paddr)
                .with_AF(true)
                .with_SH(sh)
                .with_AP(ap)
                .with_NS(ns)
                .with_ATTR_IDX(u3::from_u8(attr_idx)),
        }
    }
}

impl TranslationTableEntry<Level2> {
    fn table(paddr: u64, attrs: TableAttrs) -> Self {
        const PADDR_MASK: u64 = TableEntry::ADDR_mask();
        const PADDR_SHIFT: usize = *TableEntry::ADDR_BITS.start();
        let paddr = u36::from_u64((paddr & PADDR_MASK) >> PADDR_SHIFT);

        let ns = match attrs.security {
            SecurityDomain::NonSecure => true,
            SecurityDomain::Secure => false,
        };

        Self {
            table: TableEntry::DEFAULT.with_ADDR(paddr).with_NS(ns),
        }
    }

    fn block(paddr: u64, attrs: BlockAttrs) -> Self {
        const PADDR_MASK: u64 = BlockEntry::ADDR_LEVEL2_mask();
        const PADDR_SHIFT: usize = *BlockEntry::ADDR_LEVEL2_BITS.start();
        let paddr = u27::from_u64((paddr & PADDR_MASK) >> PADDR_SHIFT);

        let sh = match attrs.shareability {
            super::Shareability::Non => Shareability::Non,
            super::Shareability::Outer => Shareability::Outer,
            super::Shareability::Inner => Shareability::Inner,
        };

        let ap = match attrs.access {
            super::Access::PrivRead => Access::PrivRead,
            super::Access::PrivReadWrite => Access::PrivReadWrite,
            super::Access::PrivReadUnprivRead => Access::PrivReadUnprivRead,
            super::Access::PrivReadWriteUnprivReadWrite => Access::PrivReadWriteUnprivReadWrite,
        };

        let ns = match attrs.security {
            SecurityDomain::NonSecure => true,
            SecurityDomain::Secure => false,
        };

        let attr_idx = match attrs.mem_typ {
            MemoryTyp::Device_nGnRnE => 0,
            MemoryTyp::Normal_NonCacheable => 1,
            MemoryTyp::Normal_WriteThrough => 2,
            MemoryTyp::Normal_Cacheable => 3,
            MemoryTyp::Normal_InnerCacheable => 4,
            MemoryTyp::Normal_OuterCacheable => 5,
        };

        Self {
            block: BlockEntry::DEFAULT
                .with_ADDR_LEVEL2(paddr)
                .with_AF(true)
                .with_SH(sh)
                .with_AP(ap)
                .with_NS(ns)
                .with_ATTR_IDX(u3::from_u8(attr_idx)),
        }
    }
}

impl TranslationTableEntry<Level3> {
    fn page(paddr: u64, attrs: PageAttrs) -> Self {
        const PADDR_MASK: u64 = PageEntry::ADDR_mask();
        const PADDR_SHIFT: usize = *PageEntry::ADDR_BITS.start();
        let paddr = u36::from_u64((paddr & PADDR_MASK) >> PADDR_SHIFT);

        let sh = match attrs.shareability {
            super::Shareability::Non => Shareability::Non,
            super::Shareability::Outer => Shareability::Outer,
            super::Shareability::Inner => Shareability::Inner,
        };

        let ap = match attrs.access {
            super::Access::PrivRead => Access::PrivRead,
            super::Access::PrivReadWrite => Access::PrivReadWrite,
            super::Access::PrivReadUnprivRead => Access::PrivReadUnprivRead,
            super::Access::PrivReadWriteUnprivReadWrite => Access::PrivReadWriteUnprivReadWrite,
        };

        let ns = match attrs.security {
            SecurityDomain::NonSecure => true,
            SecurityDomain::Secure => false,
        };

        let attr_idx = match attrs.mem_typ {
            MemoryTyp::Device_nGnRnE => 0,
            MemoryTyp::Normal_NonCacheable => 1,
            MemoryTyp::Normal_WriteThrough => 2,
            MemoryTyp::Normal_Cacheable => 3,
            MemoryTyp::Normal_InnerCacheable => 4,
            MemoryTyp::Normal_OuterCacheable => 5,
        };

        Self {
            page: PageEntry::DEFAULT
                .with_ADDR(paddr)
                .with_AF(true)
                .with_SH(sh)
                .with_AP(ap)
                .with_NS(ns)
                .with_ATTR_IDX(u3::from_u8(attr_idx)),
        }
    }
}

#[bitfield(u64, default = 0, rw)]
struct InvalidEntry {
    #[bit(0, r)]
    VALID: bool,
}

#[bitfield(u64, default = 0b11, rw)]
struct TableEntry {
    #[bit(63, rw)]
    NS: bool,

    #[bits(12..=47, rw)]
    ADDR: u36,

    #[bit(1, r)]
    TABLE: bool,

    #[bit(0, r)]
    VALID: bool,
}

#[bitfield(u64, default = 0b01, rw)]
struct BlockEntry {
    #[bits(30..=47, rw)]
    ADDR_LEVEL1: u18,

    #[bits(21..=47, rw)]
    ADDR_LEVEL2: u27,

    #[bit(10, rw)]
    AF: bool,

    #[bits(8..=9, rw)]
    SH: Option<Shareability>,

    #[bits(6..=7, rw)]
    AP: Access,

    #[bit(5, rw)]
    NS: bool,

    #[bits(2..=4, rw)]
    ATTR_IDX: u3,

    #[bit(0, r)]
    VALID: bool,
}

#[bitfield(u64, default = 0b11, rw)]
struct PageEntry {
    #[bits(12..=47, rw)]
    ADDR: u36,

    #[bit(10, rw)]
    AF: bool,

    #[bits(8..=9, rw)]
    SH: Option<Shareability>,

    #[bits(6..=7, rw)]
    AP: Access,

    #[bit(5, rw)]
    NS: bool,

    #[bits(2..=4, rw)]
    ATTR_IDX: u3,

    #[bit(1, r)]
    PAGE: bool,

    #[bit(0, r)]
    VALID: bool,
}

#[bitenum(u2, exhaustive = false)]
enum Shareability {
    Non = 0b00,
    Outer = 0b10,
    Inner = 0b11,
}

#[bitenum(u2, exhaustive = true)]
enum Access {
    PrivReadWrite = 0b00,
    PrivReadWriteUnprivReadWrite = 0b01,
    PrivRead = 0b10,
    PrivReadUnprivRead = 0b11,
}

#[bitfield(u64, rw)]
struct VirtAddr {
    #[bits(39..=47, rw)]
    LEVEL0_IDX: u9,

    #[bits(30..=38, rw)]
    LEVEL1_IDX: u9,

    #[bits(21..=29, rw)]
    LEVEL2_IDX: u9,

    #[bits(12..=20, rw)]
    LEVEL3_IDX: u9,

    #[bits(0..=11, rw)]
    OFFSET: u12,
}
