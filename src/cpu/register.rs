use bitflags::bitflags;

pub struct Register {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
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

    #[inline]
    pub fn get_flag(&self, flag: Flag) -> bool {
        self.f & flag.bits > 0
    }

    #[inline]
    pub fn set_flag(&mut self, flag: Flag) {
        self.f |= flag.bits;
        // if value {
        //     self.f |= flag.bits;
        // } else {
        //     self.f &= !flag.bits;
        // }
    }
}

// 一位寄存器访存
impl Register {
    #[inline]
    pub fn get_a(&self) -> u8 {
        self.a
    }

    #[inline]
    pub fn get_b(&self) -> u8 {
        self.b
    }

    #[inline]
    pub fn get_c(&self) -> u8 {
        self.c
    }

    #[inline]
    pub fn get_d(&self) -> u8 {
        self.d
    }

    #[inline]
    pub fn get_e(&self) -> u8 {
        self.e
    }

    #[inline]
    pub fn get_f(&self) -> u8 {
        self.f
    }

    #[inline]
    pub fn get_h(&self) -> u8 {
        self.h
    }

    #[inline]
    pub fn get_l(&self) -> u8 {
        self.l
    }

    #[inline]
    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    #[inline]
    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    #[inline]
    pub fn set_a(&mut self, n: u8) {
        self.a = n
    }

    #[inline]
    pub fn set_b(&mut self, n: u8) {
        self.b = n
    }

    #[inline]
    pub fn set_c(&mut self, n: u8) {
        self.c = n
    }

    #[inline]
    pub fn set_d(&mut self, n: u8) {
        self.d = n
    }

    #[inline]
    pub fn set_e(&mut self, n: u8) {
        self.e = n
    }

    #[inline]
    pub fn set_f(&mut self, n: u8) {
        self.f = n
    }

    #[inline]
    pub fn set_h(&mut self, n: u8) {
        self.h = n
    }

    #[inline]
    pub fn set_l(&mut self, n: u8) {
        self.l = n
    }

    #[inline]
    pub fn set_sp(&mut self, n: u16) {
        self.sp = n
    }

    #[inline]
    pub fn pc_inc(&mut self, n: u16) {
        self.pc += n;
    }
}

// 两位寄存器访存
impl Register {
    #[inline]
    pub fn get_af(&self) -> u16 {
        (u16::from(self.a) << 8) | u16::from(self.f)
        // 从左到右是高字节到低字节，但是这个操作可能需要从内存里读好几次，比较慢
        // u16::from_be_bytes([self.a, self.f])
    }

    #[inline]
    pub fn get_bc(&self) -> u16 {
        (u16::from(self.b) << 8) | u16::from(self.c)
    }

    #[inline]
    pub fn get_de(&self) -> u16 {
        (u16::from(self.d) << 8) | u16::from(self.e)
    }

    #[inline]
    pub fn get_hl(&self) -> u16 {
        (u16::from(self.h) << 8) | u16::from(self.l)
    }

    #[inline]
    pub fn set_af(&mut self, op: u16) {
        self.a = (op >> 8) as u8;
        self.f = (op & 0x00f0) as u8;
    }

    #[inline]
    pub fn set_bc(&mut self, op: u16) {
        self.b = (op >> 8) as u8;
        self.c = (op & 0x00ff) as u8;
    }

    #[inline]
    pub fn set_de(&mut self, op: u16) {
        self.d = (op >> 8) as u8;
        self.e = (op & 0x00ff) as u8;
    }

    #[inline]
    pub fn set_hl(&mut self, op: u16) {
        self.h = (op >> 8) as u8;
        self.l = (op & 0x00ff) as u8;
    }
}
