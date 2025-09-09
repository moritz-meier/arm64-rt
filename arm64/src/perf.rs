use crate::sys_regs::*;

pub struct PerfMonitor;

impl PerfMonitor {
    pub fn enable_cycle_counter() {
        PMCNTENCLR_EL0.modify(|x| x.with_C(true));
        PMCCFILTR_EL0.modify(|x| {
            x.with_M(true)
                .with_SH(true)
                .with_M(true)
                .with_NSH(true)
                .with_NSU(true)
                .with_NSK(true)
                .with_U(true)
                .with_P(true)
        });
        PMCR_EL0.modify(|x| x.with_E(true).with_C(true));
    }

    pub fn disable_cycle_counter() {
        PMCNTENCLR_EL0.modify(|x| x.with_C(true));
        PMCR_EL0.modify(|x| x.with_E(false));
    }

    pub fn start_cycle_counter() {
        PMCNTENSET_EL0.modify(|x| x.with_C(true));
    }

    pub fn stop_cycle_counter() {
        PMCNTENCLR_EL0.modify(|x| x.with_C(true));
    }

    pub fn reset_cycle_counter() {
        PMCR_EL0.modify(|x| x.with_C(true));
    }

    pub fn get_cycle_counter() -> u64 {
        PMCCNTR_EL0.read().CCNT()
    }
}
