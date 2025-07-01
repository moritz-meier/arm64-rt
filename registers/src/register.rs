use core::{
    cell::UnsafeCell,
    marker::PhantomData,
    ptr::{read_volatile, write_volatile},
};

use crate::{LocalRegisterCopy, UInt};

pub trait RegisterName {}

pub trait ReadRegister<T: UInt, R: RegisterName> {
    unsafe fn get_value(&self) -> T;

    unsafe fn get(&self) -> LocalRegisterCopy<T, R> {
        unsafe { LocalRegisterCopy::new(self.get_value()) }
    }
}

pub trait WriteRegister<T: UInt, W: RegisterName> {
    unsafe fn set_value(&self, register: T);

    unsafe fn set(&self, register: LocalRegisterCopy<T, W>) {
        unsafe { self.set_value(register.get_value()) };
    }
}

#[derive(Default)]
#[repr(transparent)]
pub struct ReadOnly<T: UInt, R: RegisterName> {
    value: UnsafeCell<T>,
    phantom: PhantomData<R>,
}

impl<T: UInt, R: RegisterName> ReadOnly<T, R> {
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            phantom: PhantomData,
        }
    }

    pub const fn default() -> Self {
        Self {
            value: UnsafeCell::new(T::ZERO),
            phantom: PhantomData,
        }
    }
}

impl<T: UInt, R: RegisterName> ReadRegister<T, R> for ReadOnly<T, R> {
    unsafe fn get_value(&self) -> T {
        unsafe { read_volatile(self.value.get()) }
    }
}

#[derive(Default)]
#[repr(transparent)]
pub struct WriteOnly<T: UInt, W: RegisterName> {
    value: UnsafeCell<T>,
    phantom: PhantomData<W>,
}

impl<T: UInt, W: RegisterName> WriteOnly<T, W> {
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            phantom: PhantomData,
        }
    }

    pub const fn default() -> Self {
        Self {
            value: UnsafeCell::new(T::ZERO),
            phantom: PhantomData,
        }
    }
}

impl<T: UInt, W: RegisterName> WriteRegister<T, W> for WriteOnly<T, W> {
    unsafe fn set_value(&self, value: T) {
        unsafe {
            write_volatile(self.value.get(), value);
        }
    }
}

#[derive(Default)]
#[repr(transparent)]
pub struct ReadWrite<T: UInt, R: RegisterName, W: RegisterName = R> {
    value: UnsafeCell<T>,
    phantom: PhantomData<(R, W)>,
}

impl<T: UInt, R: RegisterName, W: RegisterName> ReadWrite<T, R, W> {
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            phantom: PhantomData,
        }
    }

    pub const fn default() -> Self {
        Self {
            value: UnsafeCell::new(T::ZERO),
            phantom: PhantomData,
        }
    }
}

impl<T: UInt, R: RegisterName, W: RegisterName> ReadRegister<T, R> for ReadWrite<T, R, W> {
    unsafe fn get_value(&self) -> T {
        unsafe { read_volatile(self.value.get()) }
    }
}

impl<T: UInt, R: RegisterName, W: RegisterName> WriteRegister<T, W> for ReadWrite<T, R, W> {
    unsafe fn set_value(&self, value: T) {
        unsafe {
            write_volatile(self.value.get(), value);
        }
    }
}
