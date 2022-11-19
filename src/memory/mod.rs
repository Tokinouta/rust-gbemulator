pub struct Memory {
    mem: [u8; 0x10000],
}

impl Memory {
    pub fn new() -> Self {
        Self { mem: [0; 0x10000] }
    }

    pub fn get8(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    pub fn set8(&mut self, address: u16, n: u8) {
        self.mem[address as usize] = n;
    }

    pub fn get16(&self, address: u16) -> u16 {
        // (u16::from(self.mem[address as usize + 1]) << 8) | u16::from(self.mem[address as usize])
        unsafe { *(self.mem.as_ptr().offset(address as isize) as *const u16) }
    }

    pub fn set16(&mut self, address: u16, n: u16) {
        // self.mem[address as usize] = (n >> 8) as u8;
        // self.mem[address as usize + 1] = (n & 0x00ff) as u8;
        unsafe { *(self.mem.as_mut_ptr().offset(address as isize) as *mut u16) = n };
    }
}
