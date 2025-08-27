#[macro_export]
macro_rules! dmb {
    ($opt:literal) => {
        unsafe {
            core::arch::asm!(::core::concat!("dmb ", $opt));
        }
    };
}

#[macro_export]
macro_rules! dsb {
    ($opt:literal) => {
        unsafe {
            core::arch::asm!(::core::concat!("dsb ", $opt));
        }
    };
}

#[macro_export]
macro_rules! isb {
    ($opt:literal) => {
        unsafe {
            core::arch::asm!(::core::concat!("isb ", $opt));
        }
    };
}
