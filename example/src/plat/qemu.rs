use spin::Mutex;

pub use sel4_pl011_driver as uart;

pub static UART_DRIVER: Mutex<uart::Driver> =
    Mutex::new(unsafe { uart::Driver::new_uninit(0x09000000 as *mut _) });
