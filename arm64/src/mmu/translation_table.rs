#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::marker::PhantomData;

use arbitrary_int::*;
use bitbybit::*;

pub struct Level0;
pub struct Level1;
pub struct Level2;
pub struct Level3;

#[repr(align(4096))]
pub struct TranslationTable<Level> {
    entry: [TranslationTableEntry<Level>; 512],
}

#[repr(align(8))]
#[derive(Clone, Copy)]
union TranslationTableEntry<Level> {
    block: BlockEntry,
    table: TableEntry,
    page: PageEntry,
    invalid: InvalidEntry,
    phantom: PhantomData<Level>,
}

impl<Level> TranslationTableEntry<Level> {
    const INVALID: Self = Self {
        invalid: InvalidEntry::DEFAULT,
    };
}

impl TranslationTableEntry<Level0> {
    fn block(paddr: u64) -> Self {
        const ADDR_MASK: u64 = BlockEntry::ADDR_LEVEL0_mask();
        const ADDR_SHIFT: usize = *BlockEntry::ADDR_LEVEL0_BITS.start();

        Self {
            block: BlockEntry::DEFAULT
                .with_ADDR_LEVEL0(u9::from_u64(paddr & ADDR_MASK) >> ADDR_SHIFT),
        }
    }

    fn table(paddr: u64) -> Self {
        const ADDR_MASK: u64 = TableEntry::ADDR_mask();
        const ADDR_SHIFT: usize = *TableEntry::ADDR_BITS.start();

        Self {
            table: TableEntry::DEFAULT.with_ADDR(u36::from_u64(paddr & ADDR_MASK) >> ADDR_SHIFT),
        }
    }
}

impl TranslationTableEntry<Level1> {
    fn block(paddr: u64) -> Self {
        const ADDR_MASK: u64 = BlockEntry::ADDR_LEVEL1_mask();
        const ADDR_SHIFT: usize = *BlockEntry::ADDR_LEVEL1_BITS.start();

        Self {
            block: BlockEntry::DEFAULT
                .with_ADDR_LEVEL1(u18::from_u64(paddr & ADDR_MASK) >> ADDR_SHIFT),
        }
    }

    fn table(paddr: u64) -> Self {
        const ADDR_MASK: u64 = TableEntry::ADDR_mask();
        const ADDR_SHIFT: usize = *TableEntry::ADDR_BITS.start();

        Self {
            table: TableEntry::DEFAULT.with_ADDR(u36::from_u64(paddr & ADDR_MASK) >> ADDR_SHIFT),
        }
    }
}

impl TranslationTableEntry<Level2> {
    fn block(paddr: u64) -> Self {
        const ADDR_MASK: u64 = BlockEntry::ADDR_LEVEL2_mask();
        const ADDR_SHIFT: usize = *BlockEntry::ADDR_LEVEL2_BITS.start();

        Self {
            block: BlockEntry::DEFAULT
                .with_ADDR_LEVEL2(u27::from_u64(paddr & ADDR_MASK) >> ADDR_SHIFT),
        }
    }

    fn table(paddr: u64) -> Self {
        const ADDR_MASK: u64 = TableEntry::ADDR_mask();
        const ADDR_SHIFT: usize = *TableEntry::ADDR_BITS.start();

        Self {
            table: TableEntry::DEFAULT.with_ADDR(u36::from_u64(paddr & ADDR_MASK) >> ADDR_SHIFT),
        }
    }
}

impl TranslationTableEntry<Level3> {
    fn page(paddr: u64) -> Self {
        const ADDR_MASK: u64 = PageEntry::ADDR_mask();
        const ADDR_SHIFT: usize = *PageEntry::ADDR_BITS.start();

        Self {
            page: PageEntry::DEFAULT.with_ADDR(u36::from_u64(paddr & ADDR_MASK) >> ADDR_SHIFT),
        }
    }
}

#[bitfield(u64, default = 0b01, rw)]
pub struct BlockEntry {
    #[bits(50..=63, rw)]
    UPPER_ATTRS: u14,

    #[bits(39..=47, rw)]
    ADDR_LEVEL0: u9,

    #[bits(30..=47, rw)]
    ADDR_LEVEL1: u18,

    #[bits(21..=47, rw)]
    ADDR_LEVEL2: u27,

    #[bit(16, rw)]
    NT: bool,

    #[bits(2..=11, rw)]
    LOWER_ATTRS: u10,
}

#[bitfield(u64, default = 0b11, rw)]
pub struct TableEntry {
    #[bits(59..=63, rw)]
    ATTRS: u5,

    #[bits(12..=47, rw)]
    ADDR: u36,
}

#[bitfield(u64, default = 0b11, rw)]
pub struct PageEntry {
    #[bits(50..=63, rw)]
    UPPER_ATTRS: u14,

    #[bits(12..=47, rw)]
    ADDR: u36,

    #[bits(2..=11, rw)]
    LOWER_ATTRS: u10,
}

#[bitfield(u64, default = 0, rw)]
pub struct InvalidEntry {}
