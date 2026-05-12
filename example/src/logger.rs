use core::{cell::RefCell, convert::Infallible, fmt::Write};

use embedded_hal_nb::serial;
use log::Log;
use spin::Mutex;

use crate::spin_ext::*;

pub struct Logger<'a, DRIVER: Send> {
    driver: &'a Mutex<RefCell<DRIVER>>,
}

impl<'a, DRIVER> Logger<'a, DRIVER>
where
    DRIVER: Send,
{
    pub fn new(driver: &'a Mutex<RefCell<DRIVER>>) -> Self {
        Self { driver }
    }
}

impl<'a, DRIVER> Log for Logger<'a, DRIVER>
where
    DRIVER: Send + embedded_hal_nb::serial::Write<Error = Infallible>,
{
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        self.driver.lock_irq(|driver| {
            let driver = &mut *driver.borrow_mut() as &mut dyn serial::Write<_, Error = Infallible>;
            writeln!(driver, "[{}] {}", record.level(), record.args()).unwrap();
        });
    }

    fn flush(&self) {
        self.driver.lock_irq(|driver| {
            let mut driver = driver.borrow_mut();
            driver.flush().unwrap();
        });
    }
}
