#[macro_export]
macro_rules! dmb {
    ($opt:literal) => {
        unsafe {
            asm!(::core::concat!("dmb ", $opt));
        }
    };
}

#[macro_export]
macro_rules! dsb {
    ($opt:literal) => {
        unsafe {
            asm!(::core::concat!("dsb ", $opt));
        }
    };
}

#[macro_export]
macro_rules! isb {
    ($opt:literal) => {
        unsafe {
            asm!(::core::concat!("isb ", $opt));
        }
    };
}
