#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::marker::{PhantomData, PhantomPinned};

use arbitrary_int::*;
use bitbybit::*;

use super::*;

#[repr(align(4096), C)]
struct TranslationTable<L: TranslationLevel> {
    entries: [TranslationTableEntry<L>; 512],
    pin: PhantomPinned,
}

impl<L: TranslationLevel> TranslationTable<L> {
    const DEFAULT: Self = Self {
        entries: [TranslationTableEntry::INVALID; 512],
        pin: PhantomPinned,
    };
}

impl TranslationTable<Level0> {
    fn unmap(&mut self, vaddr: u64) {
        let vaddr = VirtAddr::new_with_raw_value(vaddr);
        self.entries[vaddr.LEVEL0_IDX().as_usize()] = TranslationTableEntry::INVALID;
    }

    fn map_block(&mut self, vaddr: u64, paddr: u64) {
        let vaddr = VirtAddr::new_with_raw_value(vaddr);
        self.entries[vaddr.LEVEL0_IDX().as_usize()] = TranslationTableEntry {
            block: BlockEntry::DEFAULT.with_ADDR_LEVEL0(paddr).with_SH(sh),
        }
    }
}

impl TranslationTable<Level1> {
    fn unmap(&mut self, vaddr: u64) {
        let vaddr = VirtAddr::new_with_raw_value(vaddr);
        self.entries[vaddr.LEVEL1_IDX().as_usize()] = TranslationTableEntry::INVALID;
    }
}

impl TranslationTable<Level2> {
    fn unmap(&mut self, vaddr: u64) {
        let vaddr = VirtAddr::new_with_raw_value(vaddr);
        self.entries[vaddr.LEVEL2_IDX().as_usize()] = TranslationTableEntry::INVALID;
    }
}

impl TranslationTable<Level3> {
    fn unmap(&mut self, vaddr: u64) {
        let vaddr = VirtAddr::new_with_raw_value(vaddr);
        self.entries[vaddr.LEVEL3_IDX().as_usize()] = TranslationTableEntry::INVALID;
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
    fn block(paddr: u64, attrs: BlockAttrs) -> Self {
        const PADDR_MASK: u64 = BlockEntry::ADDR_LEVEL0_mask();
        const PADDR_SHIFT: usize = *BlockEntry::ADDR_LEVEL0_BITS.start();

        let paddr = u9::from_u64((paddr & PADDR_MASK) >> PADDR_SHIFT);
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

        let attr_idx = match attrs.mem_typ {
            MemoryTyp::Device_nGnRnE => 0,
            MemoryTyp::Normal_nGnRnE => 1,
            MemoryTyp::Normal_GRE => 2,
        };

        Self {
            block: BlockEntry::DEFAULT
                .with_ADDR_LEVEL0(paddr)
                .with_SH(sh)
                .with_AP(ap)
                .with_NS(attrs.non_secure)
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
    #[bits(39..=47, rw)]
    ADDR_LEVEL0: u9,

    #[bits(30..=47, rw)]
    ADDR_LEVEL1: u18,

    #[bits(21..=47, rw)]
    ADDR_LEVEL2: u27,

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
}
