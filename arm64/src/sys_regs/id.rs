#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

system_register! {
    pub ID_AA64MMFR2_EL1(
        "ID_AA64MMFR2_EL1", u64, r
    ) {
        #[bits(20..=23, r)]
        CCIDX: u4
    }
}
