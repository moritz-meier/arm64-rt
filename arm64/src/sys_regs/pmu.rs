#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use bitbybit::bitenum;

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
    pub PMCCNTR_EL0(
        "PMCCNTR_EL0", u64, rw
    ) {
        #[bits(0..=63, r)]
        CCNT: u64,
    }
}

system_register! {
    pub PMCEID0_EL0(
        "PMCEID0_EL0", u64, r
    ) {
        #[bit(0, r)]
        ID: [bool; 32]
    }
}

system_register! {
    pub PMCEID1_EL0(
        "PMCEID1_EL0", u64, r
    ) {
        #[bit(0, r)]
        ID: [bool; 32]
    }
}

// system_register! {
//     pub PMCNTENCLR_EL0(
//         "PMCNTENCLR_EL0", u64, rw
//     ) {

//         #[bit(31, rw)]
//         C: bool,

//         #[bit(0, rw)]
//         P: [bool; 31]
//     }
// }

system_register! {
    pub PMCNTENSET_EL0(
        "PMCNTENSET_EL0", u64, rw
    ) {

        #[bit(31, rw)]
        C: bool,

        #[bit(0, rw)]
        P: [bool; 31]
    }
}

system_register! {
    pub PMCR_EL0(
        "PMCR_EL0", u64, rw
    ) {
        #[bits(11..=15, r)]
        N: u5,

        #[bit(6, rw)]
        LC: bool,

        #[bit(3, rw)]
        D: bool,

        #[bit(2, rw)]
        C: bool,

        #[bit(1, rw)]
        P: bool,

        #[bit(0, rw)]
        E: bool,
    }
}

system_register! {
    pub PMOVSCLR_EL0(
        "PMOVSCLR_EL0", u64, rw
    ) {

        #[bit(31, rw)]
        C: bool,

        #[bit(0, rw)]
        P: [bool; 31]
    }
}

macro_rules! PMEVCNTR_n_EL0 {
    ($n:literal) => {
        pastey::paste! {
            system_register! {
                pub [<PMEVCNTR $n _EL0>] (
                    stringify!([<PMEVCNTR $n _EL0>]), u64, rw
                ) {
                    #[bits(0..=31, r)]
                    EVCNT: u32,
                }
            }
        }
    };
}

PMEVCNTR_n_EL0!(0);
PMEVCNTR_n_EL0!(1);
PMEVCNTR_n_EL0!(2);
PMEVCNTR_n_EL0!(3);

PMEVCNTR_n_EL0!(4);
PMEVCNTR_n_EL0!(5);
PMEVCNTR_n_EL0!(6);
PMEVCNTR_n_EL0!(7);

PMEVCNTR_n_EL0!(8);
PMEVCNTR_n_EL0!(9);
PMEVCNTR_n_EL0!(10);
PMEVCNTR_n_EL0!(11);

PMEVCNTR_n_EL0!(12);
PMEVCNTR_n_EL0!(13);
PMEVCNTR_n_EL0!(14);
PMEVCNTR_n_EL0!(15);

PMEVCNTR_n_EL0!(16);
PMEVCNTR_n_EL0!(17);
PMEVCNTR_n_EL0!(18);
PMEVCNTR_n_EL0!(19);

PMEVCNTR_n_EL0!(20);
PMEVCNTR_n_EL0!(21);
PMEVCNTR_n_EL0!(22);
PMEVCNTR_n_EL0!(23);

PMEVCNTR_n_EL0!(24);
PMEVCNTR_n_EL0!(25);
PMEVCNTR_n_EL0!(26);
PMEVCNTR_n_EL0!(27);

PMEVCNTR_n_EL0!(28);
PMEVCNTR_n_EL0!(29);
PMEVCNTR_n_EL0!(30);

macro_rules! PMEVTYPER_n_EL0 {
    ($n:literal) => {
        pastey::paste! {
            system_register! {
                pub [<PMEVTYPER $n _EL0>] (
                    stringify!([<PMEVTYPER $n _EL0>]), u64, rw
                ) {
                    #[bit(31, rw)]
                    P: bool,

                    #[bit(30, rw)]
                    U: bool,

                    #[bit(39, rw)]
                    NSK: bool,

                    #[bit(28, rw)]
                    NSU: bool,

                    #[bit(27, rw)]
                    NSH: bool,

                    #[bit(26, rw)]
                    M: bool,

                    #[bit(24, rw)]
                    SH: bool,

                    #[bits(0..=9, rw)]
                    EVT_COUNT: u10,
                }
            }
        }
    };
}

PMEVTYPER_n_EL0!(0);
PMEVTYPER_n_EL0!(1);
PMEVTYPER_n_EL0!(2);
PMEVTYPER_n_EL0!(3);

PMEVTYPER_n_EL0!(4);
PMEVTYPER_n_EL0!(5);
PMEVTYPER_n_EL0!(6);
PMEVTYPER_n_EL0!(7);

PMEVTYPER_n_EL0!(8);
PMEVTYPER_n_EL0!(9);
PMEVTYPER_n_EL0!(10);
PMEVTYPER_n_EL0!(11);

PMEVTYPER_n_EL0!(12);
PMEVTYPER_n_EL0!(13);
PMEVTYPER_n_EL0!(14);
PMEVTYPER_n_EL0!(15);

PMEVTYPER_n_EL0!(16);
PMEVTYPER_n_EL0!(17);
PMEVTYPER_n_EL0!(18);
PMEVTYPER_n_EL0!(19);

PMEVTYPER_n_EL0!(20);
PMEVTYPER_n_EL0!(21);
PMEVTYPER_n_EL0!(22);
PMEVTYPER_n_EL0!(23);

PMEVTYPER_n_EL0!(24);
PMEVTYPER_n_EL0!(25);
PMEVTYPER_n_EL0!(26);
PMEVTYPER_n_EL0!(27);

PMEVTYPER_n_EL0!(28);
PMEVTYPER_n_EL0!(29);
PMEVTYPER_n_EL0!(30);
