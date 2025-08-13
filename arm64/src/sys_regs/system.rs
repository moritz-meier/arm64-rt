#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

system_register! {
    pub SCTLR_EL3(
        "SCTLR_EL3", u64, rw,
        res1 = (1 << 29) | (1 << 28) | (1 << 23) | (1 << 22) | (1 << 18) | (1 << 16) | (1 << 11) | (1 << 5) | (1 << 4)
    );
}

system_register! {
    pub SCTLR_EL2(
        "SCTLR_EL2", u64, rw,
        res1 = (1 << 29) | (1 << 28) | (1 << 23) | (1 << 22) | (1 << 18) | (1 << 16) | (1 << 11) | (1 << 5) | (1 << 4)
    );
}

system_register! {
    pub SCTLR_EL1(
        "SCTLR_EL1", u64, rw,
        res1 = (1 << 29) | (1 << 28) | (1 << 23) | (1 << 22) | (1 << 20) | (1 << 11)
    );
}

system_register! {
    pub SCR_EL3(
        "SCR_EL3", u64, rw,
        res1 = (1 << 5) | (1 << 4)
    ) {
        #[bit(0, rw)]
        NS: bool,
        #[bit(1, rw)]
        IRQ: bool,
        #[bit(2, rw)]
        FIQ: bool,
        #[bit(3, rw)]
        EA: bool
    }
}

system_register! {
    pub HCR_EL2(
        "HCR_EL2", u64, rw
    ) {
        #[bit(3, rw)]
        FMO: bool,
        #[bit(4, rw)]
        IMO: bool,
        #[bit(5, rw)]
        AMO: bool
    }
}

system_register! {
    pub SPSR_EL3(
        "SPSR_EL3", u64, rw
    ) {
        #[bits(0..=4, rw)]
        M: Option<M>,
        #[bit(6, rw)]
        F: bool,
        #[bit(7, rw)]
        I: bool,
        #[bit(8, rw)]
        A: bool,
        #[bit(9, rw)]
        D: bool,
    }

    #[bitenum(u5, exhaustive = false)]
    enum M {
        AARCH64_EL0_SP_EL0 = 0b00000,
        AARCH64_EL1_SP_EL0 = 0b00100,
        AARCH64_EL1_SP_EL1 = 0b00101,
        AARCH64_EL2_SP_EL0 = 0b01000,
        AARCH64_EL2_SP_EL2 = 0b01001,
        AARCH64_EL3_SP_EL0 = 0b01100,
        AARCH64_EL3_SP_EL3 = 0b01101,
    }
}

system_register! {
    pub SPSR_EL2(
        "SPSR_EL2", u64, rw
    ) {
        #[bits(0..=4, rw)]
        M: Option<M>,
        #[bit(6, rw)]
        F: bool,
        #[bit(7, rw)]
        I: bool,
        #[bit(8, rw)]
        A: bool,
        #[bit(9, rw)]
        D: bool,
    }

    #[bitenum(u5, exhaustive = false)]
    enum M {
        AARCH64_EL0_SP_EL0 = 0b00000,
        AARCH64_EL1_SP_EL0 = 0b00100,
        AARCH64_EL1_SP_EL1 = 0b00101,
        AARCH64_EL2_SP_EL0 = 0b01000,
        AARCH64_EL2_SP_EL2 = 0b01001,
    }
}

system_register! {
    pub SPSR_EL1(
        "SPSR_EL1", u64, rw
    ) {
        #[bits(0..=4, rw)]
        M: Option<M>,
        #[bit(6, rw)]
        F: bool,
        #[bit(7, rw)]
        I: bool,
        #[bit(8, rw)]
        A: bool,
        #[bit(9, rw)]
        D: bool,
    }

    #[bitenum(u5, exhaustive = false)]
    enum M {
        AARCH64_EL0_SP_EL0 = 0b00000,
        AARCH64_EL1_SP_EL0 = 0b00100,
        AARCH64_EL1_SP_EL1 = 0b00101,
    }
}
