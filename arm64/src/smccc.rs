use core::{arch::asm, marker::PhantomData};

pub trait SmcccCall32 {
    fn call(function: u32, args: [u32; 7]) -> Result<[u32; 8], i32>;
}

pub trait SmcccCall64 {
    fn call(function: u32, args: [u64; 17]) -> Result<[u64; 18], i64>;
}

pub struct SMC;
pub struct HVC;

pub struct Smccc<T> {
    _phantom: PhantomData<T>,
}

impl SmcccCall32 for Smccc<SMC> {
    fn call(function: u32, args: [u32; 7]) -> Result<[u32; 8], i32> {
        let mut res = [0u32; 8];
        unsafe {
            asm!(
                "smc #0",
                inout("w0") function => res[0],
                inout("w1") args[0] => res[1],
                inout("w2") args[1] => res[2],
                inout("w3") args[2] => res[3],
                inout("w4") args[3] => res[4],
                inout("w5") args[4] => res[5],
                inout("w6") args[5] => res[6],
                inout("w7") args[6] => res[7],
                options(nostack)
            )
        }

        match res[0] as i32 {
            ret if ret >= 0 => Ok(res),
            ret => Err(ret),
        }
    }
}

impl SmcccCall64 for Smccc<SMC> {
    fn call(function: u32, args: [u64; 17]) -> Result<[u64; 18], i64> {
        let mut res = [0u64; 18];
        unsafe {
            asm!(
                "smc #0",
                inout("x0") function as u64 => res[0],
                inout("x1") args[0] => res[1],
                inout("x2") args[1] => res[2],
                inout("x3") args[2] => res[3],
                inout("x4") args[3] => res[4],
                inout("x5") args[4] => res[5],
                inout("x6") args[5] => res[6],
                inout("x7") args[6] => res[7],
                inout("x8") args[7] => res[8],
                inout("x9") args[8] => res[9],
                inout("x10") args[9] => res[10],
                inout("x11") args[10] => res[11],
                inout("x12") args[11] => res[12],
                inout("x13") args[12] => res[13],
                inout("x14") args[13] => res[14],
                inout("x15") args[14] => res[15],
                inout("x16") args[15] => res[16],
                inout("x17") args[16] => res[17],
                options(nostack)
            )
        }

        match res[0] as i64 {
            ret if ret >= 0 => Ok(res),
            ret => Err(ret.into()),
        }
    }
}

impl SmcccCall32 for Smccc<HVC> {
    fn call(function: u32, args: [u32; 7]) -> Result<[u32; 8], i32> {
        let mut res = [0u32; 8];
        unsafe {
            asm!(
                "hvc #0",
                inout("w0") function => res[0],
                inout("w1") args[0] => res[1],
                inout("w2") args[1] => res[2],
                inout("w3") args[2] => res[3],
                inout("w4") args[3] => res[4],
                inout("w5") args[4] => res[5],
                inout("w6") args[5] => res[6],
                inout("w7") args[6] => res[7],
                options(nostack)
            )
        }

        match res[0] as i32 {
            ret if ret >= 0 => Ok(res),
            ret => Err(ret.into()),
        }
    }
}

impl SmcccCall64 for Smccc<HVC> {
    fn call(function: u32, args: [u64; 17]) -> Result<[u64; 18], i64> {
        let mut res = [0u64; 18];
        unsafe {
            asm!(
                "hvc #0",
                inout("x0") function as u64 => res[0],
                inout("x1") args[0] => res[1],
                inout("x2") args[1] => res[2],
                inout("x3") args[2] => res[3],
                inout("x4") args[3] => res[4],
                inout("x5") args[4] => res[5],
                inout("x6") args[5] => res[6],
                inout("x7") args[6] => res[7],
                inout("x8") args[7] => res[8],
                inout("x9") args[8] => res[9],
                inout("x10") args[9] => res[10],
                inout("x11") args[10] => res[11],
                inout("x12") args[11] => res[12],
                inout("x13") args[12] => res[13],
                inout("x14") args[13] => res[14],
                inout("x15") args[14] => res[15],
                inout("x16") args[15] => res[16],
                inout("x17") args[16] => res[17],
                options(nostack)
            )
        }

        match res[0] as i64 {
            ret if ret >= 0 => Ok(res),
            ret => Err(ret.into()),
        }
    }
}
