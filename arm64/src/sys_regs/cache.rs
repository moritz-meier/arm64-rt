#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

system_register! {
    pub CSSELR_EL1(
        "CSSELR_EL1", u64, rw
    ) {
        #[bits(1..=3, rw)]
        LEVEL: u3,

        #[bit(0, rw)]
        InD: bool,
    }
}

system_register! {
    pub CCSIDR_EL1_CCIDX(
        "CCSIDR_EL1", u64, r
    ) {
        #[bits(32..=55, r)]
        NUM_SETS: u24,

        #[bits(3..=23, r)]
        ASSOCIATIVITY: u21,

        #[bits(0..=2, r)]
        LINE_SIZE: u3,
    }
}

system_register! {
    pub CCSIDR_EL1_NO_CCIDX(
        "CCSIDR_EL1", u64, r
    ) {
        #[bits(13..=27, r)]
        NUM_SETS: u15,

        #[bits(3..=12, r)]
        ASSOCIATIVITY: u10,

        #[bits(0..=2, r)]
        LINE_SIZE: u3,
    }
}

system_register! {
    pub CLIDR_EL1(
        "CLIDR_EL1", u64, r
    ) {
        #[bits(30..=32, r)]
        ICB: u3,

        #[bits(27..=29, r)]
        LOUU: u3,

        #[bits(24..=26, r)]
        LOC: u3,

        #[bits(21..=23, r)]
        LOUIS: u3,

        #[bits(0..=2, r)]
        CTYPE: [u3; 7]
    }
}
