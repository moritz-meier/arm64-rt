use arm64::sys_regs::*;

use spin::{Mutex, MutexGuard};

pub trait MutexIrqExt<T: ?Sized> {
    fn lock_irq<O>(&self, f: impl Fn(MutexGuard<'_, T>) -> O) -> O;
}

impl<T: ?Sized> MutexIrqExt<T> for Mutex<T> {
    fn lock_irq<O>(&self, f: impl Fn(MutexGuard<'_, T>) -> O) -> O {
        let daif = DAIF.read();
        DAIF.write(
            DAIF::ZERO
                .with_D(true)
                .with_A(true)
                .with_I(true)
                .with_F(true),
        );

        let res;
        {
            let lock = self.lock();
            res = f(lock);
        }

        DAIF.write(daif);

        res
    }
}
