#![no_std]
#![feature(fn_align)]
#![feature(ptr_mask)]
#![feature(stdarch_arm_barrier)]

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

mod asm;
mod regs;

pub use start::*;
