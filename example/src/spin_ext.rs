use core::arch::asm;

use spin::{Mutex, MutexGuard};

pub trait MutexIrqExt<T: ?Sized> {
    fn lock_irq<O>(&self, f: impl Fn(MutexGuard<'_, T>) -> O) -> O;
}

impl<T: ?Sized> MutexIrqExt<T> for Mutex<T> {
    fn lock_irq<O>(&self, f: impl Fn(MutexGuard<'_, T>) -> O) -> O {
        let daif: u64;
        unsafe { asm!("mrs {daif}, DAIF", "msr DAIFSet, #0xf", daif = lateout(reg) daif) }

        let res;
        {
            let lock = self.lock();
            res = f(lock);
        }

        unsafe { asm!("msr DAIF, {daif}", daif = in(reg) daif) }

        res
    }
}
