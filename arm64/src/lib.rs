#![no_std]
#![feature(fn_align)]

#[cfg(not(target_arch = "aarch64"))]
compile_error!("Only target_arch = \"aarch64\" is supported.");

#[cfg(not(target_os = "none"))]
compile_error!("Only target_os = \"none\" is supported.");

#[cfg(not(target_endian = "little"))]
compile_error!("Only target_endian = \"little\" is supported.");

pub mod exceptions;
pub mod start;

pub use start::*;
