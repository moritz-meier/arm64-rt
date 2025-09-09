#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

system_register! {
    pub PMCCNTR_EL0(
        "PMCCNTR_EL0", u64, r
    ) {
        #[bits(0..=63, r)]
        CCNT: u64
    }
}

system_register! {
    pub PMCR_EL0(
        "PMCR_EL0", u64, rw
    ) {
        #[bit(2, rw)]
        C: bool,

        #[bit(0, rw)]
        E: bool,
    }
}

system_register! {
    pub PMCCFILTR_EL0(
        "PMCCFILTR_EL0", u64, rw
    ) {
        #[bit(31, rw)]
        P: bool,

        #[bit(30, rw)]
        U: bool,

        #[bit(29, rw)]
        NSK: bool,

        #[bit(28, rw)]
        NSU: bool,

        #[bit(27, rw)]
        NSH: bool,

        #[bit(26, rw)]
        M: bool,

        #[bit(24, rw)]
        SH: bool,
    }
}

system_register! {
    pub PMCNTENCLR_EL0(
        "PMCNTENCLR_EL0", u64, rw
    ) {
        #[bit(31, rw)]
        C: bool,
    }
}

system_register! {
    pub PMCNTENSET_EL0(
        "PMCNTENSET_EL0", u64, rw
    ) {
        #[bit(31, rw)]
        C: bool,
    }
}
