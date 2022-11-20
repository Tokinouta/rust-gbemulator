use crate::memory::MemoryIO;

use super::{BankingMode, Cartridge, CartridgeHeader, MBC1};

impl MBC1 {
    pub fn new(header: CartridgeHeader) -> Self {
        Self {
            ram_enable: false,
            rom_bank_number: 1,
            ram_bank_number: 0,
            banking_mode: BankingMode::Simple,
            rom_bank: Vec::with_capacity(header.rom_size()),
            ram_bank: Vec::with_capacity(header.ram_size()),
        }
    }

    fn load_rom(&mut self, rom: &Vec<u8>) {
        self.rom_bank.clone_from(rom);
    }

    fn set_ram_enable(&mut self, is_enabled: u8) {
        match is_enabled {
            0x00 => self.ram_enable = false,
            0x0a => self.ram_enable = true,
            _ => (),
        }
    }

    fn set_rom_bank(&mut self, bank: u8) {
        match bank & 0x1f {
            0x00 | 0x01 => self.rom_bank_number = 0x01,
            0x02..=0x1f => self.rom_bank_number = bank,
            _ => (),
        }
        // match self.banking_mode {
        //     BankingMode::Simple => (),
        //     BankingMode::Advanced => self.rom_bank_number += (self.ram_bank_number & 0x03) << 5,
        // }
    }

    fn set_ram_bank(&mut self, bank: u8) {
        match bank & 0x1f {
            0x00..=0x03 => self.rom_bank_number = bank,
            _ => (),
        }
    }

    fn set_banking_mode(&mut self, mode: u8) {
        match mode {
            0x00 => self.banking_mode = BankingMode::Simple,
            0x01 => self.banking_mode = BankingMode::Advanced,
            _ => (),
        }
    }
}

impl MemoryIO for MBC1 {
    fn get8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3fff => match self.banking_mode {
                BankingMode::Simple => self.rom_bank[address as usize & 0x3fff],
                BankingMode::Advanced => {
                    self.rom_bank[address as usize & 0x3fff + (self.ram_bank_number as usize) << 19]
                }
            },
            0x4000..=0x7fff => {
                self.rom_bank[(address as usize & 0x3fff)
                    + ((self.rom_bank_number as usize) << 14)
                    + ((self.ram_bank_number as usize) << 19)]
            }
            0xa000..=0xbfff => match self.banking_mode {
                BankingMode::Simple => self.ram_bank[address as usize & 0x1fff],
                BankingMode::Advanced => {
                    self.ram_bank[address as usize & 0x1fff + (self.ram_bank_number as usize) << 13]
                }
            },
            _ => 0,
        }
    }

    fn set8(&mut self, address: u16, n: u8) {
        match address {
            0x0000..=0x1fff => self.set_ram_enable(n),
            0x2000..=0x3fff => self.set_rom_bank(n),
            0x4000..=0x5fff => self.set_ram_bank(n),
            0x6000..=0x7fff => self.set_banking_mode(n),
            0xa000..=0xbfff => match self.banking_mode {
                BankingMode::Simple => self.ram_bank[address as usize & 0x1fff] = n,
                BankingMode::Advanced => {
                    self.ram_bank
                        [address as usize & 0x1fff + (self.ram_bank_number as usize) << 13] = n
                }
            },
            _ => (),
        }
    }

    fn get16(&self, address: u16) -> u16 {
        match address {
            0x0000..=0x3fff => match self.banking_mode {
                BankingMode::Simple => unsafe {
                    let address = (address as usize & 0x3fff) as isize;
                    *(self.rom_bank.as_ptr().offset(address) as *const u16)
                },
                BankingMode::Advanced => unsafe {
                    let address = (address as usize
                        & 0x3fff + (self.ram_bank_number as usize) << 19)
                        as isize;
                    *(self.rom_bank.as_ptr().offset(address) as *const u16)
                },
            },
            0x4000..=0x7fff => unsafe {
                let address =
                    (address as usize & 0x3fff + (self.rom_bank_number as usize) << 14) as isize;
                *(self.rom_bank.as_ptr().offset(address) as *const u16)
            },
            0xa000..=0xbfff => match self.banking_mode {
                BankingMode::Simple => unsafe {
                    let address = (address as usize & 0x1fff) as isize;
                    *(self.ram_bank.as_ptr().offset(address) as *const u16)
                },
                BankingMode::Advanced => unsafe {
                    let address = (address as usize
                        & 0x1fff + (self.ram_bank_number as usize) << 13)
                        as isize;
                    *(self.ram_bank.as_ptr().offset(address) as *const u16)
                },
            },
            _ => 0,
        }
    }

    fn set16(&mut self, address: u16, n: u16) {
        match address {
            0xa000..=0xbfff => match self.banking_mode {
                BankingMode::Simple => unsafe {
                    let address = (address as usize & 0x1fff) as isize;
                    *(self.ram_bank.as_mut_ptr().offset(address as isize) as *mut u16) = n
                },
                BankingMode::Advanced => unsafe {
                    let address = (address as usize
                        & 0x1fff + (self.ram_bank_number as usize) << 13)
                        as isize;
                    *(self.ram_bank.as_mut_ptr().offset(address as isize) as *mut u16) = n
                },
            },
            _ => (),
        }
    }
}

impl From<Cartridge> for MBC1 {
    fn from(c: Cartridge) -> Self {
        let header = c.header();
        let mut mbc1 = Self::new(header);
        mbc1.load_rom(&c.content);
        mbc1
    }
}
