use core::{
    marker::PhantomData,
    ops::{BitOr, BitOrAssign},
};

use crate::{RegisterName, UInt};

#[derive(Clone, Copy)]
pub struct BitField<T: UInt, E, R: RegisterName> {
    mask: T,
    shift: usize,
    phantom: PhantomData<(E, R)>,
}

impl<T: UInt, E, R: RegisterName> BitField<T, E, R> {
    pub const fn new(mask: T, shift: usize) -> Self {
        BitField {
            mask,
            shift,
            phantom: PhantomData,
        }
    }

    pub fn read(&self, register: T) -> T {
        register & (self.mask << self.shift) >> self.shift
    }
}

impl<T: UInt, E: BitFieldEnum<T = T, R = R>, R: RegisterName> BitField<T, E, R> {
    pub fn read_enum(&self, register: T) -> Option<E> {
        E::try_from_value(self.read(register))
    }
}

macro_rules! impl_BitField {
    ($type:ty) => {
        impl<E, R: RegisterName> BitField<$type, E, R> {
            pub const fn val(&self, value: $type) -> BitFieldValue<$type, R> {
                BitFieldValue::<$type, R> {
                    mask: self.mask << self.shift,
                    value: (value & self.mask) << self.shift,
                    phantom: PhantomData,
                }
            }
        }

        // impl<E: BitFieldEnum<T = $type, R = R>, R: RegisterName> BitField<$type, E, R> {
        //     pub const fn val_enum(&self, enuum: E) -> BitFieldValue<$type, R> {
        //         BitFieldValue::<$type, R> {
        //             mask: self.mask << self.shift,
        //             value: (enuum.into_value() & self.mask) << self.shift,
        //             phantom: PhantomData,
        //         }
        //     }
        // }
    };
}

impl_BitField!(u8);
impl_BitField!(u16);
impl_BitField!(u32);
impl_BitField!(u64);
impl_BitField!(u128);

#[derive(Clone, Copy)]
pub struct BitFieldValue<T: UInt, R: RegisterName> {
    mask: T,
    value: T,
    phantom: PhantomData<R>,
}

impl<T: UInt, R: RegisterName> BitFieldValue<T, R> {
    pub fn write(&self, register: T) -> T {
        let register = register & !self.mask;
        register | self.value
    }
}

impl<T: UInt, R: RegisterName> BitOr for BitFieldValue<T, R> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            mask: self.mask | rhs.mask,
            value: self.value | rhs.value,
            phantom: PhantomData,
        }
    }
}

impl<T: UInt, R: RegisterName> BitOrAssign for BitFieldValue<T, R> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.mask |= rhs.mask;
        self.value |= rhs.value;
    }
}

pub trait BitFieldEnum: Sized {
    type T: UInt;
    type R: RegisterName;

    fn try_from_value(value: Self::T) -> Option<Self>;

    fn into_value(self) -> Self::T;
}
