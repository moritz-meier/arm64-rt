#[macro_export]
macro_rules! bitmask {
    (bit: $bit:literal) => {
        bitmask!(msb: $bit, lsb: $bit)
    };

    (msb: $msb:literal, lsb: $lsb:literal) => {
        ((1 << ($msb - $lsb)) + ((1 << ($msb - $lsb)) - 1)) << $lsb
    };
}

#[macro_export]
macro_rules! bitfield_read {
    ($register:expr, bit: $bit:literal) => {
        bitfield_read!($register, msb: $bit, lsb: $bit);
    };

    ($register:expr, msb: $msb:literal, lsb: $lsb:literal) => {
        ($register & crate::bitmask!(msb: $msb, lsb: $lsb)) >> $lsb
    };
}

#[macro_export]
macro_rules! bitfield_write {
    ($register:expr, bit: $bit:literal, value: $value:expr) => {
        bitfield_write!($register, msb: $bit, lsb: $bit, value: $value);
    };

    ($register:expr, msb: $msb:literal, lsb: $lsb:literal, value: $value:expr) => {
        {
            let bitmask = bitmask!(msb: $msb, lsb: $lsb);
            (register & !bitmask) | ((value << $lsb) & bitmask)
        }
    };
}

#[macro_export]
macro_rules! sysreg_read {
    ($sysreg:literal) => {
        {
            let value;
            unsafe {
                asm!(
                    ::core::concat!("mrs {value}, ", $sysreg),
                    value = lateout(reg)value
                );
            }
            value
        }
    };
}

#[macro_export]
macro_rules! sysreg_write {
    ($sysreg:literal, $value:expr) => {
        {
            let value = $value;
            unsafe {
                asm!(
                    ::core::concat!("msr ", $sysreg,", {value}"),
                    value = in(reg)value
                );
            }
        }
    };
}

#[macro_export]
macro_rules! sysreg_read_bitfield {
    ($sysreg:literal, bit: $bit:literal) => {
        sysreg_read_bitfield!($sysreg, msb: $bit, lsb: $bit);
    };

    ($sysreg:literal, msb: $msb:literal, lsb: $lsb:literal) => {
        {
            let value;
            unsafe {
                asm!(
                    ::core::concat!("mrs {value}, ", $sysreg),
                    "ubfm {value}, {value}, #{lsb}, #{msb}",
                    value = out(reg) value,
                    msb = const $msb,
                    lsb = const $lsb
                );
                value
            }
        }
    };
}

#[macro_export]
macro_rules! sysreg_write_bitfield {
    ($sysreg:literal, bit: $bit:literal, value: $value:expr) => {
        sysreg_write_bitfield!($sysreg, msb: $bit, lsb: $bit, value: $value);
    };

    ($sysreg:literal, msb: $msb:literal, lsb: $lsb:literal, value: $value:expr) => {
        {
            let reg = 0u64;
            let mask = bitmask!(msb: $msb, lsb: $lsb);
            let value = ($value << $lsb) & mask;
            unsafe {
                asm!(
                    ::core::concat!("mrs {reg}, ", $sysreg),
                    "bic {reg}, {reg}, {mask:x}",
                    "orr {reg}, {reg}, {value:x}",
                    ::core::concat!("msr ", $sysreg, ", {reg}"),
                    reg = in(reg) reg,
                    mask = in(reg) mask,
                    value = in(reg) value
                );
            }
        }
    };
}
