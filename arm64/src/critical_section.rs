use core::{arch::asm, ptr::addr_of_mut};

pub use critical_section::*;

struct CriticalSectionImpl;
set_impl!(CriticalSectionImpl);

static mut GLOBAL_LOCK: u64 = 0;

unsafe impl critical_section::Impl for CriticalSectionImpl {
    unsafe fn acquire() -> RawRestoreState {
        unsafe {
            asm!(
                "sevl",
                "2:",
                "wfe",
                "3:",
                "ldaxr {reg0}, [{lock_addr}]",
                "cbnz {reg0}, 2b",
                "stxr {reg1:w}, {reg2:x}, [{lock_addr}]",
                "cbnz {reg1}, 3b",
                lock_addr = in(reg) addr_of_mut!(GLOBAL_LOCK),
                reg0 = out(reg) _,
                reg1 = out(reg) _,
                reg2 = in(reg) 1,
            );
        }
    }

    unsafe fn release(_state: RawRestoreState) {
        unsafe {
            asm!(
                "stlr xzr, [{lock_addr}]", // Store operation generates an event to all cores waiting in WFE
                lock_addr = in(reg) addr_of_mut!(GLOBAL_LOCK),
            );
        }
    }
}
