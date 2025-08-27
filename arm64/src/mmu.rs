mod translation_table;

pub use translation_table::*;

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

pub struct BlockAttrs {
    mem_typ: MemoryTyp,
    shareability: Shareability,
    access: Access,
    non_secure: bool,
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

pub enum MemoryTyp {
    Device_nGnRnE,
    Normal_nGnRnE,
    Normal_GRE,
}
