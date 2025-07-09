#![no_std]
#![no_main]

use core::arch::asm;
use core::sync::atomic::AtomicBool;
use core::{cell::RefCell, fmt::Write, panic::PanicInfo, ptr::NonNull};

use arm64::cache::{DCache, ICache};
use spin::Mutex;

use arm_pl011_uart::{
    DataBits, LineConfig, PL011Registers, Parity, StopBits, Uart, UniqueMmioPointer,
};

use arm64::exceptions::{
    ELx_SP_EL0, ELx_SP_ELx, ELy_AARCH32, ELy_AARCH64, ExceptionFrame, Exceptions,
};
use arm64::{EntryInfo, entry};

const UART_ADDR: *mut PL011Registers = 0x9000000 as *mut PL011Registers;

static LOCK: AtomicBool = AtomicBool::new(true);
static UART: Mutex<RefCell<Option<Uart>>> = Mutex::new(RefCell::new(None));

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

    unsafe { init() };

    DCache::op_all(arm64::cache::CacheOp::CleanInvalidate);

    {
        let uart = UART.lock();
        uart.borrow_mut()
            .as_mut()
            .unwrap()
            .write_fmt(format_args!("Hello World! cpu_idx = {}\n", info.cpu_idx))
            .unwrap();
    }

    loop {
        unsafe { core::arch::asm!("nop") };
    }
}

unsafe fn init() {
    let uart_ptr = unsafe { UniqueMmioPointer::new(NonNull::new(UART_ADDR).unwrap()) };
    let mut uart = Uart::new(uart_ptr);

    let line_config = LineConfig {
        data_bits: DataBits::Bits8,
        parity: Parity::None,
        stop_bits: StopBits::One,
    };
    uart.enable(line_config, 115_200, 24_000_000).unwrap();

    uart.write_fmt(format_args!("init\n")).unwrap();

    {
        let x = UART.lock();
        x.borrow_mut().replace(uart);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
