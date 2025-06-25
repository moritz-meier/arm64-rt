#![no_std]
#![no_main]

use core::panic::PanicInfo;

use arm64::{EntryInfo, entry};

#[entry]
unsafe fn main(_info: EntryInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("nop") };
    }
}

fn fib(n: usize) -> usize {
    match n {
        1 | 2 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
