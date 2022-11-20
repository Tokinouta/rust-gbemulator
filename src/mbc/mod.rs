use std::{io::Read, path::PathBuf};

mod mbc1;
mod nombc;

pub struct Cartridge {
    content: Vec<u8>,
}

impl Cartridge {
    pub fn new(path: PathBuf) -> std::io::Result<Self> {
        let mut file = std::fs::File::open(path)?;
        let size = file.metadata()?.len() as usize;
        let mut v = vec![0; size];
        file.read(&mut v)?;
        Ok(Self { content: v })
    }

    pub fn header(&self) -> CartridgeHeader {
        CartridgeHeader {
            ch: self.content[0x100..=0x14f].try_into().unwrap(),
        }
    }
}

pub struct CartridgeHeader {
    ch: [u8; 0x50],
}

impl CartridgeHeader {
    pub fn entry_point(&self) -> u32 {
        u32::from_be_bytes(self.ch[0..=3].try_into().unwrap())
    }

    pub fn nintendo_logo(&self) -> [u8; 48] {
        self.ch[0x04..=0x33].try_into().unwrap()
    }

    pub fn title(&self) -> [u8; 16] {
        self.ch[0x34..=0x43].try_into().unwrap()
    }

    pub fn manufacturer_code(&self) -> [u8; 16] {
        self.ch[0x3f..=0x42].try_into().unwrap()
    }

    pub fn cgb_flag(&self) -> bool {
        match self.ch[0x43] {
            0x80 | 0xc0 => true,
            _ => false,
        }
    }

    pub fn new_licensee_code(&self) -> u16 {
        u16::from_be_bytes(self.ch[0x44..=0x45].try_into().unwrap())
    }

    pub fn sgb_flag(&self) -> bool {
        match self.ch[0x46] {
            0x03 => true,
            _ => false,
        }
    }

    pub fn cartridge_type(&self) -> u8 {
        self.ch[0x47]
    }

    pub fn rom_size(&self) -> usize {
        match self.ch[0x48] {
            0x00 => 0x8000,
            0x01 => 0x10000,
            0x02 => 0x20000,
            0x03 => 0x40000,
            0x04 => 0x80000,
            0x05 => 0x100000,
            0x06 => 0x200000,
            0x07 => 0x400000,
            0x08 => 0x800000,
            _ => 0,
        }
    }

    pub fn ram_size(&self) -> usize {
        match self.ch[0x49] {
            0x00 => 0,
            0x01 => 0,
            0x02 => 0x2000,
            0x03 => 0x8000,
            0x04 => 0x20000,
            0x05 => 0x10000,
            _ => 0,
        }
    }

    pub fn destination_code(&self) -> u8 {
        self.ch[0x4a]
    }

    pub fn old_licensee_code(&self) -> u8 {
        self.ch[0x4b]
    }

    pub fn mask_rom_version_number(&self) -> u8 {
        self.ch[0x4c]
    }

    pub fn header_checksum(&self) -> u8 {
        self.ch[0x4d]
    }

    pub fn global_checksum(&self) -> u16 {
        u16::from_be_bytes(self.ch[0x4e..=0x4f].try_into().unwrap())
    }
}

enum BankingMode {
    Simple,
    Advanced,
}

pub struct NoMBC {
    rom_bank: [u8; 0x8000],
    ram_bank: [u8; 0x2000],
}

pub struct MBC1 {
    ram_enable: bool,
    rom_bank_number: u8,
    ram_bank_number: u8,
    banking_mode: BankingMode,

    rom_bank: Vec<u8>,
    ram_bank: Vec<u8>,
}
