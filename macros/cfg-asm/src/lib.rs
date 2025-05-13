#![no_std]

#[doc(hidden)]
#[macro_export]

macro_rules! cfg_asm {
    (@inner, [$($x:tt)*], [$($opts:tt)*], ) => {
        asm!($($x)* $($opts)*)
    };

    (@inner, [$($x:tt)*], [$($opts:tt)*], #[cfg($meta:meta)] $asm:literal, $($rest:tt)*) => {
        #[cfg($meta)]
        cfg_asm!(@inner, [$($x)* $asm,], [$($opts)*], $($rest)*);
        #[cfg(not($meta))]
        cfg_asm!(@inner, [$($x)*], [$($opts)*], $($rest)*)
    };

    (@inner, [$($x:tt)*], [$($opts:tt)*], $asm:literal, $($rest:tt)*) => {
        cfg_asm!(@inner, [$($x)* $asm,], [$($opts)*], $($rest)*)
    };

    ({$($asms:tt)*}, $($opts:tt)*) => {
        cfg_asm!(@inner, [], [$($opts)*], $($asms)*)
    };
}

#[macro_export]
macro_rules! cfg_global_asm {
    {@inner, [$($x:tt)*], } => {
        global_asm!{$($x)*}
    };

    (@inner, [$($x:tt)*], #[cfg($meta:meta)] $asm:literal, $($rest:tt)*) => {
        #[cfg($meta)]
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
        #[cfg(not($meta))]
        cfg_global_asm!{@inner, [$($x)*], $($rest)*}
    };

    {@inner, [$($x:tt)*], $asm:literal, $($rest:tt)*} => {
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
    };

    {$($asms:tt)*} => {
        cfg_global_asm!{@inner, [], $($asms)*}
    };
}

#[macro_export]
macro_rules! cfg_naked_asm {
    (@inner, [$($x:tt)*], [$($opts:tt)*], ) => {
        naked_asm!($($x)* $($opts)*)
    };

    (@inner, [$($x:tt)*], [$($opts:tt)*], #[cfg($meta:meta)] $asm:literal, $($rest:tt)*) => {
        #[cfg($meta)]
        cfg_naked_asm!(@inner, [$($x)* $asm,], [$($opts)*], $($rest)*);
        #[cfg(not($meta))]
        cfg_naked_asm!(@inner, [$($x)*], [$($opts)*], $($rest)*)
    };

    (@inner, [$($x:tt)*], [$($opts:tt)*], $asm:literal, $($rest:tt)*) => {
        cfg_naked_asm!(@inner, [$($x)* $asm,], [$($opts)*], $($rest)*)
    };

    ({$($asms:tt)*}, $($opts:tt)*) => {
        cfg_naked_asm!(@inner, [], [$($opts)*], $($asms)*)
    };
}
