#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::{
    marker::{PhantomData, PhantomPinned},
    ops::*,
};

use arbitrary_int::*;
use bitbybit::*;

pub trait Level: Copy {
    const NUM: usize;

    const VADDR_IDX_LEN: usize;
    const VADDR_IDX_SHIFT: usize;
    const VADDR_IDX_MASK: u64 = ((1 << Self::VADDR_IDX_LEN) - 1) << Self::VADDR_IDX_SHIFT;

    const BLOCK_SIZE: usize = 1 << Self::VADDR_IDX_SHIFT;

    fn idx(vaddr: u64) -> usize {
        ((vaddr & Self::VADDR_IDX_MASK) >> Self::VADDR_IDX_SHIFT) as usize
    }
}

#[derive(Clone, Copy)]
pub struct Level0;
impl Level for Level0 {
    const NUM: usize = 0;

    const VADDR_IDX_LEN: usize = 9;
    const VADDR_IDX_SHIFT: usize = 39;
}

#[derive(Clone, Copy)]
pub struct Level1;
impl Level for Level1 {
    const NUM: usize = 1;

    const VADDR_IDX_LEN: usize = 9;
    const VADDR_IDX_SHIFT: usize = 30;
}

#[derive(Clone, Copy)]
pub struct Level2;
impl Level for Level2 {
    const NUM: usize = 2;

    const VADDR_IDX_LEN: usize = 9;
    const VADDR_IDX_SHIFT: usize = 21;
}

#[derive(Clone, Copy)]
pub struct Level3;
impl Level for Level3 {
    const NUM: usize = 3;

    const VADDR_IDX_LEN: usize = 9;
    const VADDR_IDX_SHIFT: usize = 12;
}

pub struct TranslationTableRef<'a, L: Level> {
    ptr: *const TranslationTable<L>,
    phantom: PhantomData<&'a ()>,
}

impl<'a, L: Level> Deref for TranslationTableRef<'a, L> {
    type Target = TranslationTable<L>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<'a, L: Level> DerefMut for TranslationTableRef<'a, L> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.ptr as *mut _) }
    }
}

#[repr(align(4096), C)]
pub struct TranslationTable<L: Level> {
    entries: [TranslationTableEntry<L>; 512],
    pin: PhantomPinned,
}

impl<L: Level> TranslationTable<L> {
    pub const DEFAULT: Self = Self {
        entries: [TranslationTableEntry::INVALID; 512],
        pin: PhantomPinned,
    };

    pub fn unmap(&mut self, vaddr: u64) {
        self.entries[L::idx(vaddr)] = TranslationTableEntry::INVALID;
    }
}

impl TranslationTable<Level0> {
    pub fn map_block(&mut self, vaddr: u64, paddr: u64) {
        self.entries[Level0::idx(vaddr)] = TranslationTableEntry::<Level0>::block(paddr);
    }

    pub fn map_table(&mut self, vaddr: u64, table_paddr: u64) {
        self.entries[Level0::idx(vaddr)] = TranslationTableEntry::<Level0>::table(table_paddr);
    }
}

impl TranslationTable<Level1> {
    pub fn map_block(&mut self, vaddr: u64, paddr: u64) {
        self.entries[Level1::idx(vaddr)] = TranslationTableEntry::<Level1>::block(paddr);
    }

    pub fn map_table(&mut self, vaddr: u64, table_paddr: u64) {
        self.entries[Level1::idx(vaddr)] = TranslationTableEntry::<Level1>::table(table_paddr);
    }
}

impl TranslationTable<Level2> {
    pub fn map_block(&mut self, vaddr: u64, paddr: u64) {
        self.entries[Level2::idx(vaddr)] = TranslationTableEntry::<Level2>::block(paddr);
    }

    pub fn map_table(&mut self, vaddr: u64, table_paddr: u64) {
        self.entries[Level2::idx(vaddr)] = TranslationTableEntry::<Level2>::table(table_paddr);
    }
}

impl TranslationTable<Level3> {
    pub fn map_page(&mut self, vaddr: u64, paddr: u64) {
        self.entries[Level3::idx(vaddr)] = TranslationTableEntry::<Level3>::page(paddr);
    }
}

#[repr(align(8), C)]
#[derive(Clone, Copy)]
union TranslationTableEntry<L: Level> {
    block: BlockEntry,
    table: TableEntry,
    page: PageEntry,
    invalid: InvalidEntry,
    phantom: PhantomData<L>,
}

impl<L: Level> TranslationTableEntry<L> {
    const INVALID: Self = Self {
        invalid: InvalidEntry::DEFAULT,
    };
}

impl TranslationTableEntry<Level0> {
    fn is_invalid(self) -> bool {
        unsafe { self.block.raw_value & 0b01 == 0b00 }
    }

    fn is_block(self) -> bool {
        unsafe { self.block.raw_value & 0b11 == 0b01 }
    }

    fn is_table(self) -> bool {
        unsafe { self.block.raw_value & 0b11 == 0b11 }
    }

    fn block(paddr: u64) -> Self {
        const PADDR_MASK: u64 = BlockEntry::ADDR_LEVEL0_mask();
        const PADDR_SHIFT: usize = *BlockEntry::ADDR_LEVEL0_BITS.start();

        Self {
            block: BlockEntry::DEFAULT
                .with_ADDR_LEVEL0(u9::from_u64(paddr & PADDR_MASK) >> PADDR_SHIFT),
        }
    }

    fn table(paddr: u64) -> Self {
        const PADDR_MASK: u64 = TableEntry::ADDR_mask();
        const PADDR_SHIFT: usize = *TableEntry::ADDR_BITS.start();

        Self {
            table: TableEntry::DEFAULT.with_ADDR(u36::from_u64(paddr & PADDR_MASK) >> PADDR_SHIFT),
        }
    }
}

impl TranslationTableEntry<Level1> {
    fn is_invalid(self) -> bool {
        unsafe { self.block.raw_value & 0b01 == 0b00 }
    }

    fn is_block(self) -> bool {
        unsafe { self.block.raw_value & 0b11 == 0b01 }
    }

    fn is_table(self) -> bool {
        unsafe { self.block.raw_value & 0b11 == 0b11 }
    }

    fn block(paddr: u64) -> Self {
        const PADDR_MASK: u64 = BlockEntry::ADDR_LEVEL1_mask();
        const PADDR_SHIFT: usize = *BlockEntry::ADDR_LEVEL1_BITS.start();

        Self {
            block: BlockEntry::DEFAULT
                .with_ADDR_LEVEL1(u18::from_u64(paddr & PADDR_MASK) >> PADDR_SHIFT),
        }
    }

    fn table(paddr: u64) -> Self {
        const PADDR_MASK: u64 = TableEntry::ADDR_mask();
        const PADDR_SHIFT: usize = *TableEntry::ADDR_BITS.start();

        Self {
            table: TableEntry::DEFAULT.with_ADDR(u36::from_u64(paddr & PADDR_MASK) >> PADDR_SHIFT),
        }
    }
}

impl TranslationTableEntry<Level2> {
    fn is_invalid(self) -> bool {
        unsafe { self.block.raw_value & 0b01 == 0b00 }
    }

    fn is_block(self) -> bool {
        unsafe { self.block.raw_value & 0b11 == 0b01 }
    }

    fn is_table(self) -> bool {
        unsafe { self.block.raw_value & 0b11 == 0b11 }
    }

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
    fn is_invalid(self) -> bool {
        unsafe { self.block.raw_value & 0b11 != 0b11 }
    }

    fn is_page(self) -> bool {
        unsafe { self.block.raw_value & 0b11 == 0b11 }
    }

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
    #[bits(39..=47, rw)]
    ADDR_LEVEL0: u9,

    #[bits(30..=47, rw)]
    ADDR_LEVEL1: u18,

    #[bits(21..=47, rw)]
    ADDR_LEVEL2: u27,

    #[bits(8..=9, rw)]
    SH: Option<ShareabilityAttributes>,

    #[bits(6..=7, rw)]
    AP: AccessPermissions,

    #[bit(5, rw)]
    NS: bool,

    #[bits(2..=4, rw)]
    ATTR_IDX: u3,
}

#[bitfield(u64, default = 0b11, rw)]
pub struct TableEntry {
    #[bit(63, rw)]
    NS: bool,

    #[bits(12..=47, rw)]
    ADDR: u36,
}

#[bitfield(u64, default = 0b11, rw)]
pub struct PageEntry {
    #[bits(12..=47, rw)]
    ADDR: u36,

    #[bits(8..=9, rw)]
    SH: Option<ShareabilityAttributes>,

    #[bits(6..=7, rw)]
    AP: AccessPermissions,

    #[bit(5, rw)]
    NS: bool,

    #[bits(2..=4, rw)]
    ATTR_IDX: u3,
}

#[bitfield(u64, default = 0, rw)]
pub struct InvalidEntry {}

#[bitenum(u2, exhaustive = false)]
enum ShareabilityAttributes {
    NonShareable = 0b00,
    OuterShareable = 0b10,
    InnerShareable = 0b11,
}

#[bitenum(u2, exhaustive = true)]
enum AccessPermissions {
    PrivReadWrite = 0b00,
    PrivReadWriteUnprivReadWrite = 0b01,
    PrivRead = 0b10,
    PrivReadUnprivRead = 0b11,
}
