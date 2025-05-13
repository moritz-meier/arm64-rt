#![no_std]
#![no_main]

use core::panic::PanicInfo;

use armv8a::entry;

#[entry]
unsafe fn main() -> ! {
    loop {
        unsafe { core::arch::asm!("nop") };
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
