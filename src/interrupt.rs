use bitflags::bitflags;

use crate::memory::MemoryIO;

bitflags! {
    pub struct IntFlag: u8 {
        const VBLANK = 0x01;
        const LCDSTAT = 0x02;
        const TIMER = 0x04;
        const SERIAL = 0x08;
        const JOYPAD = 0x10;
    }
}

pub struct Interrupt {
    pub request: u8,
    pub enabled: u8,
}

impl Interrupt {
    pub fn new() -> Self {
        Self {
            request: 0,
            enabled: 0,
        }
    }

    pub fn request_interrupt(&mut self, flag: IntFlag) {
        self.request |= flag.bits();
    }
}

impl MemoryIO for Interrupt {
    fn get8(&self, address: u16) -> u8 {
        match address {
            0xffff => self.enabled,
            0xff0f => self.request,
            _ => 0,
        }
    }

    fn set8(&mut self, address: u16, n: u8) {
        match address {
            0xffff => self.enabled = n,
            0xff0f => self.request = n,
            _ => (),
        }
    }

    fn get16(&self, address: u16) -> u16 {
        unimplemented!()
    }

    fn set16(&mut self, address: u16, n: u16) {
        unimplemented!()
    }
}
