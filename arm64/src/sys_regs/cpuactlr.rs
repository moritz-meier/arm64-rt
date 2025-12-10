#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "cortex-a53")]
system_register! {
    pub CPUACTLR_EL1(
        "CPUACTLR_EL1", u64, rw
    ) {
        #[bit(44, rw)]
        ENDCCASCI: bool,

        #[bit(30, rw)]
        FPDIDIS: bool,

        #[bit(29, rw)]
        DIDIS: bool,

        #[bits(27..=28, rw)]
        RADIS: u2,

        #[bits(25..=26, rw)]
        L1RADIS: u2,

        #[bit(24, rw)]
        DTAH: bool,

        #[bit(23, rw)]
        STBPFRS: bool,

        #[bit(22, rw)]
        STBPFDIS: bool,

        #[bit(21, rw)]
        IFUTHDIS: bool,

        #[bits(19..=20, rw)]
        NPFSTRM: u2,

        #[bit(18, rw)]
        DSTDIS: bool,

        #[bit(17, rw)]
        STRIDE: bool,

        #[bits(13..=15, rw)]
        L1PCTL: u3,

        #[bit(10, rw)]
        DODMBS: bool,

        #[bit(6, rw)]
        L1DEIEN: bool,
    }
}
