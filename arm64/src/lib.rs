#![no_std]
#![feature(fn_align)]
#![feature(ptr_mask)]

#[cfg(not(target_arch = "aarch64"))]
compile_error!("Only target_arch = \"aarch64\" is supported.");

#[cfg(not(target_os = "none"))]
compile_error!("Only target_os = \"none\" is supported.");

#[cfg(not(target_endian = "little"))]
compile_error!("Only target_endian = \"little\" is supported.");

#[cfg(not(any(feature = "cortex-a53")))]
compile_error!("A ARMv8A impl \"{cortex-a53, }\" must be selected.");

pub mod cache;
pub mod exceptions;
pub mod start;

pub use start::*;

#[macro_export]
macro_rules! bitmask {
    (bit: $bit:literal) => {
        bitmask!(msb: $bit, lsb: $bit)
    };

    (msb: $msb:literal, lsb: $lsb:literal) => {
        ((1 << ($msb - $lsb)) + ((1 << ($msb - $lsb)) - 1)) << $lsb
    };
}

#[macro_export]
macro_rules! read_bitfield {
    ($register:expr, bit: $bit:literal) => {
        ($register & crate::bitmask!(bit: $bit)) >> $lsb
    };

    ($register:expr, msb: $msb:literal, lsb: $lsb:literal) => {
        ($register & crate::bitmask!(msb: $msb, lsb: $lsb)) >> $lsb
    };
}
