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
        const N =  0x40; // operation
        const H =  0x20; // half-carry
        const C =  0x10; // carry
    }
}

impl Register {
    #[rustfmt::skip]
    pub fn new() -> Self {
        Self { a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, h: 0, l: 0, pc: 0x100, sp: 0xfffe }
    }

    pub fn get_flag(&self) -> Flag {
        Flag::from_bits_truncate(self.f)
    }

    pub fn set_flag(&mut self, flag: Flag) {
        self.f |= flag.bits;
    }
}

// 一位寄存器访存
impl Register {
    pub fn get_a(&self) -> u8 {
        self.a
    }

    pub fn get_b(&self) -> u8 {
        self.b
    }

    pub fn get_c(&self) -> u8 {
        self.c
    }

    pub fn get_d(&self) -> u8 {
        self.d
    }

    pub fn get_e(&self) -> u8 {
        self.e
    }

    pub fn get_f(&self) -> u8 {
        self.f
    }

    pub fn get_h(&self) -> u8 {
        self.h
    }

    pub fn get_l(&self) -> u8 {
        self.l
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    pub fn set_a(&mut self, n: u8) {
        self.a = n
    }

    pub fn set_b(&mut self, n: u8) {
        self.b = n
    }

    pub fn set_c(&mut self, n: u8) {
        self.c = n
    }

    pub fn set_d(&mut self, n: u8) {
        self.d = n
    }

    pub fn set_e(&mut self, n: u8) {
        self.e = n
    }

    pub fn set_f(&mut self, n: u8) {
        self.f = n
    }

    pub fn set_h(&mut self, n: u8) {
        self.h = n
    }

    pub fn set_l(&mut self, n: u8) {
        self.l = n
    }

    pub fn set_sp(&mut self, n: u16) {
        self.sp = n
    }

    pub fn pc_inc(&mut self, n: u16) {
        self.pc += n;
    }
}

// 两位寄存器访存
impl Register {
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
}
