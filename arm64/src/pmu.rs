use arbitrary_int::*;

use crate::sys_regs::*;

pub struct PMU;

impl PMU {
    pub fn enable() {
        PMCCFILTR_EL0.modify(|pmccfiltr_el0| {
            pmccfiltr_el0
                .with_P(false)
                .with_U(false)
                .with_NSK(false)
                .with_NSU(false)
                .with_NSH(true)
                .with_M(false)
                .with_SH(false)
        });

        PMCNTENSET_EL0.write(PMCNTENSET_EL0::new_with_raw_value(0));
        PMOVSCLR_EL0.write(PMOVSCLR_EL0::new_with_raw_value(0));

        PMCR_EL0.modify(|pmcr_el0| {
            pmcr_el0
                .with_E(true)
                .with_P(true)
                .with_C(true)
                .with_D(false)
                .with_LC(true)
        });
    }

    pub fn disable() {
        PMCR_EL0.modify(|pmcr_el0| pmcr_el0.with_E(false));
        PMCNTENSET_EL0.write(PMCNTENSET_EL0::new_with_raw_value(0));
    }

    pub fn setup_counter(n: usize, event: Event) {
        macro_rules! match_n {
            ($n:literal) => {
                pastey::paste! {
                    [<PMEVTYPER $n _EL0>].modify(|x|
                        x.with_EVT_COUNT(u10::from_u16(event as u16))
                        .with_P(false)
                        .with_U(false)
                        .with_NSK(false)
                        .with_NSU(false)
                        .with_NSH(true)
                        .with_M(false)
                        .with_SH(false))
                }
            };
        }

        match n {
            0 => match_n!(0),
            1 => match_n!(1),
            2 => match_n!(2),
            3 => match_n!(3),
            4 => match_n!(4),
            5 => match_n!(5),
            6 => match_n!(6),
            7 => match_n!(7),
            8 => match_n!(8),
            9 => match_n!(9),
            10 => match_n!(10),
            11 => match_n!(11),
            12 => match_n!(12),
            13 => match_n!(13),
            14 => match_n!(14),
            15 => match_n!(15),
            16 => match_n!(16),
            17 => match_n!(17),
            18 => match_n!(18),
            19 => match_n!(19),
            20 => match_n!(20),
            21 => match_n!(21),
            22 => match_n!(22),
            23 => match_n!(23),
            24 => match_n!(24),
            25 => match_n!(25),
            26 => match_n!(26),
            27 => match_n!(27),
            28 => match_n!(28),
            29 => match_n!(29),
            30 => match_n!(30),
            _ => {}
        }
    }

    pub fn start() {
        PMCNTENSET_EL0.write(PMCNTENSET_EL0::new_with_raw_value(u32::MAX as u64));
    }

    pub fn stop() {
        PMCNTENSET_EL0.write(PMCNTENSET_EL0::new_with_raw_value(0));
    }

    pub fn reset() {
        PMCR_EL0.modify(|pmcr_el0| pmcr_el0.with_C(true).with_P(true));
        PMOVSCLR_EL0.write(PMOVSCLR_EL0::new_with_raw_value(0));
    }

    pub fn get_cycle_counter() -> CounterValue<u64> {
        if !PMOVSCLR_EL0.read().C() {
            CounterValue::Ok(PMCCNTR_EL0.read().CCNT())
        } else {
            CounterValue::Overflowed(PMCCNTR_EL0.read().CCNT())
        }
    }

    pub fn get_counter(n: usize) -> CounterValue<u32> {
        macro_rules! match_n {
            ($n:literal) => {
                pastey::paste! {
                    if !PMOVSCLR_EL0.read().P($n) {
                        CounterValue::Ok([<PMEVCNTR $n _EL0>].read().EVCNT())
                    } else {
                        CounterValue::Overflowed([<PMEVCNTR $n _EL0>].read().EVCNT())
                    }
                }
            };
        }

        match n {
            0 => match_n!(0),
            1 => match_n!(1),
            2 => match_n!(2),
            3 => match_n!(3),
            4 => match_n!(4),
            5 => match_n!(5),
            6 => match_n!(6),
            7 => match_n!(7),
            8 => match_n!(8),
            9 => match_n!(9),
            10 => match_n!(10),
            11 => match_n!(11),
            12 => match_n!(12),
            13 => match_n!(13),
            14 => match_n!(14),
            15 => match_n!(15),
            16 => match_n!(16),
            17 => match_n!(17),
            18 => match_n!(18),
            19 => match_n!(19),
            20 => match_n!(20),
            21 => match_n!(21),
            22 => match_n!(22),
            23 => match_n!(23),
            24 => match_n!(24),
            25 => match_n!(25),
            26 => match_n!(26),
            27 => match_n!(27),
            28 => match_n!(28),
            29 => match_n!(29),
            30 => match_n!(30),
            _ => {
                return CounterValue::Ok(0);
            }
        }
    }
}

#[cfg(feature = "cortex-a53")]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Event {
    L1D_CACHE_ALLOCATE = 0x1F,
    CHAIN = 0x1E,
    BUS_CYCLES = 0x1D,
    TTBR_WRITE_RETIRED = 0x1C,
    INST_SPEC = 0x1B,
    MEMORY_ERROR = 0x1A,
    BUS_ACCESS = 0x19,
    L2D_CACHE_WB = 0x18,
    L2D_CACHE_REFILL = 0x17,
    L2D_CACHE = 0x16,
    L1D_CACHE_WB = 0x15,
    L1I_CACHE = 0x14,
    MEM_ACCESS = 0x13,
    BR_PRED = 0x12,
    CPU_CYCLES = 0x11,
    BR_MIS_PRED = 0x10,
    UNALIGNED_LDST_RETIRED = 0x0F,
    BR_RETURN_RETIRED = 0x0E,
    BR_IMMED_RETIRED = 0x0D,
    PC_WRITE_RETIRED = 0x0C,
    CID_WRITE_RETIRED = 0x0B,
    EXC_RETURN = 0x0A,
    EXC_TAKEN = 0x09,
    INST_RETIRED = 0x08,
    ST_RETIRED = 0x07,
    LD_RETIRED = 0x06,
    L1D_TLB_REFILL = 0x05,
    L1D_CACHE = 0x04,
    L1D_CACHE_REFILL = 0x03,
    L1I_TLB_REFILL = 0x02,
    L1I_CACHE_REFILL = 0x01,
    SW_INCR = 0x00,
    FOO = 0x60,
}

#[derive(Copy, Clone, Debug)]
pub enum CounterValue<T> {
    Ok(T),
    Overflowed(T),
}
