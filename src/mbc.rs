struct CartridgeHeader {
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
    
    pub fn cgb_flag(&self) -> u8 {
        self.ch[0x43]
    }
    
    pub fn new_licensee_code(&self) -> u16 {
        u16::from_be_bytes(self.ch[0x44..=0x45].try_into().unwrap())
    }
    
    pub fn sgb_flag(&self) -> u8 {
        self.ch[0x46]
    }
    
    pub fn cartridge_type(&self) -> u8 {
        self.ch[0x47]
    }
    
    pub fn rom_size(&self) -> u8 {
        self.ch[0x48]
    }
    
    pub fn ram_size(&self) -> u8 {
        self.ch[0x49]
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