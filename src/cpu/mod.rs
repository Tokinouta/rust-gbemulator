use crate::memory::Memory;

use self::register::{Flag, Register};

mod register;

struct Cpu {
    register: Register,
    memory: Memory,
    opcode: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            register: Register::new(),
            memory: Memory::new(),
            opcode: 0,
        }
    }

    fn imm8(&mut self) -> u8 {
        let imm8 = self.memory.get8(self.register.get_pc());
        self.register.pc_inc(1);
        imm8
    }

    fn imm16(&mut self) -> u16 {
        let imm16 = self.memory.get16(self.register.get_pc());
        self.register.pc_inc(2);
        imm16
    }

    pub fn ld(&mut self, opcode: u8) {
        match opcode {
            // ld nn,n 8 bit immediate
            0x06 => {
                let n = self.imm8();
                self.register.set_b(n)
            }
            0x0e => {
                let n = self.imm8();
                self.register.set_c(n)
            }
            0x16 => {
                let n = self.imm8();
                self.register.set_d(n)
            }
            0x1e => {
                let n = self.imm8();
                self.register.set_e(n)
            }
            0x26 => {
                let n = self.imm8();
                self.register.set_h(n)
            }
            0x2e => {
                let n = self.imm8();
                self.register.set_l(n)
            }
            0x36 => {
                let n = self.imm8();
                self.memory.set8(self.register.get_hl(), n)
            }
            0x3e => {
                let n = self.imm8();
                self.register.set_a(n)
            }

            // LD n,nn 16 bit immediate
            0x01 => {
                let n = self.imm16();
                self.register.set_bc(n)
            }
            0x11 => {
                let n = self.imm16();
                self.register.set_de(n)
            }
            0x21 => {
                let n = self.imm16();
                self.register.set_hl(n)
            }
            0x31 => {
                let n = self.imm16();
                self.register.set_sp(n)
            }

            // ld r1, r2
            0x7f => self.register.set_a(self.register.get_a()),
            0x78 => self.register.set_a(self.register.get_b()),
            0x79 => self.register.set_a(self.register.get_c()),
            0x7a => self.register.set_a(self.register.get_d()),
            0x7b => self.register.set_a(self.register.get_e()),
            0x7c => self.register.set_a(self.register.get_h()),
            0x7d => self.register.set_a(self.register.get_l()),

            0x40 => self.register.set_b(self.register.get_b()),
            0x41 => self.register.set_b(self.register.get_c()),
            0x42 => self.register.set_b(self.register.get_d()),
            0x43 => self.register.set_b(self.register.get_e()),
            0x44 => self.register.set_b(self.register.get_h()),
            0x45 => self.register.set_b(self.register.get_l()),
            0x47 => self.register.set_b(self.register.get_a()),

            0x48 => self.register.set_c(self.register.get_b()),
            0x49 => self.register.set_c(self.register.get_c()),
            0x4a => self.register.set_c(self.register.get_d()),
            0x4b => self.register.set_c(self.register.get_e()),
            0x4c => self.register.set_c(self.register.get_h()),
            0x4d => self.register.set_c(self.register.get_l()),
            0x4f => self.register.set_c(self.register.get_a()),

            0x50 => self.register.set_d(self.register.get_b()),
            0x51 => self.register.set_d(self.register.get_c()),
            0x52 => self.register.set_d(self.register.get_d()),
            0x53 => self.register.set_d(self.register.get_e()),
            0x54 => self.register.set_d(self.register.get_h()),
            0x55 => self.register.set_d(self.register.get_l()),
            0x57 => self.register.set_d(self.register.get_a()),

            0x58 => self.register.set_e(self.register.get_b()),
            0x59 => self.register.set_e(self.register.get_c()),
            0x5a => self.register.set_e(self.register.get_d()),
            0x5b => self.register.set_e(self.register.get_e()),
            0x5c => self.register.set_e(self.register.get_h()),
            0x5d => self.register.set_e(self.register.get_l()),
            0x5f => self.register.set_e(self.register.get_a()),

            0x60 => self.register.set_h(self.register.get_b()),
            0x61 => self.register.set_h(self.register.get_c()),
            0x62 => self.register.set_h(self.register.get_d()),
            0x63 => self.register.set_h(self.register.get_e()),
            0x64 => self.register.set_h(self.register.get_h()),
            0x65 => self.register.set_h(self.register.get_l()),
            0x67 => self.register.set_h(self.register.get_a()),

            0x68 => self.register.set_l(self.register.get_b()),
            0x69 => self.register.set_l(self.register.get_c()),
            0x6a => self.register.set_l(self.register.get_d()),
            0x6b => self.register.set_l(self.register.get_e()),
            0x6c => self.register.set_l(self.register.get_h()),
            0x6d => self.register.set_l(self.register.get_l()),
            0x6f => self.register.set_l(self.register.get_a()),

            // LD from/to memory
            0x0a => self
                .register
                .set_a(self.memory.get8(self.register.get_bc())),
            0x1a => self
                .register
                .set_a(self.memory.get8(self.register.get_de())),
            0x7e => self
                .register
                .set_a(self.memory.get8(self.register.get_hl())),
            0x46 => self
                .register
                .set_b(self.memory.get8(self.register.get_hl())),
            0x4e => self
                .register
                .set_c(self.memory.get8(self.register.get_hl())),
            0x56 => self
                .register
                .set_d(self.memory.get8(self.register.get_hl())),
            0x5e => self
                .register
                .set_e(self.memory.get8(self.register.get_hl())),
            0x66 => self
                .register
                .set_h(self.memory.get8(self.register.get_hl())),
            0x6e => self
                .register
                .set_l(self.memory.get8(self.register.get_hl())),
            0x02 => self
                .memory
                .set8(self.register.get_bc(), self.register.get_a()),
            0x12 => self
                .memory
                .set8(self.register.get_de(), self.register.get_a()),
            0x70 => self
                .memory
                .set8(self.register.get_hl(), self.register.get_b()),
            0x71 => self
                .memory
                .set8(self.register.get_hl(), self.register.get_c()),
            0x72 => self
                .memory
                .set8(self.register.get_hl(), self.register.get_d()),
            0x73 => self
                .memory
                .set8(self.register.get_hl(), self.register.get_e()),
            0x74 => self
                .memory
                .set8(self.register.get_hl(), self.register.get_h()),
            0x75 => self
                .memory
                .set8(self.register.get_hl(), self.register.get_l()),
            0x77 => self
                .memory
                .set8(self.register.get_hl(), self.register.get_a()),

            0xfa => {
                let n = self.imm16();
                self.register.set_a(self.memory.get8(n))
            }
            0xea => {
                let n = self.imm16();
                self.memory.set8(n, self.register.get_a())
            }

            // LD A,(C)
            0xf2 => self
                .register
                .set_a(self.memory.get8(0xff00 + self.register.get_c() as u16)),
            // LD (C),A
            0xe2 => self
                .memory
                .set8(0xff00 + self.register.get_c() as u16, self.register.get_a()),
            // LDI A,(HL)
            0x2a => {
                let temp = self.register.get_hl();
                self.register.set_a(self.memory.get8(temp));
                self.register.set_hl(temp + 1);
            }
            // LDI (HL),A
            0x22 => {
                let temp = self.register.get_hl();
                self.memory.set8(temp, self.register.get_a());
                self.register.set_hl(temp + 1);
            }
            // LDD A,(HL)
            0x3a => {
                let temp = self.register.get_hl();
                self.register.set_a(self.memory.get8(temp));
                self.register.set_hl(temp - 1);
            }
            // LDD (HL),A
            0x32 => {
                let temp = self.register.get_hl();
                self.memory.set8(temp, self.register.get_a());
                self.register.set_hl(temp - 1);
            }
            // LDH (n),A
            0xe0 => {
                let n = self.imm8();
                self.memory.set8(0xff00 + n as u16, self.register.get_a())
            }
            // LDH A,(n)
            0xf0 => {
                let n = self.imm8();
                self.register.set_a(self.memory.get8(0xff00 + n as u16))
            }

            // LD SP,HL
            0xf9 => self.register.set_sp(self.register.get_hl()),
            // LD HL,SP+n
            0xf8 => {
                let a = self.register.get_sp();
                let b = self.imm8() as i8 as i16 as u16;
                let flag = !Flag::Z
                    | !Flag::N
                    | if (a & 0x00ff) + (b & 0x00ff) > 0x00ff {
                        Flag::C
                    } else {
                        !Flag::C
                    }
                    | if (a & 0x000f) + (b & 0x000f) > 0x000f {
                        Flag::H
                    } else {
                        !Flag::H
                    };
                self.register.set_flag(flag);
                self.register.set_hl(a.wrapping_add(b));
            }
            // LD (nn),SP
            0x08 => {
                let n = self.imm16();
                self.memory.set16(self.register.get_sp(), n)
            }

            // PUSH nn
            0xf5 => self.push(self.register.get_af()),
            0xc5 => self.push(self.register.get_bc()),
            0xd5 => self.push(self.register.get_de()),
            0xe5 => self.push(self.register.get_hl()),

            // POP nn
            0xf1 => {
                self.register
                    .set_af(self.memory.get16(self.register.get_sp()));
                self.register.set_sp(self.register.get_sp() + 2)
            }
            0xc1 => {
                self.register
                    .set_bc(self.memory.get16(self.register.get_sp()));
                self.register.set_sp(self.register.get_sp() + 2)
            }
            0xd1 => {
                self.register
                    .set_de(self.memory.get16(self.register.get_sp()));
                self.register.set_sp(self.register.get_sp() + 2)
            }
            0xe1 => {
                self.register
                    .set_hl(self.memory.get16(self.register.get_sp()));
                self.register.set_sp(self.register.get_sp() + 2)
            }
            _ => (),
        }
    }

    #[inline]
    pub fn push(&mut self, n: u16) {
        self.memory.set16(self.register.get_sp(), n);
        self.register.set_sp(self.register.get_sp() - 2)
    }

    pub fn add8(&mut self, b: u8) {
        let a = self.register.get_a();
        let res = a.wrapping_add(b);
        let flag = if res == 0 { Flag::Z } else { !Flag::Z }
            | !Flag::N
            | if (a as u16) + (b as u16) > 0x00ff {
                Flag::C
            } else {
                !Flag::C
            }
            | if (a & 0x0f) + (b & 0x0f) > 0x0f {
                Flag::H
            } else {
                !Flag::H
            };
        self.register.set_flag(flag);
        self.register.set_a(res);
    }

    pub fn adc8(&mut self, b: u8) {
        let a = self.register.get_a();
        let carry = u8::from(self.register.get_flag(Flag::C));
        let res = a.wrapping_add(b).wrapping_sub(carry);
        let flag = if res == 0 { Flag::Z } else { !Flag::Z }
            | !Flag::N
            | if (a as u16) + (b as u16) + (carry as u16) > 0x00ff {
                Flag::C
            } else {
                !Flag::C
            }
            | if (a & 0x0f) + (b & 0x0f) + carry > 0x0f {
                Flag::H
            } else {
                !Flag::H
            };
        self.register.set_flag(flag);
        self.register.set_a(res);
    }

    pub fn sub8(&mut self, b: u8) {
        let a = self.register.get_a();
        let res = a.wrapping_sub(b);
        let flag = if res == 0 { Flag::Z } else { !Flag::Z }
            | !Flag::N
            | if (a as u16) < (b as u16) {
                Flag::C
            } else {
                !Flag::C
            }
            | if (a & 0x0f) < (b & 0x0f) {
                Flag::H
            } else {
                !Flag::H
            };
        self.register.set_flag(flag);
        self.register.set_a(res);
    }

    pub fn sbc8(&mut self, b: u8) {
        let a = self.register.get_a();
        let carry = u8::from(self.register.get_flag(Flag::C));
        let res = a.wrapping_sub(b).wrapping_sub(carry);
        let flag = if res == 0 { Flag::Z } else { !Flag::Z }
            | !Flag::N
            | if (a as u16) < ((b + carry) as u16) {
                Flag::C
            } else {
                !Flag::C
            }
            | if (a & 0x0f) < ((b + carry) & 0x0f) {
                Flag::H
            } else {
                !Flag::H
            };
        self.register.set_flag(flag);
        self.register.set_a(res);
    }
}
