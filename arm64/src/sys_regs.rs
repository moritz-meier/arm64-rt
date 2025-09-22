macro_rules! system_register {
    {
        $(#[$attrs:meta])*
        $vis:vis $name:ident (
            $reg_name:expr, $t:ty, $access:ident
            $(,res0 = $res0:expr)?
            $(,res1 = $res1:expr)?)
            $fields:tt
            $($(#[$enum_attrs:meta])* $enum_vis:vis enum $enum_name:ident $enum_values:tt)*
        } => {
        pastey::paste! {
            pub use [<$name:lower>]::$name;

            $(#[$attrs])*
            $vis const $name: [<$name:lower>]::Register = [<$name:lower>]::Register;

            $(#[$attrs])*
            $vis mod [<$name:lower>] {
                $vis struct Register;

                $vis const RES0: $t = Register::RES0;
                $vis const RES1: $t = Register::RES1;

                impl_system_register! {
                    $vis impl Register($reg_name, $name($t), $access $(,res0 = $res0)? $(,res1 = $res1)?)
                }

                #[allow(unused)]
                use arbitrary_int::*;
                use bitbybit::*;

                system_register_bitfields! {
                    $vis struct $name($t, default = RES1) $fields
                }

                $(
                    $(#[$enum_attrs])*
                    $enum_vis enum $enum_name $enum_values
                )*
            }
        }
    };
}

macro_rules! impl_system_register {
    ($vis:vis impl $name:ident($reg_name:expr, $valtyp:ident($t:ty), r $(,res0 = $res0:expr)? $(,res1 = $res1:expr)?)) => {
        impl $name {

            $vis const RES0: $t = expr_or_default!($($res0)?, 0);
            $vis const RES1: $t = expr_or_default!($($res1)?, 0);

            impl_sysreg_read!{
                $vis fn read($reg_name, $valtyp, $t)
            }
        }
    };

    ($vis:vis impl $name:ident($reg_name:expr, $valtyp:ident($t:ty), w $(,res0 = $res0:expr)? $(,res1 = $res1:expr)?)) => {
        impl $name {

            $vis const RES0: $t = expr_or_default!($($res0)?, 0);
            $vis const RES1: $t = expr_or_default!($($res1)?, 0);
        }
    };

    ($vis:vis impl $name:ident($reg_name:expr, $valtyp:ident($t:ty), rw $(,res0 = $res0:expr)? $(,res1 = $res1:expr)?)) => {
        impl $name {

            $vis const RES0: $t = expr_or_default!($($res0)?, 0);
            $vis const RES1: $t = expr_or_default!($($res1)?, 0);

            impl_sysreg_read!{
                $vis fn read($reg_name, $valtyp, $t)
            }

            impl_sysreg_write!{
                $vis fn write($reg_name, $valtyp, $t)
            }

            impl_sysreg_modify!{
                $vis fn modify($reg_name, $valtyp, $t)
            }
        }
    };
}

macro_rules! impl_sysreg_read {
    ($vis:vis fn read($reg_name:expr, $valtyp:ident, $t:ty)) => {
        $vis fn read(&self) -> $valtyp {
            let value: $t;
            unsafe {
                core::arch::asm!(
                    core::concat!("mrs {value}, ", $reg_name),
                    value = lateout(reg) value
                );
            }
            $valtyp::new_with_raw_value((value & !RES0) | RES1)
        }
    };
}

macro_rules! impl_sysreg_write {
    ($vis:vis fn write($reg_name:expr, $valtyp:ty, $t:ty)) => {
        $vis fn write(&self, value: $valtyp) {
            let value: $t = (value.raw_value() & !RES0) | RES1;
            unsafe {
                core::arch::asm!(
                    "dsb sy",
                    core::concat!("msr ", $reg_name, ", {value}"),
                    "isb sy",
                    value = in(reg) value
                );
            }
        }
    };
}

macro_rules! impl_sysreg_modify {
    ($vis:vis fn modify($reg_name:expr, $valtyp:ty, $t:ty)) => {
        $vis fn modify(&self, f: impl Fn($valtyp) -> $valtyp) {
            let reg = self.read();
            let reg = f(reg);
            self.write(reg);
        }
    };
}

macro_rules! system_register_bitfields {
    ($vis:vis struct $name:ident($t:ty $(,default = $default:expr)?) $fields:tt) => {
        #[bitfield($t $(,default = $default)?)]
        $vis struct $name $fields
    };
}

macro_rules! expr_or_default {
    ($expr:expr, $default:expr) => {
        $expr
    };

    (, $default:expr) => {
        $default
    };
}

mod cache;
mod id;
mod mmu;
mod pmu;
mod system;
mod timer;

pub use cache::*;
pub use id::*;
pub use mmu::*;
pub use pmu::*;
pub use system::*;
pub use timer::*;
