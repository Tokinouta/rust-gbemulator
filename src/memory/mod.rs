use crate::{mbc::{Cartridge, CartridgeHeader, NoMBC, MBC1}, interrupt::Interrupt};

pub struct Memory {
    cartridge: Box<dyn MemoryIO>,
    vram: [u8; 0x2000],
    wram: [u8; 0x2000],
    echo_ram: [u8; 0x1dff],
    oam: [u8; 0x9f],
    io_registers: [u8; 0x7f],
    hram: [u8; 0x7e],
    interrupt: Interrupt,
}

pub trait MemoryIO {
    // 读取一个字节
    fn get8(&self, address: u16) -> u8;
    // 写入一个字节
    fn set8(&mut self, address: u16, n: u8);
    // 读取两个字节
    fn get16(&self, address: u16) -> u16;
    // 写入两个字节
    fn set16(&mut self, address: u16, n: u16);
}

impl Memory {
    pub fn new(header: CartridgeHeader) -> Self {
        Self {
            cartridge: match header.cartridge_type() {
                0 => Box::new(NoMBC::new()),
                0x01 | 0x02 | 0x03 => Box::new(MBC1::new(header)),
                _ => Box::new(NoMBC::new()),
            },
            vram: [0; 0x2000],
            wram: [0; 0x2000],
            echo_ram: [0; 0x1dff],
            oam: [0; 0x9f],
            io_registers: [0; 0x7f],
            hram: [0; 0x7e],
            interrupt: Interrupt::new(),
        }
    }
}

impl MemoryIO for Memory {
    fn get8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7fff => self.cartridge.get8(address),
            0x8000..=0x9fff => self.vram[address as usize - 0x8000],
            0xa000..=0xbfff => self.cartridge.get8(address),
            0xc000..=0xcfff => self.wram[address as usize - 0xc000],
            0xd000..=0xdfff => self.wram[address as usize - 0xc000],
            0xe000..=0xfdff => self.echo_ram[address as usize - 0xe000],
            0xfe00..=0xfe9f => self.oam[address as usize - 0xfe00],
            0xfea0..=0xfeff => 0,
            0xff00..=0xff7f => self.io_registers[address as usize - 0xff00],
            0xff80..=0xfffe => self.hram[address as usize - 0xff80],
            0xffff => self.interrupt.enabled,
        }
    }

    fn set8(&mut self, address: u16, n: u8) {
        match address {
            0x0000..=0x7fff => self.cartridge.set8(address, n),
            0x8000..=0x9fff => self.vram[address as usize - 0x8000] = n,
            0xa000..=0xbfff => self.cartridge.set8(address, n),
            0xc000..=0xcfff => self.wram[address as usize - 0xc000] = n,
            0xd000..=0xdfff => self.wram[address as usize - 0xc000] = n,
            0xe000..=0xfdff => self.echo_ram[address as usize - 0xe000] = n,
            0xfe00..=0xfe9f => self.oam[address as usize - 0xfe00] = n,
            0xfea0..=0xfeff => (),
            0xff00..=0xff7f => self.io_registers[address as usize - 0xff00] = n,
            0xff80..=0xfffe => self.hram[address as usize - 0xff80] = n,
            0xffff => self.interrupt.enabled = n,
        }
    }

    fn get16(&self, address: u16) -> u16 {
        // (u16::from(self.mem[address as usize + 1]) << 8) | u16::from(self.mem[address as usize])
        match address {
            0x0000..=0x7fff => self.cartridge.get16(address),
            0x8000..=0x9fff => unsafe {
                *(self.vram.as_ptr().offset(address as isize - 0x8000) as *const u16)
            },
            0xa000..=0xbfff => self.cartridge.get16(address),
            0xc000..=0xcfff => unsafe {
                *(self.wram.as_ptr().offset(address as isize - 0xc000) as *const u16)
            },
            0xd000..=0xdfff => unsafe {
                *(self.wram.as_ptr().offset(address as isize - 0xc000) as *const u16)
            },
            0xe000..=0xfdff => unsafe {
                *(self.echo_ram.as_ptr().offset(address as isize - 0xe000) as *const u16)
            },
            0xfe00..=0xfe9f => unsafe {
                *(self.oam.as_ptr().offset(address as isize - 0xfe00) as *const u16)
            },
            0xfea0..=0xfeff => 0,
            0xff00..=0xff7f => unsafe {
                *(self.io_registers.as_ptr().offset(address as isize - 0xff00) as *const u16)
            },
            0xff80..=0xfffe => unsafe {
                *(self.hram.as_ptr().offset(address as isize - 0xff80) as *const u16)
            },
            0xffff => 0,
        }
    }

    fn set16(&mut self, address: u16, n: u16) {
        // self.mem[address as usize] = (n >> 8) as u8;
        // self.mem[address as usize + 1] = (n & 0x00ff) as u8;
        match address {
            0x0000..=0x7fff => self.cartridge.set16(address, n),
            0x8000..=0x9fff => unsafe {
                *(self.vram.as_mut_ptr().offset(address as isize - 0x8000) as *mut u16) = n;
            },
            0xa000..=0xbfff => self.cartridge.set16(address, n),
            0xc000..=0xcfff => unsafe {
                *(self.wram.as_mut_ptr().offset(address as isize - 0xc000) as *mut u16) = n;
            },
            0xd000..=0xdfff => unsafe {
                *(self.wram.as_mut_ptr().offset(address as isize - 0xc000) as *mut u16) = n;
            },
            0xe000..=0xfdff => unsafe {
                *(self.echo_ram.as_mut_ptr().offset(address as isize - 0xe000) as *mut u16) = n;
            },
            0xfe00..=0xfe9f => unsafe {
                *(self.oam.as_mut_ptr().offset(address as isize - 0xfe00) as *mut u16) = n;
            },
            0xfea0..=0xfeff => (),
            0xff00..=0xff7f => unsafe {
                *(self
                    .io_registers
                    .as_mut_ptr()
                    .offset(address as isize - 0xff00) as *mut u16) = n;
            },
            0xff80..=0xfffe => unsafe {
                *(self.hram.as_mut_ptr().offset(address as isize - 0xff80) as *mut u16) = n;
            },
            0xffff => (),
        };
    }
}
