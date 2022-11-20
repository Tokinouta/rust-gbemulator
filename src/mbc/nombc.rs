use crate::memory::MemoryIO;

use super::NoMBC;

impl NoMBC {
    pub fn new() -> Self {
        Self {
            rom_bank: [0; 0x8000],
            ram_bank: [0; 0x2000],
        }
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
