use core::marker::PhantomData;

use crate::{RegisterName, UInt};

#[derive(Clone, Copy, Debug, Default)]
pub struct LocalRegisterCopy<T: UInt, R: RegisterName, W: RegisterName = R> {
    value: T,
    phantom: PhantomData<(R, W)>,
}

impl<T: UInt, R: RegisterName, W: RegisterName> LocalRegisterCopy<T, R, W> {
    pub const fn new(value: T) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }

    pub const fn default() -> Self {
        Self {
            value: T::ZERO,
            phantom: PhantomData,
        }
    }

    pub const fn get_value(&self) -> T {
        self.value
    }

    pub const fn set_value(&mut self, value: T) {
        self.value = value
    }
}

impl<T: UInt, R: RegisterName, W: RegisterName> From<T> for LocalRegisterCopy<T, R, W> {
    fn from(value: T) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }
}

macro_rules! impl_LocalRegisterCopy {
    ($type:ty) => {
        impl<R: RegisterName, W: RegisterName> From<LocalRegisterCopy<$type, R, W>> for $type {
            fn from(register: LocalRegisterCopy<$type, R, W>) -> Self {
                register.value
            }
        }
    };
}

impl_LocalRegisterCopy!(u8);
impl_LocalRegisterCopy!(u16);
impl_LocalRegisterCopy!(u32);
impl_LocalRegisterCopy!(u64);
impl_LocalRegisterCopy!(u128);
