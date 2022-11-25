use bitflags::bitflags;

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
    pub flag: u8,
    pub enabled: u8,
}

impl Interrupt {
    pub fn new() -> Self {
        Self {
            flag: 0,
            enabled: 0,
        }
    }

    pub fn handle_interrupt(&mut self, flag: IntFlag) {
        self.flag |= flag.bits();
    }
}
