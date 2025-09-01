#![no_std]
#![no_main]

use core::cell::RefCell;
use core::fmt::Write as FmtWrite;
use core::panic::PanicInfo;

use arm64::cache::{DCache, ICache};
use arm64::critical_section::Mutex;
use arm64::mmu::*;
use arm64::psci::Psci;
use arm64::{EntryInfo, critical_section, entry};
use arm64::{smccc::*, start};

use embedded_hal_nb::serial::Write;

mod excps;
mod plat;

use excps::*;
use plat::*;

static L0TABLE: Mutex<RefCell<TranslationTable<Level0>>> =
    Mutex::new(RefCell::new(TranslationTable::DEFAULT));

static L1TABLE: Mutex<RefCell<TranslationTable<Level1>>> =
    Mutex::new(RefCell::new(TranslationTable::DEFAULT));

#[entry(exceptions = Excps)]
unsafe fn main(info: EntryInfo) -> ! {
    critical_section::with(|cs| {
        let mut l0 = L0TABLE.borrow_ref_mut(cs);
        let mut l1 = L1TABLE.borrow_ref_mut(cs);

        l0.map_table(0x4000_0000, l1.base_addr() as u64, TableAttrs::NON_SECURE);
        l1.map_block(0x0000_0000, 0x0000_0000, BlockAttrs::DEFAULT);
        l1.map_block(0x4000_0000, 0x4000_0000, BlockAttrs::DEFAULT);

        MMU::enable_el1(l0.base_addr() as u64);

        ICache::enable();
        DCache::enable();
    });

    if info.cpu_idx == 0 {
        critical_section::with(|cs| {
            UART_DRIVER.borrow_ref_mut(cs).init();
        })
    }

    UartWriter
        .write_fmt(format_args!("Hello World! cpu_idx = {}", info.cpu_idx))
        .unwrap();

    // Psci::cpu_on_64::<Smccc<SMC>>(
    //     (info.cpu_idx + 1) as u64,
    //     (start::<EntryImpl, Excps> as *const fn() -> !) as u64,
    //     0,
    // )
    // .unwrap();

    loop {
        unsafe { core::arch::asm!("nop") };
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Psci::system_reset::<Smccc<SMC>>().unwrap();
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
