#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

system_register! {
    pub ID_AA64MMFR0_EL1(
        "ID_AA64MMFR0_EL1", u64, r
    ) {
        #[bits(4..=7, r)]
        ASIDBITS: u4,

        #[bits(0..=3, r)]
        PARANGE: u4
    }
}

system_register! {
    pub ID_AA64MMFR2_EL1(
        "ID_AA64MMFR2_EL1", u64, r
    ) {
        #[bits(20..=23, r)]
        CCIDX: u4
    }
}

system_register! {
    pub ID_AA64DFR0_EL1(
        "ID_AA64DFR0_EL1", u64, r
    ) {
        #[bits(16..=19, r)]
        PMSS: u4,

        #[bits(8..=11, r)]
        PMU_VER: u4,

        #[bits(4..=7, r)]
        TRACE_VER: u4,

        #[bits(0..=3, r)]
        DEBUG_VER: u4
    }
}

system_register! {
    pub ID_AA64DFR1_EL1(
        "ID_AA64DFR1_EL1", u64, r
    ) {
        #[bits(36..=39, r)]
        PMICNTR: u4,
    }
}
