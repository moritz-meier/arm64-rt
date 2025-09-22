#![no_std]
#![no_main]

use core::cell::RefCell;
use core::fmt::Write as FmtWrite;
use core::panic::PanicInfo;
use core::u32;
use core::u64;

use arm64::cache::*;
use arm64::critical_section::*;
use arm64::mmu::*;
use arm64::pmu::PMU;
use arm64::psci::*;
use arm64::smccc::*;
use arm64::*;

use embedded_hal_nb::serial::Write;

mod excps;
mod plat;

use excps::*;
use plat::*;

static L0TABLE: Mutex<RefCell<TranslationTable<Level0>>> =
    Mutex::new(RefCell::new(TranslationTable::DEFAULT));

static L1TABLE: Mutex<RefCell<TranslationTable<Level1>>> =
    Mutex::new(RefCell::new(TranslationTable::DEFAULT));

const DEVICE_ATTRS: BlockAttrs = BlockAttrs::DEFAULT
    .with_mem_type(MemoryTyp::Device_nGnRnE)
    .with_shareability(Shareability::Non)
    .with_access(Access::PrivReadWrite)
    .with_security(SecurityDomain::NonSecure);

const NORMAL_ATTRS: BlockAttrs = BlockAttrs::DEFAULT
    .with_mem_type(MemoryTyp::Normal_Cacheable)
    .with_shareability(Shareability::Inner)
    .with_access(Access::PrivReadWrite)
    .with_security(SecurityDomain::NonSecure);

#[entry(exceptions = Excps)]
fn main(info: EntryInfo) -> ! {
    critical_section::with(|cs| {
        let mut l0 = L0TABLE.borrow_ref_mut(cs);
        let mut l1 = L1TABLE.borrow_ref_mut(cs);

        match () {
            #[cfg(feature = "qemu")]
            () => {
                l0.map_table(0x0000_0000, l1.base_addr() as u64, TableAttrs::DEFAULT);
                l1.map_block(0x0000_0000, 0x0000_0000, DEVICE_ATTRS);
                l1.map_block(0x4000_0000, 0x4000_0000, NORMAL_ATTRS);
            }

            #[cfg(feature = "kr260")]
            () => {
                l0.map_table(0x0000_0000, l1.base_addr() as u64, TableAttrs::DEFAULT);
                l1.map_block(0x0000_0000, 0x0000_0000, NORMAL_ATTRS);
                l1.map_block(0x4000_0000, 0x4000_0000, NORMAL_ATTRS);
                l1.map_block(0xC000_0000, 0xC000_0000, DEVICE_ATTRS);
            }
        }

        MMU::enable_el2(l0.base_addr() as u64);

        ICache::enable();
        DCache::enable();
    });

    DCache::op_all(CacheOp::CleanInvalidate);

    critical_section::with(|cs| {
        UART_DRIVER.borrow_ref_mut(cs).init();
    });

    UartWriter
        .write_fmt(format_args!("Hello World! cpu_idx = {}\n", info.cpu_idx))
        .unwrap();

    PMU::enable();
    PMU::setup_counter(0, pmu::Event::CPU_CYCLES);
    PMU::setup_counter(1, pmu::Event::INST_RETIRED);
    PMU::setup_counter(2, pmu::Event::L1D_CACHE);
    PMU::setup_counter(3, pmu::Event::L2D_CACHE);

    // Psci::cpu_on_64::<Smccc<SMC>>(
    //     1,
    //     (start::<SecEntryImpl, Excps> as *const fn() -> !) as u64,
    //     0,
    // )
    // .unwrap();

    loop {
        PMU::reset();
        PMU::start();

        let x = 100;

        unsafe {
            core::arch::asm!(
                "2:",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "sub {x}, {x}, #1",
                "cbnz {x}, 2b",
                x = in(reg) x,
            )
        };

        PMU::stop();

        UartWriter
            .write_fmt(format_args!(
                "cycle_count =  {:?}   cpu_cycles = {:?}    insts = {:?}    l1 = {:?}   l2 = {:?}\n",
                PMU::get_cycle_counter(),
                PMU::get_counter(0),
                PMU::get_counter(1),
                PMU::get_counter(2),
                PMU::get_counter(3)
            ))
            .unwrap();
    }
}

struct SecEntryImpl;

impl Entry for SecEntryImpl {
    unsafe extern "C" fn entry(info: EntryInfo) -> ! {
        main2(info)
    }
}

fn main2(info: EntryInfo) -> ! {
    critical_section::with(|cs| {
        let l0 = L0TABLE.borrow_ref_mut(cs);
        MMU::enable_el2(l0.base_addr() as u64);

        ICache::enable();
        DCache::enable();
    });

    DCache::op_all(CacheOp::CleanInvalidate);

    UartWriter
        .write_fmt(format_args!("Hello World! cpu_idx = {}\n", info.cpu_idx))
        .unwrap();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Psci::system_reset::<Smccc<SMC>>().unwrap();
    UartWriter
        .write_fmt(format_args!("{:?}\n", info.message()))
        .unwrap();

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
