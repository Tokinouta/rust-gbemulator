use bitflags::bitflags;

pub struct Register {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
}

bitflags! {
    pub struct Flag: u8 {
        const Z =  0x80; // zero
        const O =  0x40; // operation
        const H =  0x20; // half-carry
        const C =  0x10; // carry
    }
}

impl Register {
    #[rustfmt::skip]
    pub fn new() -> Self {
        Self { a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, h: 0, l: 0, pc: 0x100, sp: 0xfffe }
    }

    pub fn get_af(&self) -> u16 {
        (u16::from(self.a) << 8) | u16::from(self.f)
        // 从左到右是高字节到低字节，但是这个操作可能需要从内存里读好几次，比较慢
        // u16::from_be_bytes([self.a, self.f])
    }

    pub fn get_bc(&self) -> u16 {
        (u16::from(self.b) << 8) | u16::from(self.c)
    }

    pub fn get_de(&self) -> u16 {
        (u16::from(self.d) << 8) | u16::from(self.e)
    }

    pub fn get_hl(&self) -> u16 {
        (u16::from(self.h) << 8) | u16::from(self.l)
    }

    pub fn get_flag(&self) -> Flag {
        Flag::from_bits_truncate(self.f)
    }

    pub fn set_af(&mut self, op: u16) {
        self.a = (op >> 8) as u8;
        self.f = (op & 0x00f0) as u8;
    }

    pub fn set_bc(&mut self, op: u16) {
        self.b = (op >> 8) as u8;
        self.c = (op & 0x00ff) as u8;
    }

    pub fn set_de(&mut self, op: u16) {
        self.d = (op >> 8) as u8;
        self.e = (op & 0x00ff) as u8;
    }

    pub fn set_hl(&mut self, op: u16) {
        self.h = (op >> 8) as u8;
        self.l = (op & 0x00ff) as u8;
    }

    pub fn set_flag(&mut self, flag: Flag) {
        self.f |= flag.bits;
    }
}
