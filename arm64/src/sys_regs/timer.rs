#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

system_register! {
    pub CNTP_CTL_EL0(
        "CNTP_CTL_EL0", u64, rw
    ) {
        #[bit(1, r)]
        ISTATUS: bool,

        #[bit(1, rw)]
        IMASK: bool,

        #[bit(0, rw)]
        ENABLE: bool,
    }
}

system_register! {
    pub CNTPCT_EL0(
        "CNTPCT_EL0", u64, r
    ) {
        #[bits(0..=63, r)]
        CNT: u64,
    }
}

system_register! {
    pub CNTVCT_EL0(
        "CNTVCT_EL0", u64, r
    ) {
        #[bits(0..=63, r)]
        CNT: u64,
    }
}

system_register! {
    pub CNTVOFF_EL2(
        "CNTVOFF_EL2", u64, rw
    ) {
        #[bits(0..=63, rw)]
        CNT: u64,
    }
}

system_register! {
    pub CNTFRQ_EL0(
        "CNTFRQ_EL0", u64, rw
    ) {
        #[bits(0..=63, rw)]
        FREQ: u64,
    }
}
