#![no_std]
#![no_main]
#![feature(cfg_select)]

use core::cell::RefCell;
use core::panic::PanicInfo;

use log::*;
use spin::Mutex;

use arm64::cache::*;
use arm64::mmu::*;
use arm64::psci::*;
use arm64::smccc::*;
use arm64::*;

mod excps;
mod logger;
mod plat;
mod spin_ext;

use excps::*;
use logger::*;
use plat::*;
use spin::Once;
use spin_ext::*;

struct TranslationTables {
    l0: TranslationTable<Level0>,
    l1: TranslationTable<Level1>,
}

static TRANSLATION_TABLES: Mutex<RefCell<TranslationTables>> =
    Mutex::new(RefCell::new(TranslationTables {
        l0: TranslationTable::DEFAULT,
        l1: TranslationTable::DEFAULT,
    }));

// Default memory attributes for virtual memory blocks
// Device memory for Non-Cacheable MMIO access to peripherals
const DEVICE_ATTRS: BlockAttrs = BlockAttrs::DEFAULT
    .with_mem_type(MemoryTyp::Device_nGnRnE)
    .with_shareability(Shareability::Non)
    .with_access(Access::PrivReadWrite)
    .with_security(SecurityDomain::NonSecure);

// Cacheable memory for data + code
const NORMAL_ATTRS: BlockAttrs = BlockAttrs::DEFAULT
    .with_mem_type(MemoryTyp::Normal_Cacheable)
    .with_shareability(Shareability::Inner)
    .with_access(Access::PrivReadWrite)
    .with_security(SecurityDomain::NonSecure);

pub static LOGGER: Once<Logger<'static, plat::uart::Driver>> = Once::new();

#[entry(exceptions = Excps)]
fn main(info: EntryInfo) -> ! {
    // Lock mutex and disable interrupts
    TRANSLATION_TABLES.lock_irq(|tables| {
        let mut tables = tables.borrow_mut();

        cfg_select! {
            feature = "qemu" => {
                let l1_base_addr = tables.l1.base_addr();

                // Split first 512GB of virtual memory into 512 x 1GB blocks
                tables.l0.map_table(
                    0x0000_0000,
                    l1_base_addr as u64,
                    TableAttrs::DEFAULT,
                );

                // Map first 1GB of physical memory (QEMU MMIO devices) into virtual memory (simple unity map virtaddr = physaddr)
                tables.l1.map_block(0x0000_0000, 0x0000_0000, DEVICE_ATTRS);

                // Map first 1GB of physial RAM into virtual memory (unity map)
                tables.l1.map_block(0x4000_0000, 0x4000_0000, NORMAL_ATTRS);
            }

            feature = "kr260" => {
                let l1_base_addr = tables.l1.base_addr();
                 tables.l0.map_table(
                    0x0000_0000,
                    l1_base_addr as u64,
                    TableAttrs::DEFAULT,
                );

                // Map first (lower) 2GB of DDR RAM into virtual memory
                tables.l1.map_block(0x0000_0000, 0x0000_0000, NORMAL_ATTRS);
                tables.l1.map_block(0x4000_0000, 0x4000_0000, NORMAL_ATTRS);

                // Map 1GB of devices into virtual memory
                tables.l1.map_block(0xC000_0000, 0xC000_0000, DEVICE_ATTRS);
            }
        }

        // Enable MMU using the prepared translation tables
        MMU::enable_el2(tables.l0.base_addr() as u64);

        // Enables caches
        ICache::enable();
        DCache::enable();
    });

    // Init Uart driver
    UART_DRIVER.lock_irq(|uart| {
        let mut uart = uart.borrow_mut();
        uart.init();
    });

    // Setup logger to output logs via the uart driver
    {
        let logger = LOGGER.call_once(|| Logger::new(&UART_DRIVER));
        set_logger(logger).unwrap();
        set_max_level(log::LevelFilter::Info);
    }

    info!("Hello World! cpu_idx = {}", info.cpu_idx);

    // Start secondary core via PSCI syscall to ARM Trusted Firmware
    Psci::cpu_on_64::<Smccc<SMC>>(1, (_secondary_start as *const fn() -> !) as u64, 0).unwrap();

    loop {
        unsafe { core::arch::asm!("nop") };
    }
}

#[secondary_entry(exceptions = Excps)]
fn main2(info: EntryInfo) -> ! {
    // Enable virtual memory, using the same translation tables as the primary core
    TRANSLATION_TABLES.lock_irq(|tables| {
        let tables = tables.borrow();

        MMU::enable_el2(tables.l0.base_addr() as u64);

        ICache::enable();
        DCache::enable();
    });

    info!("Hello World! cpu_idx = {}", info.cpu_idx);

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("PANIC: {:?}", info);

    // Psci::system_reset::<Smccc<SMC>>().unwrap();

    loop {}
}
