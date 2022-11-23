use std::sync::Arc;

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

            // JP nn
            0xc3 => self.jump(),
            // JP cc, nn
            0xc2 => self.jump_ncondition(Flag::Z),
            0xca => self.jump_condition(Flag::Z),
            0xd2 => self.jump_ncondition(Flag::C),
            0xda => self.jump_condition(Flag::C),
            // JP (HL)
            0xe9 => self.jump_register(),
            // JR n
            0x18 => self.jump_relative(),
            // JR cc, n
            0x20 => self.jump_relative_ncondition(Flag::Z),
            0x28 => self.jump_relative_condition(Flag::Z),
            0x30 => self.jump_relative_ncondition(Flag::C),
            0x38 => self.jump_relative_condition(Flag::C),
            
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

                    // BIT 0, r
                    0x47 => self.bit(self.register.a, 0),
                    0x40 => self.bit(self.register.b, 0),
                    0x41 => self.bit(self.register.c, 0),
                    0x42 => self.bit(self.register.d, 0),
                    0x43 => self.bit(self.register.e, 0),
                    0x44 => self.bit(self.register.h, 0),
                    0x45 => self.bit(self.register.l, 0),
                    0x46 => {
                        let temp = self.memory.get8(self.register.get_hl());
                        self.bit(temp, 0);
                    }

                    // BIT 1, r
                    0x4f => self.bit(self.register.a, 1),
                    0x48 => self.bit(self.register.b, 1),
                    0x49 => self.bit(self.register.c, 1),
                    0x4a => self.bit(self.register.d, 1),
                    0x4b => self.bit(self.register.e, 1),
                    0x4c => self.bit(self.register.h, 1),
                    0x4d => self.bit(self.register.l, 1),
                    0x4e => {
                        let temp = self.memory.get8(self.register.get_hl());
                        self.bit(temp, 1);
                    }

                    // BIT 2, r
                    0x57 => self.bit(self.register.a, 2),
                    0x50 => self.bit(self.register.b, 2),
                    0x51 => self.bit(self.register.c, 2),
                    0x52 => self.bit(self.register.d, 2),
                    0x53 => self.bit(self.register.e, 2),
                    0x54 => self.bit(self.register.h, 2),
                    0x55 => self.bit(self.register.l, 2),
                    0x56 => {
                        let temp = self.memory.get8(self.register.get_hl());
                        self.bit(temp, 2);
                    }

                    // BIT 3, r
                    0x5f => self.bit(self.register.a, 3),
                    0x58 => self.bit(self.register.b, 3),
                    0x59 => self.bit(self.register.c, 3),
                    0x5a => self.bit(self.register.d, 3),
                    0x5b => self.bit(self.register.e, 3),
                    0x5c => self.bit(self.register.h, 3),
                    0x5d => self.bit(self.register.l, 3),
                    0x5e => {
                        let temp = self.memory.get8(self.register.get_hl());
                        self.bit(temp, 3);
                    }

                    // BIT 4, r
                    0x67 => self.bit(self.register.a, 4),
                    0x60 => self.bit(self.register.b, 4),
                    0x61 => self.bit(self.register.c, 4),
                    0x62 => self.bit(self.register.d, 4),
                    0x63 => self.bit(self.register.e, 4),
                    0x64 => self.bit(self.register.h, 4),
                    0x65 => self.bit(self.register.l, 4),
                    0x66 => {
                        let temp = self.memory.get8(self.register.get_hl());
                        self.bit(temp, 4);
                    }

                    // BIT 5, r
                    0x6f => self.bit(self.register.a, 5),
                    0x68 => self.bit(self.register.b, 5),
                    0x69 => self.bit(self.register.c, 5),
                    0x6a => self.bit(self.register.d, 5),
                    0x6b => self.bit(self.register.e, 5),
                    0x6c => self.bit(self.register.h, 5),
                    0x6d => self.bit(self.register.l, 5),
                    0x6e => {
                        let temp = self.memory.get8(self.register.get_hl());
                        self.bit(temp, 5);
                    }

                    // BIT 6, r
                    0x77 => self.bit(self.register.a, 6),
                    0x70 => self.bit(self.register.b, 6),
                    0x71 => self.bit(self.register.c, 6),
                    0x72 => self.bit(self.register.d, 6),
                    0x73 => self.bit(self.register.e, 6),
                    0x74 => self.bit(self.register.h, 6),
                    0x75 => self.bit(self.register.l, 6),
                    0x76 => {
                        let temp = self.memory.get8(self.register.get_hl());
                        self.bit(temp, 6);
                    }

                    // BIT 7, r
                    0x7f => self.bit(self.register.a, 7),
                    0x78 => self.bit(self.register.b, 7),
                    0x79 => self.bit(self.register.c, 7),
                    0x7a => self.bit(self.register.d, 7),
                    0x7b => self.bit(self.register.e, 7),
                    0x7c => self.bit(self.register.h, 7),
                    0x7d => self.bit(self.register.l, 7),
                    0x7e => {
                        let temp = self.memory.get8(self.register.get_hl());
                        self.bit(temp, 7);
                    }

                    // SET 0, r
                    0xc7 => self.register.a = self.set(self.register.a, 0),
                    0xc0 => self.register.b = self.set(self.register.b, 0),
                    0xc1 => self.register.c = self.set(self.register.c, 0),
                    0xc2 => self.register.d = self.set(self.register.d, 0),
                    0xc3 => self.register.e = self.set(self.register.e, 0),
                    0xc4 => self.register.h = self.set(self.register.h, 0),
                    0xc5 => self.register.l = self.set(self.register.l, 0),
                    0xc6 => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.set(temp, 0);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // SET 1, r
                    0xcf => self.register.a = self.set(self.register.a, 1),
                    0xc8 => self.register.b = self.set(self.register.b, 1),
                    0xc9 => self.register.c = self.set(self.register.c, 1),
                    0xca => self.register.d = self.set(self.register.d, 1),
                    0xcb => self.register.e = self.set(self.register.e, 1),
                    0xcc => self.register.h = self.set(self.register.h, 1),
                    0xcd => self.register.l = self.set(self.register.l, 1),
                    0xce => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.set(temp, 1);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // SET 2, r
                    0xd7 => self.register.a = self.set(self.register.a, 2),
                    0xd0 => self.register.b = self.set(self.register.b, 2),
                    0xd1 => self.register.c = self.set(self.register.c, 2),
                    0xd2 => self.register.d = self.set(self.register.d, 2),
                    0xd3 => self.register.e = self.set(self.register.e, 2),
                    0xd4 => self.register.h = self.set(self.register.h, 2),
                    0xd5 => self.register.l = self.set(self.register.l, 2),
                    0xd6 => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.set(temp, 2);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // SET 3, r
                    0xdf => self.register.a = self.set(self.register.a, 3),
                    0xd8 => self.register.b = self.set(self.register.b, 3),
                    0xd9 => self.register.c = self.set(self.register.c, 3),
                    0xda => self.register.d = self.set(self.register.d, 3),
                    0xdb => self.register.e = self.set(self.register.e, 3),
                    0xdc => self.register.h = self.set(self.register.h, 3),
                    0xdd => self.register.l = self.set(self.register.l, 3),
                    0xde => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.set(temp, 3);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // SET 4, r
                    0xe7 => self.register.a = self.set(self.register.a, 4),
                    0xe0 => self.register.b = self.set(self.register.b, 4),
                    0xe1 => self.register.c = self.set(self.register.c, 4),
                    0xe2 => self.register.d = self.set(self.register.d, 4),
                    0xe3 => self.register.e = self.set(self.register.e, 4),
                    0xe4 => self.register.h = self.set(self.register.h, 4),
                    0xe5 => self.register.l = self.set(self.register.l, 4),
                    0xe6 => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.set(temp, 4);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // SET 5, r
                    0xef => self.register.a = self.set(self.register.a, 5),
                    0xe8 => self.register.b = self.set(self.register.b, 5),
                    0xe9 => self.register.c = self.set(self.register.c, 5),
                    0xea => self.register.d = self.set(self.register.d, 5),
                    0xeb => self.register.e = self.set(self.register.e, 5),
                    0xec => self.register.h = self.set(self.register.h, 5),
                    0xed => self.register.l = self.set(self.register.l, 5),
                    0xee => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.set(temp, 5);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // SET 6, r
                    0xf7 => self.register.a = self.set(self.register.a, 6),
                    0xf0 => self.register.b = self.set(self.register.b, 6),
                    0xf1 => self.register.c = self.set(self.register.c, 6),
                    0xf2 => self.register.d = self.set(self.register.d, 6),
                    0xf3 => self.register.e = self.set(self.register.e, 6),
                    0xf4 => self.register.h = self.set(self.register.h, 6),
                    0xf5 => self.register.l = self.set(self.register.l, 6),
                    0xf6 => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.set(temp, 6);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // SET 7, r
                    0xff => self.register.a = self.set(self.register.a, 7),
                    0xf8 => self.register.b = self.set(self.register.b, 7),
                    0xf9 => self.register.c = self.set(self.register.c, 7),
                    0xfa => self.register.d = self.set(self.register.d, 7),
                    0xfb => self.register.e = self.set(self.register.e, 7),
                    0xfc => self.register.h = self.set(self.register.h, 7),
                    0xfd => self.register.l = self.set(self.register.l, 7),
                    0xfe => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.set(temp, 7);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // RES 0, r
                    0x87 => self.register.a = self.reset(self.register.a, 0),
                    0x80 => self.register.b = self.reset(self.register.b, 0),
                    0x81 => self.register.c = self.reset(self.register.c, 0),
                    0x82 => self.register.d = self.reset(self.register.d, 0),
                    0x83 => self.register.e = self.reset(self.register.e, 0),
                    0x84 => self.register.h = self.reset(self.register.h, 0),
                    0x85 => self.register.l = self.reset(self.register.l, 0),
                    0x86 => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.reset(temp, 0);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // RES 1, r
                    0x8f => self.register.a = self.reset(self.register.a, 1),
                    0x88 => self.register.b = self.reset(self.register.b, 1),
                    0x89 => self.register.c = self.reset(self.register.c, 1),
                    0x8a => self.register.d = self.reset(self.register.d, 1),
                    0x8b => self.register.e = self.reset(self.register.e, 1),
                    0x8c => self.register.h = self.reset(self.register.h, 1),
                    0x8d => self.register.l = self.reset(self.register.l, 1),
                    0x8e => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.reset(temp, 1);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // RES 2, r
                    0x97 => self.register.a = self.reset(self.register.a, 2),
                    0x90 => self.register.b = self.reset(self.register.b, 2),
                    0x91 => self.register.c = self.reset(self.register.c, 2),
                    0x92 => self.register.d = self.reset(self.register.d, 2),
                    0x93 => self.register.e = self.reset(self.register.e, 2),
                    0x94 => self.register.h = self.reset(self.register.h, 2),
                    0x95 => self.register.l = self.reset(self.register.l, 2),
                    0x96 => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.reset(temp, 2);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // RES 3, r
                    0x9f => self.register.a = self.reset(self.register.a, 3),
                    0x98 => self.register.b = self.reset(self.register.b, 3),
                    0x99 => self.register.c = self.reset(self.register.c, 3),
                    0x9a => self.register.d = self.reset(self.register.d, 3),
                    0x9b => self.register.e = self.reset(self.register.e, 3),
                    0x9c => self.register.h = self.reset(self.register.h, 3),
                    0x9d => self.register.l = self.reset(self.register.l, 3),
                    0x9e => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.reset(temp, 3);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // RES 4, r
                    0xa7 => self.register.a = self.reset(self.register.a, 4),
                    0xa0 => self.register.b = self.reset(self.register.b, 4),
                    0xa1 => self.register.c = self.reset(self.register.c, 4),
                    0xa2 => self.register.d = self.reset(self.register.d, 4),
                    0xa3 => self.register.e = self.reset(self.register.e, 4),
                    0xa4 => self.register.h = self.reset(self.register.h, 4),
                    0xa5 => self.register.l = self.reset(self.register.l, 4),
                    0xa6 => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.reset(temp, 4);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // RES 5, r
                    0xaf => self.register.a = self.reset(self.register.a, 5),
                    0xa8 => self.register.b = self.reset(self.register.b, 5),
                    0xa9 => self.register.c = self.reset(self.register.c, 5),
                    0xaa => self.register.d = self.reset(self.register.d, 5),
                    0xab => self.register.e = self.reset(self.register.e, 5),
                    0xac => self.register.h = self.reset(self.register.h, 5),
                    0xad => self.register.l = self.reset(self.register.l, 5),
                    0xae => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.reset(temp, 5);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // RES 6, r
                    0xb7 => self.register.a = self.reset(self.register.a, 6),
                    0xb0 => self.register.b = self.reset(self.register.b, 6),
                    0xb1 => self.register.c = self.reset(self.register.c, 6),
                    0xb2 => self.register.d = self.reset(self.register.d, 6),
                    0xb3 => self.register.e = self.reset(self.register.e, 6),
                    0xb4 => self.register.h = self.reset(self.register.h, 6),
                    0xb5 => self.register.l = self.reset(self.register.l, 6),
                    0xb6 => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.reset(temp, 6);
                        self.memory.set8(self.register.get_hl(), temp);
                    }

                    // RES 7, r
                    0xbf => self.register.a = self.reset(self.register.a, 7),
                    0xb8 => self.register.b = self.reset(self.register.b, 7),
                    0xb9 => self.register.c = self.reset(self.register.c, 7),
                    0xba => self.register.d = self.reset(self.register.d, 7),
                    0xbb => self.register.e = self.reset(self.register.e, 7),
                    0xbc => self.register.h = self.reset(self.register.h, 7),
                    0xbd => self.register.l = self.reset(self.register.l, 7),
                    0xbe => {
                        let mut temp = self.memory.get8(self.register.get_hl());
                        temp = self.reset(temp, 7);
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

    fn bit(&mut self, reg: u8, b: u8) {
        let flag = if reg & (1 << b) == 0 {
            Flag::Z
        } else {
            !Flag::Z
        } | !Flag::N
            | Flag::H;
        self.register.set_flag(flag);
    }

    fn set(&mut self, reg: u8, b: u8) -> u8 {
        reg | (1 << b)
    }

    fn reset(&mut self, reg: u8, b: u8) -> u8 {
        reg & !(1 << b)
    }

    fn jump(&mut self) {
        let address = self.imm16();
        self.register.pc = address;
    }

    fn jump_condition(&mut self, condition: Flag) {
        if self.register.get_flag(condition) {
            self.jump();
        }
    }

    fn jump_ncondition(&mut self, condition: Flag) {
        if !self.register.get_flag(condition) {
            self.jump();
        }
    }

    fn jump_register(&mut self) {
        let reg = self.register.get_hl();
        let address = self.memory.get16(reg);
        self.register.pc = address;
    }

    fn jump_relative(&mut self) {
        let displacement = self.imm8();
        self.register.pc_inc(displacement as i8 as i16);
    }

    fn jump_relative_condition(&mut self, condition: Flag) {
        if self.register.get_flag(condition) {
            self.jump_relative();
        }
    }

    fn jump_relative_ncondition(&mut self, condition: Flag) {
        if !self.register.get_flag(condition) {
            self.jump_relative();
        }
    }
}
