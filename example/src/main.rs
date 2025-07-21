#![no_std]
#![no_main]

use core::fmt::Write as FmtWrite;
use core::panic::PanicInfo;

use arm64::cache::{DCache, ICache};
use arm64::psci::Psci;
use arm64::{EntryInfo, critical_section, entry};
use arm64::{smccc::*, start};

use embedded_hal_nb::serial::Write;

mod excps;
mod plat;

use excps::*;
use plat::*;

#[entry(exceptions = Excps)]
unsafe fn main(info: EntryInfo) -> ! {
    ICache::enable();
    DCache::enable();

    if info.cpu_idx == 0 {
        critical_section::with(|cs| {
            UART_DRIVER.borrow_ref_mut(cs).init();
        })
    }

    UartWriter
        .write_fmt(format_args!("Hello World! cpu_idx = {}", info.cpu_idx))
        .unwrap();

    Psci::cpu_on_64::<Smccc<SMC>>(
        (info.cpu_idx + 1) as u64,
        (start::<EntryImpl, Excps> as *const fn() -> !) as u64,
        0,
    )
    .unwrap();

    loop {
        unsafe { core::arch::asm!("nop") };
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    Psci::system_reset::<Smccc<SMC>>().unwrap();
    loop {}
}

struct UartWriter;

impl FmtWrite for UartWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        critical_section::with(|cs| {
            let mut driver = UART_DRIVER.borrow_ref_mut(cs);
            for c in s.chars() {
                driver.write(c as u8).unwrap()
            }
        });

        Ok(())
    }
}
