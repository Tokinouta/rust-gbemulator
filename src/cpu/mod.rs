use crate::{
    mbc::CartridgeHeader,
    memory::{Memory, MemoryIO},
};

use self::register::{Flag, Register};

mod register;

struct Cpu {
    register: Register,
    memory: Memory,
    opcode: u8,
}

impl Cpu {
    pub fn new(header: CartridgeHeader) -> Self {
        Self {
            register: Register::new(),
            memory: Memory::new(header),
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
            // NOP
            0x00 => (),

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
            0xf1 | 0xc1 | 0xd1 | 0xe1 => self.pop(opcode),

            // ADD A,n
            0x87 => self.add8(self.register.get_a()),
            0x80 => self.add8(self.register.get_b()),
            0x81 => self.add8(self.register.get_c()),
            0x82 => self.add8(self.register.get_d()),
            0x83 => self.add8(self.register.get_e()),
            0x84 => self.add8(self.register.get_h()),
            0x85 => self.add8(self.register.get_l()),
            0x86 => self.add8(self.memory.get8(self.register.get_hl())),
            0xc6 => {
                let n = self.imm8();
                self.add8(n)
            }

            // ADC A,n
            0x8f => self.adc8(self.register.get_a()),
            0x88 => self.adc8(self.register.get_b()),
            0x89 => self.adc8(self.register.get_c()),
            0x8a => self.adc8(self.register.get_d()),
            0x8b => self.adc8(self.register.get_e()),
            0x8c => self.adc8(self.register.get_h()),
            0x8d => self.adc8(self.register.get_l()),
            0x8e => self.adc8(self.memory.get8(self.register.get_hl())),
            0xce => {
                let n = self.imm8();
                self.adc8(n)
            }

            // SUB A,n
            0x97 => self.sub8(self.register.get_a()),
            0x90 => self.sub8(self.register.get_b()),
            0x91 => self.sub8(self.register.get_c()),
            0x92 => self.sub8(self.register.get_d()),
            0x93 => self.sub8(self.register.get_e()),
            0x94 => self.sub8(self.register.get_h()),
            0x95 => self.sub8(self.register.get_l()),
            0x96 => self.sub8(self.memory.get8(self.register.get_hl())),
            0xd6 => {
                let n = self.imm8();
                self.sub8(n)
            }

            // SBC A,n
            0x9f => self.sbc8(self.register.get_a()),
            0x98 => self.sbc8(self.register.get_b()),
            0x99 => self.sbc8(self.register.get_c()),
            0x9a => self.sbc8(self.register.get_d()),
            0x9b => self.sbc8(self.register.get_e()),
            0x9c => self.sbc8(self.register.get_h()),
            0x9d => self.sbc8(self.register.get_l()),
            0x9e => self.sbc8(self.memory.get8(self.register.get_hl())),
            0xde => {
                let n = self.imm8();
                self.sbc8(n)
            }

            // AND A,n
            0xa7 => self.and8(self.register.get_a()),
            0xa0 => self.and8(self.register.get_b()),
            0xa1 => self.and8(self.register.get_c()),
            0xa2 => self.and8(self.register.get_d()),
            0xa3 => self.and8(self.register.get_e()),
            0xa4 => self.and8(self.register.get_h()),
            0xa5 => self.and8(self.register.get_l()),
            0xa6 => self.and8(self.memory.get8(self.register.get_hl())),
            0xe6 => {
                let n = self.imm8();
                self.and8(n)
            }

            // OR A,n
            0xb7 => self.or8(self.register.get_a()),
            0xb0 => self.or8(self.register.get_b()),
            0xb1 => self.or8(self.register.get_c()),
            0xb2 => self.or8(self.register.get_d()),
            0xb3 => self.or8(self.register.get_e()),
            0xb4 => self.or8(self.register.get_h()),
            0xb5 => self.or8(self.register.get_l()),
            0xb6 => self.or8(self.memory.get8(self.register.get_hl())),
            0xf6 => {
                let n = self.imm8();
                self.or8(n)
            }

            // XOR A,n
            0xaf => self.xor8(self.register.get_a()),
            0xa8 => self.xor8(self.register.get_b()),
            0xa9 => self.xor8(self.register.get_c()),
            0xaa => self.xor8(self.register.get_d()),
            0xab => self.xor8(self.register.get_e()),
            0xac => self.xor8(self.register.get_h()),
            0xad => self.xor8(self.register.get_l()),
            0xae => self.xor8(self.memory.get8(self.register.get_hl())),
            0xee => {
                let n = self.imm8();
                self.xor8(n)
            }

            // CP A,n
            0xbf => self.cp8(self.register.get_a()),
            0xb8 => self.cp8(self.register.get_b()),
            0xb9 => self.cp8(self.register.get_c()),
            0xba => self.cp8(self.register.get_d()),
            0xbb => self.cp8(self.register.get_e()),
            0xbc => self.cp8(self.register.get_h()),
            0xbd => self.cp8(self.register.get_l()),
            0xbe => self.cp8(self.memory.get8(self.register.get_hl())),
            0xfe => {
                let n = self.imm8();
                self.cp8(n)
            }

            // INC n
            0x3c | 0x04 | 0x0c | 0x14 | 0x1c | 0x24 | 0x2c | 0x34 => self.inc8(opcode),
            // DEC n
            0x3d | 0x05 | 0x0d | 0x15 | 0x1d | 0x25 | 0x2d | 0x35 => self.dec8(opcode),

            // ADD HL,n
            0x09 => self.add16(self.register.get_bc()),
            0x19 => self.add16(self.register.get_de()),
            0x29 => self.add16(self.register.get_hl()),
            0x39 => self.add16(self.register.get_sp()),

            // ADD SP,n
            0xe8 => self.add16_sp(),

            // INC nn
            0x03 => self.register.set_bc(self.register.get_bc().wrapping_add(1)),
            0x13 => self.register.set_de(self.register.get_de().wrapping_add(1)),
            0x23 => self.register.set_hl(self.register.get_hl().wrapping_add(1)),
            0x33 => self.register.set_sp(self.register.get_sp().wrapping_add(1)),
            // DEC nn
            0x0b => self.register.set_bc(self.register.get_bc().wrapping_sub(1)),
            0x1b => self.register.set_de(self.register.get_de().wrapping_sub(1)),
            0x2b => self.register.set_hl(self.register.get_hl().wrapping_sub(1)),
            0x3b => self.register.set_sp(self.register.get_sp().wrapping_sub(1)),

            // DAA
            0x27 => self.daa(),

            // CPL
            0x2f => self.register.a ^= 0xff,

            // CCF
            0x3f => {
                let flag = self.register.get_flag(Flag::C);
                let new_flag = if flag { !Flag::C } else { Flag::C } | !Flag::N | !Flag::H;
                self.register.set_flag(new_flag);
            }

            // SCF
            0x37 => {
                let flag = Flag::C | !Flag::N | !Flag::C;
                self.register.set_flag(flag);
            }

            // HALT
            0x76 => (),

            // STOP
            0x10 => (),

            // DI
            0xf3 => (),

            // EI
            0xfb => (),

            // RLCA
            0x07 => self.register.a = self.rlc(self.register.a),
            // RLCA
            0x0f => self.register.a = self.rl(self.register.a),
            // RLCA
            0x17 => self.register.a = self.rrc(self.register.a),
            // RLCA
            0x1f => self.register.a = self.rr(self.register.a),

            0xcb => {
                let opcode2 = self.imm8();
                match opcode2 {
                    // SWAP
                    0x37 => self.register.a = self.swap(self.register.a),
                    0x30 => self.register.b = self.swap(self.register.b),
                    0x31 => self.register.c = self.swap(self.register.c),
                    0x32 => self.register.d = self.swap(self.register.d),
                    0x33 => self.register.e = self.swap(self.register.e),
                    0x34 => self.register.h = self.swap(self.register.h),
                    0x35 => self.register.l = self.swap(self.register.l),
                    0x36 => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.swap(temp);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // RLC n
                    0x07 => self.register.a = self.rlc(self.register.a),
                    0x00 => self.register.b = self.rlc(self.register.b),
                    0x01 => self.register.c = self.rlc(self.register.c),
                    0x02 => self.register.d = self.rlc(self.register.d),
                    0x03 => self.register.e = self.rlc(self.register.e),
                    0x04 => self.register.h = self.rlc(self.register.h),
                    0x05 => self.register.l = self.rlc(self.register.l),
                    0x06 => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.rlc(temp);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // RL n
                    0x17 => self.register.a = self.rl(self.register.a),
                    0x10 => self.register.b = self.rl(self.register.b),
                    0x11 => self.register.c = self.rl(self.register.c),
                    0x12 => self.register.d = self.rl(self.register.d),
                    0x13 => self.register.e = self.rl(self.register.e),
                    0x14 => self.register.h = self.rl(self.register.h),
                    0x15 => self.register.l = self.rl(self.register.l),
                    0x16 => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.rl(temp);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // RRC n
                    0x0f => self.register.a = self.rrc(self.register.a),
                    0x08 => self.register.b = self.rrc(self.register.b),
                    0x09 => self.register.c = self.rrc(self.register.c),
                    0x0a => self.register.d = self.rrc(self.register.d),
                    0x0b => self.register.e = self.rrc(self.register.e),
                    0x0c => self.register.h = self.rrc(self.register.h),
                    0x0d => self.register.l = self.rrc(self.register.l),
                    0x0e => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.rrc(temp);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // RR n
                    0x1f => self.register.a = self.rr(self.register.a),
                    0x18 => self.register.b = self.rr(self.register.b),
                    0x19 => self.register.c = self.rr(self.register.c),
                    0x1a => self.register.d = self.rr(self.register.d),
                    0x1b => self.register.e = self.rr(self.register.e),
                    0x1c => self.register.h = self.rr(self.register.h),
                    0x1d => self.register.l = self.rr(self.register.l),
                    0x1e => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.rr(temp);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // SLA n
                    0x27 => self.register.a = self.sl(self.register.a),
                    0x20 => self.register.b = self.sl(self.register.b),
                    0x21 => self.register.c = self.sl(self.register.c),
                    0x22 => self.register.d = self.sl(self.register.d),
                    0x23 => self.register.e = self.sl(self.register.e),
                    0x24 => self.register.h = self.sl(self.register.h),
                    0x25 => self.register.l = self.sl(self.register.l),
                    0x26 => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.sl(temp);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // SRA n
                    0x2f => self.register.a = self.sr(self.register.a),
                    0x28 => self.register.b = self.sr(self.register.b),
                    0x29 => self.register.c = self.sr(self.register.c),
                    0x2a => self.register.d = self.sr(self.register.d),
                    0x2b => self.register.e = self.sr(self.register.e),
                    0x2c => self.register.h = self.sr(self.register.h),
                    0x2d => self.register.l = self.sr(self.register.l),
                    0x2e => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.sr(temp);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // SRL n
                    0x3f => self.register.a = self.srl(self.register.a),
                    0x38 => self.register.b = self.srl(self.register.b),
                    0x39 => self.register.c = self.srl(self.register.c),
                    0x3a => self.register.d = self.srl(self.register.d),
                    0x3b => self.register.e = self.srl(self.register.e),
                    0x3c => self.register.h = self.srl(self.register.h),
                    0x3d => self.register.l = self.srl(self.register.l),
                    0x3e => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.srl(temp);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    _ => (),
                }
            }
            _ => (),
        }
    }

    fn push(&mut self, n: u16) {
        self.memory.set16(self.register.get_sp(), n);
        self.register.set_sp(self.register.get_sp() - 2)
    }

    fn pop(&mut self, opcode: u8) {
        let address = self.memory.get16(self.register.get_sp());
        match opcode {
            0xf1 => self.register.set_af(address),
            0xc1 => self.register.set_bc(address),
            0xd1 => self.register.set_de(address),
            0xe1 => self.register.set_hl(address),
            _ => (),
        }
        self.register.set_sp(self.register.get_sp() + 2);
    }

    fn add8(&mut self, n: u8) {
        let a = self.register.get_a();
        let (res, carry) = a.overflowing_add(n);
        let flag = if res == 0 { Flag::Z } else { !Flag::Z }
            | !Flag::N
            | if carry { Flag::C } else { !Flag::C }
            | if (a & 0x0f) + (n & 0x0f) > 0x0f {
                Flag::H
            } else {
                !Flag::H
            };
        self.register.set_flag(flag);
        self.register.set_a(res);
    }

    fn add16(&mut self, n: u16) {
        let a = self.register.get_hl();
        let (res, carry) = a.overflowing_add(n);
        let flag = !Flag::N
            | if carry { Flag::C } else { !Flag::C }
            | if (a & 0x0fff) + (n & 0x0fff) > 0x0fff {
                Flag::H
            } else {
                !Flag::H
            };
        self.register.set_flag(flag);
        self.register.set_hl(res);
    }

    fn add16_sp(&mut self) {
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
        self.register.set_sp(a.wrapping_add(b));
    }

    fn adc8(&mut self, n: u8) {
        let a = self.register.get_a();
        let carry = u8::from(self.register.get_flag(Flag::C));
        let res = a.wrapping_add(n).wrapping_sub(carry);
        let flag = if res == 0 { Flag::Z } else { !Flag::Z }
            | !Flag::N
            | if (a as u16) + (n as u16) + (carry as u16) > 0x00ff {
                Flag::C
            } else {
                !Flag::C
            }
            | if (a & 0x0f) + (n & 0x0f) + carry > 0x0f {
                Flag::H
            } else {
                !Flag::H
            };
        self.register.set_flag(flag);
        self.register.set_a(res);
    }

    fn sub8(&mut self, n: u8) {
        let a = self.register.get_a();
        let res = a.wrapping_sub(n);
        let flag = if res == 0 { Flag::Z } else { !Flag::Z }
            | Flag::N
            | if (a as u16) < (n as u16) {
                Flag::C
            } else {
                !Flag::C
            }
            | if (a & 0x0f) < (n & 0x0f) {
                Flag::H
            } else {
                !Flag::H
            };
        self.register.set_flag(flag);
        self.register.set_a(res);
    }

    fn sbc8(&mut self, n: u8) {
        let a = self.register.get_a();
        let carry = u8::from(self.register.get_flag(Flag::C));
        let res = a.wrapping_sub(n).wrapping_sub(carry);
        let flag = if res == 0 { Flag::Z } else { !Flag::Z }
            | Flag::N
            | if (a as u16) < ((n + carry) as u16) {
                Flag::C
            } else {
                !Flag::C
            }
            | if (a & 0x0f) < ((n + carry) & 0x0f) {
                Flag::H
            } else {
                !Flag::H
            };
        self.register.set_flag(flag);
        self.register.set_a(res);
    }

    fn and8(&mut self, n: u8) {
        let a = self.register.get_a();
        let res = a & n;
        let flag = if res == 0 { Flag::Z } else { !Flag::Z } | !Flag::N | Flag::H | !Flag::C;
        self.register.set_flag(flag);
        self.register.set_a(res);
    }

    fn or8(&mut self, n: u8) {
        let a = self.register.get_a();
        let res = a | n;
        let flag = if res == 0 { Flag::Z } else { !Flag::Z } | !Flag::N | !Flag::H | !Flag::C;
        self.register.set_flag(flag);
        self.register.set_a(res);
    }

    fn xor8(&mut self, n: u8) {
        let a = self.register.get_a();
        let res = a ^ n;
        let flag = if res == 0 { Flag::Z } else { !Flag::Z } | !Flag::N | !Flag::H | !Flag::C;
        self.register.set_flag(flag);
        self.register.set_a(res);
    }

    fn cp8(&mut self, n: u8) {
        let a = self.register.get_a();
        let flag = if a == n { Flag::Z } else { !Flag::Z }
            | Flag::N
            | if (a as u16) < (n as u16) {
                Flag::C
            } else {
                !Flag::C
            }
            | if (a & 0x0f) < (n & 0x0f) {
                Flag::H
            } else {
                !Flag::H
            };
        self.register.set_flag(flag);
    }

    fn inc8(&mut self, opcode: u8) {
        let mut temp = 0;
        match opcode {
            0x3c => {
                self.register.set_a(self.register.get_a().wrapping_add(1));
                temp = self.register.get_a();
            }
            0x04 => {
                self.register.set_b(self.register.get_b().wrapping_add(1));
                temp = self.register.get_b();
            }
            0x0c => {
                self.register.set_c(self.register.get_c().wrapping_add(1));
                temp = self.register.get_c();
            }
            0x14 => {
                self.register.set_d(self.register.get_d().wrapping_add(1));
                temp = self.register.get_d();
            }
            0x1c => {
                self.register.set_e(self.register.get_e().wrapping_add(1));
                temp = self.register.get_e();
            }
            0x24 => {
                self.register.set_h(self.register.get_h().wrapping_add(1));
                temp = self.register.get_h();
            }
            0x2c => {
                self.register.set_l(self.register.get_l().wrapping_add(1));
                temp = self.register.get_l();
            }
            0x34 => {
                let address = self.register.get_hl();
                let new_value = self.memory.get8(address).wrapping_add(1);
                self.memory.set8(address, new_value);
                temp = new_value;
            }
            _ => (),
        }
        let flag = if temp == 0 { Flag::Z } else { !Flag::Z }
            | !Flag::N
            | if temp & 0x0f == 0 { Flag::H } else { !Flag::H };
        self.register.set_flag(flag)
    }

    fn dec8(&mut self, opcode: u8) {
        let mut temp = 0;
        match opcode {
            0x3d => {
                self.register.set_a(self.register.get_a().wrapping_sub(1));
                temp = self.register.get_a();
            }
            0x05 => {
                self.register.set_b(self.register.get_b().wrapping_sub(1));
                temp = self.register.get_b();
            }
            0x0d => {
                self.register.set_c(self.register.get_c().wrapping_sub(1));
                temp = self.register.get_c();
            }
            0x15 => {
                self.register.set_d(self.register.get_d().wrapping_sub(1));
                temp = self.register.get_d();
            }
            0x1d => {
                self.register.set_e(self.register.get_e().wrapping_sub(1));
                temp = self.register.get_e();
            }
            0x25 => {
                self.register.set_h(self.register.get_h().wrapping_sub(1));
                temp = self.register.get_h();
            }
            0x2d => {
                self.register.set_l(self.register.get_l().wrapping_sub(1));
                temp = self.register.get_l();
            }
            0x35 => {
                let address = self.register.get_hl();
                let new_value = self.memory.get8(address).wrapping_sub(1);
                self.memory.set8(address, new_value);
                temp = new_value;
            }
            _ => (),
        }
        let flag = if temp == 0 { Flag::Z } else { !Flag::Z }
            | Flag::N
            | if temp & 0x0f == 0x0f {
                Flag::H
            } else {
                !Flag::H
            };
        self.register.set_flag(flag)
    }

    fn swap(&mut self, reg: u8) -> u8 {
        let flag = if reg == 0 { Flag::Z } else { !Flag::Z } | !Flag::N | !Flag::H | !Flag::C;
        self.register.set_flag(flag);
        (reg & 0x0f) << 4 | (reg & 0xf0) >> 4
    }

    fn daa(&mut self) {
        let mut a = self.register.a;
        let mut adjust = 0;
        if self.register.get_flag(Flag::C) {
            adjust |= 0x60;
        }
        if self.register.get_flag(Flag::H) {
            adjust |= 0x06;
        };
        if !self.register.get_flag(Flag::N) {
            if a & 0x0F > 0x09 {
                adjust |= 0x06;
            };
            if a > 0x99 {
                adjust |= 0x60;
            };
            a = a.wrapping_add(adjust);
        } else {
            a = a.wrapping_sub(adjust);
        }

        let flag = if adjust >= 0x60 { Flag::C } else { !Flag::C }
            | !Flag::H
            | if a == 0 { Flag::Z } else { !Flag::Z };
        self.register.set_flag(flag);
        self.register.a = a;
    }

    fn rlc(&mut self, reg: u8) -> u8 {
        let temp = reg | 0x80;
        let mut flag = if temp != 0 { Flag::C } else { !Flag::C } | !Flag::N | !Flag::H;
        let reg = reg << 1 | temp >> 7;
        flag |= if reg == 0 { Flag::Z } else { !Flag::Z };
        self.register.set_flag(flag);
        reg
    }

    fn rl(&mut self, reg: u8) -> u8 {
        let temp = reg | 0x80;
        let mut flag = if temp != 0 { Flag::C } else { !Flag::C } | !Flag::N | !Flag::H;
        let temp = if self.register.get_flag(Flag::C) {
            1
        } else {
            0
        };
        let reg = reg << 1 | temp;
        flag |= if reg == 0 { Flag::Z } else { !Flag::Z };
        self.register.set_flag(flag);
        reg
    }

    fn rrc(&mut self, reg: u8) -> u8 {
        let temp = reg | 0x01;
        let mut flag = if temp != 0 { Flag::C } else { !Flag::C } | !Flag::N | !Flag::H;
        let reg = reg >> 1 | temp << 7;
        flag |= if reg == 0 { Flag::Z } else { !Flag::Z };
        self.register.set_flag(flag);
        reg
    }

    fn rr(&mut self, reg: u8) -> u8 {
        let temp = reg | 0x01;
        let mut flag = if temp != 0 { Flag::C } else { !Flag::C } | !Flag::N | !Flag::H;
        let temp = if self.register.get_flag(Flag::C) {
            1
        } else {
            0
        };
        let reg = reg >> 1 | temp << 7;
        flag |= if reg == 0 { Flag::Z } else { !Flag::Z };
        self.register.set_flag(flag);
        reg
    }

    fn sl(&mut self, reg: u8) -> u8 {
        let temp = reg | 0x80;
        let mut flag = if temp != 0 { Flag::C } else { !Flag::C } | !Flag::N | !Flag::H;
        let reg = reg << 1;
        flag |= if reg == 0 { Flag::Z } else { !Flag::Z };
        self.register.set_flag(flag);
        reg
    }

    fn sr(&mut self, reg: u8) -> u8 {
        let temp = reg | 0x80;
        let mut flag = if temp != 0 { Flag::C } else { !Flag::C } | !Flag::N | !Flag::H;
        let reg = ((reg as i8) >> 1) as u8;
        flag |= if reg == 0 { Flag::Z } else { !Flag::Z };
        self.register.set_flag(flag);
        reg
    }

    fn srl(&mut self, reg: u8) -> u8 {
        let temp = reg | 0x80;
        let mut flag = if temp != 0 { Flag::C } else { !Flag::C } | !Flag::N | !Flag::H;
        let reg = reg >> 1;
        flag |= if reg == 0 { Flag::Z } else { !Flag::Z };
        self.register.set_flag(flag);
        reg
    }

}
