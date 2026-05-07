use core::ptr::NonNull;

use safe_mmio::{UniqueMmioPointer, field, fields::WriteOnly};

#[allow(non_camel_case_types)]
pub enum StmType {
    G_DTMS = 0x0,
    G_DM = 0x8,
    G_DTS = 0x10,
    G_D = 0x18,

    G_FLAGTS = 0x60,
    G_FLAG = 0x68,
    G_TRIGTS = 0x70,
    G_TRIG = 0x78,

    I_DMTS = 0x80,
    I_DM = 0x88,
    I_DTS = 0x90,
    I_D = 0x98,

    I_FLAGTS = 0xE0,
    I_FLAG = 0xE8,
    I_TRIGTS = 0xF0,
    I_TRIG = 0xF8,
}

pub struct Stm<'a> {
    ptr: UniqueMmioPointer<'a, StmMmio>,
}

impl<'a> Stm<'a> {
    pub const fn new(ptr: NonNull<StmMmio>) -> Self {
        Stm {
            ptr: unsafe { UniqueMmioPointer::new(ptr) },
        }
    }

    pub fn write_u8(&mut self, port: u16, typ: StmType, data: u8) {
        let mut ports = field!(self.ptr, ports);
        let mut port = ports.get(port as usize).unwrap();
        let mut regs = field!(port, regs);
        let mut reg = regs.get(typ as usize).unwrap();

        reg.write(data)
    }

    pub fn write_u16(&mut self, port: u16, typ: StmType, data: u16) {
        let mut ports = field!(self.ptr, ports);
        let mut port = ports.get(port as usize).unwrap();
        let mut regs = field!(port, regs);
        let mut reg = regs.get(typ as usize).unwrap();

        unsafe {
            UniqueMmioPointer::new(NonNull::new(reg.ptr_mut() as *mut u16).unwrap())
                .write_unsafe(data)
        }
    }

    pub fn write_u32(&mut self, port: u16, typ: StmType, data: u32) {
        let mut ports = field!(self.ptr, ports);
        let mut port = ports.get(port as usize).unwrap();
        let mut regs = field!(port, regs);
        let mut reg = regs.get(typ as usize).unwrap();

        unsafe {
            UniqueMmioPointer::new(NonNull::new(reg.ptr_mut() as *mut u32).unwrap())
                .write_unsafe(data)
        }
    }

    pub fn write_u64(&mut self, port: u16, typ: StmType, data: u64) {
        let mut ports = field!(self.ptr, ports);
        let mut port = ports.get(port as usize).unwrap();
        let mut regs = field!(port, regs);
        let mut reg = regs.get(typ as usize).unwrap();

        unsafe {
            UniqueMmioPointer::new(NonNull::new(reg.ptr_mut() as *mut u64).unwrap())
                .write_unsafe(data)
        }
    }
}

pub struct StmMmio {
    ports: [StimulusPortMmio; 0x1_0000],
}

pub struct StimulusPortMmio {
    regs: [WriteOnly<u8>; 0x100],
}
