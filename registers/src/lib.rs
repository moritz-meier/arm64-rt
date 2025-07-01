#![no_std]

mod field;
mod local_register;
mod register;

use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

pub use field::*;
pub use local_register::*;
pub use register::*;

pub trait UInt:
    Copy
    + Clone
    + Default
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Shl<usize, Output = Self>
    + ShlAssign
    + Shr<usize, Output = Self>
    + ShrAssign
    + Not<Output = Self>
{
    const ZERO: Self;
    const ONE: Self;
}

macro_rules! impl_UInt {
    ($type:ty) => {
        impl UInt for $type {
            const ZERO: Self = 0;
            const ONE: Self = 1;
        }
    };
}

impl_UInt!(u8);
impl_UInt!(u16);
impl_UInt!(u32);
impl_UInt!(u64);
impl_UInt!(u128);

pub const SCTLR_EL1: ReadWrite<u64, SCTLR_EL1::Register> = ReadWrite::new(3475457);

#[allow(non_snake_case)]
mod SCTLR_EL1 {
    use crate::RegisterName;

    #[derive(Default)]
    pub struct Register {}
    impl RegisterName for Register {}
}

fn foo() {
    unsafe {
        let x = SCTLR_EL1.get();
    };
}
