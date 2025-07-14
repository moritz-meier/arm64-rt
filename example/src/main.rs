#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt::Write as FmtWrite;
use core::panic::PanicInfo;

use arm64::cache::{DCache, ICache};
use arm64::{EntryInfo, entry, exceptions::*};

use embedded_hal_nb::serial::Write;

mod plat;

use plat::*;

struct ExcpsImpl;
impl Exceptions<ELx_SP_EL0> for ExcpsImpl {}
impl Exceptions<ELx_SP_ELx> for ExcpsImpl {
    fn sync_excp(_frame: &mut ExceptionFrame) {
        loop {}
    }

    fn serror(_frame: &mut ExceptionFrame) {
        loop {}
    }
}
impl Exceptions<ELy_AARCH64> for ExcpsImpl {}
impl Exceptions<ELy_AARCH32> for ExcpsImpl {}

#[entry(exceptions = ExcpsImpl)]
unsafe fn main(info: EntryInfo) -> ! {
    ICache::enable();
    DCache::enable();

    if info.cpu_idx != 0 {
        loop {
            unsafe { asm!("wfe") }
        }
    }

    UART_DRIVER.lock().init();

    UartWriter
        .write_fmt(format_args!("Hello World! cpu_idx = {}", info.cpu_idx))
        .unwrap();

    loop {
        unsafe { core::arch::asm!("nop") };
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

struct UartWriter;

impl FmtWrite for UartWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut driver = UART_DRIVER.lock();
        for c in s.chars() {
            driver.write(c as u8).unwrap()
        }

        Ok(())
    }
}
