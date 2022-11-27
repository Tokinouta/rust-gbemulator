// TODO: I think i should finish writing the CPU, the interrupt system before starting the GPU. So this part will be continued after I finished the parts above.

// Actually, I lack a lot of necessary knowledge about a computer system, and that is the hardest part which always hampers me from smoothly writing.

use std::{cell::RefCell, rc::Rc};

use crate::{
    interrupt::{IntFlag, Interrupt},
    memory::MemoryIO,
};

#[derive(Eq, PartialEq)]
pub enum HdmaMode {
    /// When using this transfer method, all data is transferred at once. The execution of the program is halted until
    /// the transfer has completed. Note that the General Purpose DMA blindly attempts to copy the data, even if the
    /// CD controller is currently accessing VRAM. So General Purpose DMA should be used only if the Display is disabled,
    /// or during V-Blank, or (for rather short blocks) during H-Blank. The execution of the program continues when the
    /// transfer has been completed, and FF55 then contains a value of FFh.
    Gdma,
    /// The H-Blank DMA transfers 10h bytes of data during each H-Blank, ie. at LY=0-143, no data is transferred during
    /// V-Blank (LY=144-153), but the transfer will then continue at LY=00. The execution of the program is halted
    /// during the separate transfers, but the program execution continues during the 'spaces' between each data block.
    /// Note that the program should not change the Destination VRAM bank (FF4F), or the Source ROM/RAM bank (in case
    /// data is transferred from bankable memory) until the transfer has completed! (The transfer should be paused as
    /// described below while the banks are switched) Reading from Register FF55 returns the remaining length (divided
    /// by 10h, minus 1), a value of 0FFh indicates that the transfer has completed. It is also possible to terminate
    /// an active H-Blank transfer by writing zero to Bit 7 of FF55. In that case reading from FF55 will return how many
    /// $10 "blocks" remained (minus 1) in the lower 7 bits, but Bit 7 will be read as "1". Stopping the transfer
    /// doesn't set HDMA1-4 to $FF.
    Hdma,
}

pub struct Hdma {
    /// These two registers specify the address at which the transfer will read data from. Normally, this should be
    /// either in ROM, SRAM or WRAM, thus either in range 0000-7FF0 or A000-DFF0. [Note : this has yet to be tested on
    /// Echo RAM, OAM, FEXX, IO and HRAM]. Trying to specify a source address in VRAM will cause garbage to be copied.
    /// The four lower bits of this address will be ignored and treated as 0.
    pub source: u16,
    /// These two registers specify the address within 8000-9FF0 to which the data will be copied. Only bits 12-4 are
    /// respected; others are ignored. The four lower bits of this address will be ignored and treated as 0.
    pub destination: u16,
    pub active: bool,
    pub mode: HdmaMode,
    pub remain: u8,
}

impl Hdma {
    pub fn new() -> Self {
        Self {
            source: 0x0000,
            destination: 0x8000,
            active: false,
            mode: HdmaMode::Gdma,
            remain: 0x00,
        }
    }
}

impl MemoryIO for Hdma {
    fn get8(&self, a: u16) -> u8 {
        match a {
            0xff51 => (self.source >> 8) as u8,
            0xff52 => self.source as u8,
            0xff53 => (self.destination >> 8) as u8,
            0xff54 => self.destination as u8,
            0xff55 => self.remain | if self.active { 0x00 } else { 0x80 },
            _ => panic!(""),
        }
    }

    fn set8(&mut self, a: u16, v: u8) {
        match a {
            0xff51 => self.source = (u16::from(v) << 8) | (self.source & 0x00ff),
            0xff52 => self.source = (self.source & 0xff00) | u16::from(v & 0xf0),
            0xff53 => {
                self.destination = 0x8000 | (u16::from(v & 0x1f) << 8) | (self.destination & 0x00ff)
            }
            0xff54 => self.destination = (self.destination & 0xff00) | u16::from(v & 0xf0),
            0xff55 => {
                if self.active && self.mode == HdmaMode::Hdma {
                    if v & 0x80 == 0x00 {
                        self.active = false;
                    };
                    return;
                }
                self.active = true;
                self.remain = v & 0x7f;
                self.mode = if v & 0x80 != 0x00 {
                    HdmaMode::Hdma
                } else {
                    HdmaMode::Gdma
                };
            }
            _ => panic!(""),
        };
    }

    fn get16(&self, _: u16) -> u16 {
        unimplemented!("HDMA doesn't support reading a 2-byte data.")
    }

    fn set16(&mut self, _: u16, _: u16) {
        unimplemented!("HDMA doesn't support writing a 2-byte data.")
    }
}

/// LCDC is the main LCD Control register. Its bits toggle what elements are displayed on the screen, and how.
struct LcdControl {
    // lcd_control: u8,
    pub lcd_and_ppu_enable: bool,
    pub window_tile_map_area: u8,
    pub window_enable: bool,
    pub bg_and_window_tile_data_area: u8,
    pub bg_tile_map_area: u8,
    pub obj_size: bool,
    pub obj_enable: bool,
    pub bg_and_window_enable: bool,
}

impl LcdControl {
    pub fn new() -> Self {
        Self {
            lcd_and_ppu_enable: true,
            window_tile_map_area: 0,
            window_enable: true,
            bg_and_window_tile_data_area: 0,
            bg_tile_map_area: 0,
            obj_size: true,
            obj_enable: true,
            bg_and_window_enable: true,
        }
    }
}

impl MemoryIO for LcdControl {
    fn get8(&self, _: u16) -> u8 {
        let mut res = 0;
        if self.lcd_and_ppu_enable {
            res |= 0x80;
        }
        if self.window_tile_map_area == 1 {
            res |= 0x40;
        }
        if self.window_enable {
            res |= 0x20;
        }
        if self.bg_and_window_tile_data_area == 1 {
            res |= 0x10;
        }
        if self.bg_tile_map_area == 1 {
            res |= 0x08;
        }
        if self.obj_size {
            res |= 0x04;
        }
        if self.obj_enable {
            res |= 0x02;
        }
        if self.bg_and_window_enable {
            res |= 0x01;
        }
        res
    }

    fn set8(&mut self, _: u16, n: u8) {
        self.lcd_and_ppu_enable = n & 0x80 != 0;
        self.window_tile_map_area = (n & 0x40) >> 6;
        self.window_enable = n & 0x20 != 0;
        self.bg_and_window_tile_data_area = (n & 0x10) >> 4;
        self.bg_tile_map_area = (n & 0x08) >> 3;
        self.obj_size = n & 0x04 != 0;
        self.obj_enable = n & 0x02 != 0;
        self.bg_and_window_enable = n & 0x01 != 0;
    }

    fn get16(&self, _: u16) -> u16 {
        unimplemented!()
    }

    fn set16(&mut self, _: u16, _: u16) {
        unimplemented!()
    }
}
/// LCD Status
///
/// - LYC=LY STAT Interrupt source         (1=Enable) (Read/Write)
/// - Bit 5 - Mode 2 OAM STAT Interrupt source     (1=Enable) (Read/Write)
/// - Bit 4 - Mode 1 VBlank STAT Interrupt source  (1=Enable) (Read/Write)
/// - Bit 3 - Mode 0 HBlank STAT Interrupt source  (1=Enable) (Read/Write)
/// - Bit 2 - LYC=LY Flag                          (0=Different, 1=Equal) (Read Only)
/// - Bit 1-0 - Mode Flag
struct LcdStatus {
    // data: u8,
    pub current_line_interrupt: bool,
    pub is_mode2_interrupt_enabled: bool,
    pub is_mode1_interrupt_enabled: bool,
    pub is_mode0_interrupt_enabled: bool,
    pub current_line_flag: bool,
    pub mode: u8,
}

impl LcdStatus {
    pub fn new() -> Self {
        Self {
            current_line_interrupt: true,
            is_mode2_interrupt_enabled: true,
            is_mode1_interrupt_enabled: true,
            is_mode0_interrupt_enabled: true,
            current_line_flag: true,
            mode: 0,
        }
    }
}

impl MemoryIO for LcdStatus {
    fn get8(&self, _: u16) -> u8 {
        let mut res = 0;
        if self.current_line_interrupt {
            res |= 0x80;
        }
        if self.is_mode2_interrupt_enabled {
            res |= 0x40;
        }
        if self.is_mode1_interrupt_enabled {
            res |= 0x20;
        }
        if self.is_mode0_interrupt_enabled {
            res |= 0x10;
        }
        if self.current_line_flag {
            res |= 0x08;
        }
        res |= self.mode;
        res
    }

    fn set8(&mut self, _: u16, n: u8) {
        self.current_line_interrupt = n & 0x80 != 0;
        self.is_mode2_interrupt_enabled = n & 0x40 != 0;
        self.is_mode1_interrupt_enabled = n & 0x20 != 0;
        self.is_mode0_interrupt_enabled = n & 0x10 != 0;
        self.current_line_flag = n & 0x08 != 0;
        self.mode = n & 0x03;
    }

    fn get16(&self, _: u16) -> u16 {
        unimplemented!()
    }

    fn set16(&mut self, _: u16, _: u16) {
        unimplemented!()
    }
}

/// The Game Boy PPU can display up to 40 sprites either in 8x8 or in 8x16 pixels. Because of a limitation of
/// hardware, only ten sprites can be displayed per scan line. Sprite tiles have the same format as BG tiles,
/// but they are taken from the Sprite Tiles Table located at $8000-8FFF and have unsigned numbering.
///
/// Sprite attributes reside in the Sprite Attribute Table (OAM: Object Attribute Memory) at $FE00-FE9F. Each
/// of the 40 entries consists of four bytes.
#[derive(Clone, Copy, Default)]
struct OAMEntry {
    y_position: u8,
    x_position: u8,
    tile_index: u8,
    flags: u8,
}

impl OAMEntry {
    fn bg_over_obj(&self) -> bool {
        self.flags & 0x80 != 0
    }

    fn y_flip(&self) -> bool {
        self.flags & 0x40 != 0
    }

    fn x_flip(&self) -> bool {
        self.flags & 0x20 != 0
    }

    fn palette_number(&self) -> u8 {
        (self.flags & 0x10) >> 4
    }

    fn vram_bank(&self) -> u8 {
        (self.flags & 0x08) >> 3
    }

    fn palette_number_cgb(&self) -> u8 {
        self.flags & 0x07
    }
}

impl MemoryIO for OAMEntry {
    fn get8(&self, address: u16) -> u8 {
        match address & 0x03 {
            0x00 => self.y_position,
            0x01 => self.x_position,
            0x02 => self.tile_index,
            0x03 => self.flags,
            _ => unimplemented!(),
        }
    }

    fn set8(&mut self, address: u16, n: u8) {
        match address & 0x03 {
            0x00 => self.y_position = n,
            0x01 => self.x_position = n,
            0x02 => self.tile_index = n,
            0x03 => self.flags = n,
            _ => unimplemented!(),
        }
    }

    fn get16(&self, _: u16) -> u16 {
        unimplemented!()
    }

    fn set16(&mut self, _: u16, _: u16) {
        unimplemented!()
    }
}

impl MemoryIO for [OAMEntry] {
    fn get8(&self, address: u16) -> u8 {
        self[(address - 0xfe00) as usize >> 2].get8(address)
    }

    fn set8(&mut self, address: u16, n: u8) {
        self[(address - 0xfe00) as usize >> 2].set8(address, n);
    }

    fn get16(&self, address: u16) -> u16 {
        unimplemented!()
    }

    fn set16(&mut self, address: u16, n: u16) {
        unimplemented!()
    }
}

/// LCD Color Palettes (CGB only)
///
/// The CGB has a small amount of RAM used to store its color palettes. Unlike most of the hardware interface,
/// palette RAM (or CRAM for Color RAM) is not accessed directly, but instead through the following registers
struct ColorPalette {
    index: usize,
    auto_increment: bool,
    data: [u16; 32],
}

impl ColorPalette {
    pub fn new() -> Self {
        Self {
            index: 0,
            auto_increment: false,
            data: [0; 32],
        }
    }

    pub fn get_color(&self) -> (u8, u8, u8) {
        let r = self.data[self.index] & 0x1f;
        let g = self.data[self.index] & 0x3e;
        let b = self.data[self.index] & 0x7c;
        (r as u8, g as u8, b as u8)
    }

    pub fn set_color(&mut self, color: (u8, u8, u8)) {
        let mut new = 0u16;
        new |= (color.0 & 0x1f) as u16;
        new |= ((color.1 & 0x1f) as u16) << 5;
        new |= ((color.2 & 0x1f) as u16) << 10;
        self.data[self.index] = new;
    }
}

impl MemoryIO for ColorPalette {
    fn get8(&self, address: u16) -> u8 {
        match address & 0x01 {
            0x00 => self.index as u8 & 0x3f | if self.auto_increment { 0x80 } else { 0 },
            0x01 => {
                let i = self.index >> 1;
                let mask: u16 = if self.index & 1 == 0 { 0x00ff } else { 0xff00 };
                (self.data[i] & mask) as u8
            }
            _ => unimplemented!(),
        }
    }

    fn set8(&mut self, address: u16, n: u8) {
        match address & 0x01 {
            0x00 => {
                self.index = (n & 0x3f) as usize;
                self.auto_increment = n & 0x80 != 0;
            }
            0x01 => {
                let i = self.index >> 1;
                if self.index & 1 == 0 {
                    self.data[i] = (self.data[i] & 0xff00) | (n as u16);
                } else {
                    self.data[i] = (self.data[i] & 0x00ff) | ((n as u16) << 8);
                }
            }
            _ => unimplemented!(),
        }
        if self.auto_increment {
            self.index += 1;
            self.index &= 0x3f;
        }
    }

    fn get16(&self, _: u16) -> u16 {
        unimplemented!()
    }

    fn set16(&mut self, _: u16, _: u16) {
        unimplemented!()
    }
}

pub struct Gpu {
    vram: [u8; 0x4000],
    oam: [OAMEntry; 40],
    scrollx: u8,
    scrolly: u8,
    wndposx: u8,
    wndposy: u8,
    lcd_control: LcdControl,
    lcd_status: LcdStatus,
    lcd_y_coordinate: u8,
    ly_compare: u8,

    bg_palette_data: u8,
    obj_palette_0: u8,
    obj_palette_1: u8,

    background_palette: ColorPalette,
    object_palette: ColorPalette,
    ram_bank: u8,
    // BGP, OBP0 and OBP1, and BCPS/BGPI, BCPD/BGPD, OCPS/OBPI and OCPD/OBPD (CGB Mode).
    mode: u8,
    dots: u32,
    interrupt: Rc<RefCell<Interrupt>>,
}

impl Gpu {
    pub fn new() -> Self {
        Self {
            vram: [0; 0x4000],
            oam: [OAMEntry::default(); 40],
            scrollx: 0,
            scrolly: 0,
            wndposx: 0,
            wndposy: 0,
            lcd_control: LcdControl::new(),
            lcd_status: LcdStatus::new(),
            lcd_y_coordinate: 0,
            ly_compare: 0,
            bg_palette_data: 0,
            obj_palette_0: 0,
            obj_palette_1: 0,

            mode: 0,
            dots: 0,
            ram_bank: 0,
            background_palette: ColorPalette::new(),
            object_palette: ColorPalette::new(),
            interrupt: Rc::new(RefCell::new(Interrupt::new())),
        }
    }

    fn tick(&mut self, cycles: u32) {
        // 首先检查LCD是不是已经启用了，如果没启用就直接返回。
        if !self.lcd_control.lcd_and_ppu_enable {
            return;
        }

        // 如果向前走0个周期，那么也直接返回。
        if cycles == 0 {
            return;
        }

        if self.lcd_y_coordinate < 144 {
            if self.dots % 456 < 80 {
                self.lcd_status.mode = 2;
            } else if self.dots % 456 >= 80 && self.dots % 456 < (80 + 172) {
                self.lcd_status.mode = 3;
            } else if self.dots % 456 >= 80 + 172 {
                self.lcd_status.mode = 0;
            }
        } else {
            self.lcd_status.mode = 1;
        }
        self.change_mode();
    }

    /// 这里主要控制中断，不同模式的中断不一样
    fn change_mode(&mut self) {
        match self.mode {
            0 => {
                if self.lcd_status.is_mode0_interrupt_enabled {
                    self.interrupt
                        .borrow_mut()
                        .request_interrupt(IntFlag::LCDSTAT);
                }
                // Render scanline
                // if self.term == Term::GBC || self.lcdc.bit0() {
                //     self.draw_bg();
                // }
                // if self.lcdc.bit1() {
                //     self.draw_sprites();
                // }
            }
            1 => {
                self.interrupt
                    .borrow_mut()
                    .request_interrupt(IntFlag::VBLANK);
                if self.lcd_status.is_mode1_interrupt_enabled {
                    self.interrupt
                        .borrow_mut()
                        .request_interrupt(IntFlag::LCDSTAT);
                }
            }
            2 => {
                if self.lcd_status.is_mode2_interrupt_enabled {
                    self.interrupt
                        .borrow_mut()
                        .request_interrupt(IntFlag::LCDSTAT);
                }
            }
            3 => {
                self.mode = 3;
            }
            _ => (),
        }
    }
}

impl MemoryIO for Gpu {
    fn get8(&self, address: u16) -> u8 {
        match address {
            0xff40 => self.lcd_control.get8(address),
            0xff41 => self.lcd_status.get8(address),
            0xff42 => self.scrolly,
            0xff43 => self.scrollx,
            0xff44 => self.lcd_y_coordinate,
            0xff45 => self.ly_compare,
            0xff46 => 0, // DMA
            0xff47 => self.bg_palette_data,
            0xff48 => self.obj_palette_0,
            0xff49 => self.obj_palette_1,
            0xff4a => self.wndposy,
            0xff4b => self.wndposx,
            0xff4f => 0xfe | self.ram_bank,
            0xff68 => self.background_palette.get8(address), // BGPI, Background color palette specification / Background palette index
            0xff69 => self.background_palette.get8(address), // BGPD, Background color palette data / Background palette data
            0xff6a => self.object_palette.get8(address), // OBPI, OBJ color palette specification / OBJ palette index
            0xff6b => self.object_palette.get8(address), // OBPD, OBJ color palette data / OBJ palette data
            0x8000..=0x9fff => {
                self.vram[self.ram_bank as usize * 0x2000 + address as usize - 0x8000]
            }
            0xfe00..=0xfe9f => self.oam.get8(address),
            _ => 0,
        }
    }

    fn set8(&mut self, address: u16, n: u8) {
        match address {
            0xff40 => self.lcd_control.set8(address, n),
            0xff41 => self.lcd_status.set8(address, n),
            0xff42 => self.scrolly = n,
            0xff43 => self.scrollx = n,
            0xff44 => self.lcd_y_coordinate = n,
            0xff45 => self.ly_compare = n,
            0xff46 => (), // DMA
            0xff47 => self.bg_palette_data = n,
            0xff48 => self.obj_palette_0 = n,
            0xff49 => self.obj_palette_1 = n,
            0xff4a => self.wndposy = n,
            0xff4b => self.wndposx = n,
            0xff4f => self.ram_bank = 0xfe | n,
            0xff68 => self.background_palette.set8(address, n), // BGPI, Background color palette specification / Background palette index
            0xff69 => self.background_palette.set8(address, n), // BGPD, Background color palette data / Background palette data
            0xff6a => self.object_palette.set8(address, n), // OBPI, OBJ color palette specification / OBJ palette index
            0xff6b => self.object_palette.set8(address, n), // OBPD, OBJ color palette data / OBJ palette data
            0x8000..=0x9fff => {
                self.vram[self.ram_bank as usize * 0x2000 + address as usize - 0x8000] = n
            }
            0xfe00..=0xfe9f => self.oam.set8(address, n),
            _ => (),
        }
    }

    fn get16(&self, _: u16) -> u16 {
        unimplemented!("GPU doesn't support 16-bit IO.")
    }

    fn set16(&mut self, _: u16, _: u16) {
        unimplemented!("GPU doesn't support 16-bit IO.")
    }
}

fn data_to_tile(data: [u8; 16]) -> [u8; 64] {
    let mut tile = [0u8; 64];
    for i in 0..8 {
        for j in 0..8 {
            tile[i * 8 + 7 - j] =
                ((data[i * 2] & (1 << j)) >> j) + ((data[i * 2 + 1] & (1 << j)) >> j << 1);
        }
    }
    tile
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_to_tile() {
        #[rustfmt::skip]
        let data = [
            0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56, 0x38, 0x7C,
        ];
        let tile = data_to_tile(data);
        println!("{:?}", tile);
        assert_eq!(
            tile,
            [
                0, 2, 3, 3, 3, 3, 2, 0, 0, 3, 0, 0, 0, 0, 3, 0, 0, 3, 0, 0, 0, 0, 3, 0, 0, 3, 0, 0,
                0, 0, 3, 0, 0, 3, 1, 3, 3, 3, 3, 0, 0, 1, 1, 1, 3, 1, 3, 0, 0, 3, 1, 3, 1, 3, 2, 0,
                0, 2, 3, 3, 3, 2, 0, 0
            ]
        );
    }
}
