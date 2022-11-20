use std::{io::Read, path::PathBuf};

use crate::memory::MemoryIO;

use super::{Cartridge, NoMBC};

impl NoMBC {
    pub fn new() -> Self {
        Self {
            rom_bank: [0; 0x8000],
            ram_bank: [0; 0x2000],
        }
    }

    fn load_rom(&mut self, rom: &Vec<u8>) {
        self.rom_bank.clone_from_slice(&rom[..]);
    }
}

impl MemoryIO for NoMBC {
    fn get8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7fff => self.rom_bank[address as usize],
            0xa000..=0xbfff => self.ram_bank[address as usize],
            _ => 0,
        }
    }

    fn set8(&mut self, address: u16, n: u8) {
        match address {
            0x0000..=0x7fff => self.rom_bank[address as usize] = n,
            0xa000..=0xbfff => self.ram_bank[address as usize] = n,
            _ => (),
        }
    }

    fn get16(&self, address: u16) -> u16 {
        match address {
            0x0000..=0x7fff => unsafe {
                *(self.rom_bank.as_ptr().offset(address as isize) as *const u16)
            },
            0xa000..=0xbfff => unsafe {
                *(self.ram_bank.as_ptr().offset(address as isize) as *const u16)
            },
            _ => 0,
        }
    }

    fn set16(&mut self, address: u16, n: u16) {
        match address {
            0x0000..=0x7fff => unsafe {
                *(self.rom_bank.as_mut_ptr().offset(address as isize) as *mut u16) = n
            },
            0xa000..=0xbfff => unsafe {
                *(self.ram_bank.as_mut_ptr().offset(address as isize) as *mut u16) = n
            },
            _ => (),
        }
    }
}

impl From<Cartridge> for NoMBC {
    fn from(c: Cartridge) -> Self {
        let mut mbc1 = Self::new();
        mbc1.load_rom(&c.content);
        mbc1
    }
}
