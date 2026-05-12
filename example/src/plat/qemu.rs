use core::cell::RefCell;

use spin::Mutex;

pub use sel4_pl011_driver as uart;

pub static UART_DRIVER: Mutex<RefCell<uart::Driver>> = Mutex::new(RefCell::new(unsafe {
    uart::Driver::new_uninit(0x09000000 as *mut _)
}));
