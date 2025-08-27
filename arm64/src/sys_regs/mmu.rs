#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

system_register! {
    pub TTBR0_EL3(
        "TTBR0_EL3", u64, rw
    ) {
        #[bits(48..=63, rw)]
        ASID16: u16,

        #[bits(48..=55, rw)]
        ASID8: u8,

        #[bits(1..=47, rw)]
        BADDR: u47,
    }
}

system_register! {
    pub TTBR0_EL2(
        "TTBR0_EL2", u64, rw
    ) {
        #[bits(48..=63, rw)]
        ASID16: u16,

        #[bits(48..=55, rw)]
        ASID8: u8,

        #[bits(1..=47, rw)]
        BADDR: u47,
    }
}

system_register! {
    pub TTBR0_EL1(
        "TTBR0_EL1", u64, rw
    ) {
        #[bits(48..=63, rw)]
        ASID16: u16,

        #[bits(48..=55, rw)]
        ASID8: u8,

        #[bits(1..=47, rw)]
        BADDR: u47,
    }
}

system_register! {
    pub TCR_EL3(
        "TCR_EL3", u64, rw, res1 = (1 << 31)
    ) {
        #[bit(20, rw)]
        TBI: bool,

        #[bits(16..=18, rw)]
        PS: u3,

        #[bits(14..=15, rw)]
        TG0: u2,

        #[bits(12..=13, rw)]
        SH0: u2,

        #[bits(10..=11, rw)]
        ORGN0: u2,

        #[bits(8..=9, rw)]
        IRGN0: u2,

        #[bits(0..=5, rw)]
        T0SZ: u6,
    }
}

system_register! {
    pub TCR_EL2(
        "TCR_EL2", u64, rw, res1 = (1 << 31)
    ) {
        #[bit(20, rw)]
        TBI: bool,

        #[bits(16..=18, rw)]
        PS: u3,

        #[bits(14..=15, rw)]
        TG0: u2,

        #[bits(12..=13, rw)]
        SH0: u2,

        #[bits(10..=11, rw)]
        ORGN0: u2,

        #[bits(8..=9, rw)]
        IRGN0: u2,

        #[bits(0..=5, rw)]
        T0SZ: u6,
    }
}

system_register! {
    pub TCR_EL1(
        "TCR_EL1", u64, rw
    ) {
        #[bit(38, rw)]
        TBI1: bool,

        #[bit(37, rw)]
        TBI0: bool,

        #[bit(36, rw)]
        AS: bool,

        #[bits(32..=34, rw)]
        IPS: u3,

        #[bits(30..=31, rw)]
        TG1: u2,

        #[bits(28..=29, rw)]
        SH1: u2,

        #[bits(26..=27, rw)]
        ORGN1: u2,

        #[bits(24..=25, rw)]
        IRGN1: u2,

        #[bit(23, rw)]
        EPD1: bool,

        #[bit(22, rw)]
        A1: bool,

        #[bits(16..=21, rw)]
        T1SZ: u6,

        #[bits(14..=15, rw)]
        TG0: u2,

        #[bits(12..=13, rw)]
        SH0: u2,

        #[bits(10..=11, rw)]
        ORGN0: u2,

        #[bits(8..=9, rw)]
        IRGN0: u2,

        #[bit(7, rw)]
        EPD0: bool,

        #[bits(0..=5, rw)]
        T0SZ: u6,
    }
}

system_register! {
    pub MAIR_EL3(
        "MAIR_EL3", u64, rw
    ) {
        #[bits(0..=7, rw)]
        ATTR7: u8,

        #[bits(0..=7, rw)]
        ATTR6: u8,

        #[bits(0..=7, rw)]
        ATTR5: u8,

        #[bits(0..=7, rw)]
        ATTR4: u8,

        #[bits(0..=7, rw)]
        ATTR3: u8,

        #[bits(0..=7, rw)]
        ATTR2: u8,

        #[bits(0..=7, rw)]
        ATTR1: u8,

        #[bits(0..=7, rw)]
        ATTR0: u8,
    }
}

system_register! {
    pub MAIR_EL2(
        "MAIR_EL2", u64, rw
    ) {
        #[bits(0..=7, rw)]
        ATTR7: u8,

        #[bits(0..=7, rw)]
        ATTR6: u8,

        #[bits(0..=7, rw)]
        ATTR5: u8,

        #[bits(0..=7, rw)]
        ATTR4: u8,

        #[bits(0..=7, rw)]
        ATTR3: u8,

        #[bits(0..=7, rw)]
        ATTR2: u8,

        #[bits(0..=7, rw)]
        ATTR1: u8,

        #[bits(0..=7, rw)]
        ATTR0: u8,
    }
}

system_register! {
    pub MAIR_EL1(
        "MAIR_EL1", u64, rw
    ) {
        #[bits(0..=7, rw)]
        ATTR7: u8,

        #[bits(0..=7, rw)]
        ATTR6: u8,

        #[bits(0..=7, rw)]
        ATTR5: u8,

        #[bits(0..=7, rw)]
        ATTR4: u8,

        #[bits(0..=7, rw)]
        ATTR3: u8,

        #[bits(0..=7, rw)]
        ATTR2: u8,

        #[bits(0..=7, rw)]
        ATTR1: u8,

        #[bits(0..=7, rw)]
        ATTR0: u8,
    }
}
