// TODO: I think i should finish writing the CPU, the interrupt system before starting the GPU. So this part will be continued after I finished the parts above.

// Actually, I lack a lot of necessary knowledge about a computer system, and that is the hardest part which always hampers me from smoothly writing.

use std::{cell::RefCell, rc::Rc};

use crate::{
    interrupt::{IntFlag, Interrupt},
    memory::MemoryIO,
    Term,
};

pub const SCREEN_W: usize = 160;
pub const SCREEN_H: usize = 144;

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
    pub window_tile_base: u16,
    pub window_enable: bool,
    pub bg_and_window_tile_base: u16,
    pub bg_tile_base: u16,
    pub obj_size: bool,
    pub obj_enable: bool,
    pub bg_and_window_enable: bool,
}

impl LcdControl {
    pub fn new() -> Self {
        Self {
            lcd_and_ppu_enable: true,
            window_tile_base: 0x9800,
            window_enable: true,
            bg_and_window_tile_base: 0x8000,
            bg_tile_base: 0x9800,
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
        if self.window_tile_base == 0x9c00 {
            res |= 0x40;
        }
        if self.window_enable {
            res |= 0x20;
        }
        if self.bg_and_window_tile_base == 0x8000 {
            res |= 0x10;
        }
        if self.bg_tile_base == 0x9c00 {
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
        self.window_tile_base = 0x9800 | (((n as u16) & 0x40) >> 6 << 10);
        self.window_enable = n & 0x20 != 0;
        self.bg_and_window_tile_base = 0x8000 | ((!(n as u16) & 0x10) >> 4 << 11);
        self.bg_tile_base = 0x9800 | (((n as u16) & 0x08) >> 3 << 10);
        self.obj_size = n & 0x04 != 0;
        self.obj_enable = n & 0x02 != 0;
        self.bg_and_window_enable = n & 0x01 != 0;
    }

    fn get16(&self, _: u16) -> u16 {
        unimplemented!("LCD control register doesn't support reading 2-byte data.")
    }

    fn set16(&mut self, _: u16, _: u16) {
        unimplemented!("LCD control register doesn't support writing 2-byte data.")
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
        unimplemented!("LCD status register doesn't support reading 2-byte data.")
    }

    fn set16(&mut self, _: u16, _: u16) {
        unimplemented!("LCD status register doesn't support writing 2-byte data.")
    }
}

#[derive(Clone, Copy, Default)]
struct Attributes {
    /// true for background first, false for sprite first
    pub priority: bool,
    pub is_y_flipped: bool,
    pub is_x_flipped: bool,
    pub palette_number: u8,
    pub tile_bank: u8,
    pub palette_number_cgb: u8,
}

impl MemoryIO for Attributes {
    fn get8(&self, _: u16) -> u8 {
        let mut res = 0;
        if self.priority {
            res |= 0x80;
        }
        if self.is_y_flipped {
            res |= 0x40;
        }
        if self.is_x_flipped {
            res |= 0x20;
        }
        res |= self.palette_number << 4;
        res |= self.tile_bank << 3;
        res |= self.palette_number_cgb & 0x07;
        res
    }

    fn set8(&mut self, _: u16, n: u8) {
        self.priority = n & 0x80 != 0;
        self.is_y_flipped = n & 0x40 != 0;
        self.is_x_flipped = n & 0x20 != 0;
        self.palette_number = (n & 0x10) >> 4;
        self.tile_bank = (n & 0x08) >> 3;
        self.palette_number_cgb = n & 0x07;
    }

    fn get16(&self, _: u16) -> u16 {
        unimplemented!("Tile map attribute doesn't support reading 2-byte data.")
    }

    fn set16(&mut self, _: u16, _: u16) {
        unimplemented!("Tile map attribute doesn't support writing 2-byte data.")
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
    pub y_position: u8,
    pub x_position: u8,
    pub tile_index: u8,
    pub flags: Attributes,
}

impl MemoryIO for OAMEntry {
    fn get8(&self, address: u16) -> u8 {
        match address & 0x03 {
            0x00 => self.y_position,
            0x01 => self.x_position,
            0x02 => self.tile_index,
            0x03 => self.flags.get8(address),
            _ => unimplemented!(),
        }
    }

    fn set8(&mut self, address: u16, n: u8) {
        match address & 0x03 {
            0x00 => self.y_position = n,
            0x01 => self.x_position = n,
            0x02 => self.tile_index = n,
            0x03 => self.flags.set8(address, n),
            _ => unimplemented!(),
        }
    }

    fn get16(&self, _: u16) -> u16 {
        unimplemented!("OAM doesn't support reading 2-byte data.")
    }

    fn set16(&mut self, _: u16, _: u16) {
        unimplemented!("OAM doesn't support writing 2-byte data.")
    }
}

impl MemoryIO for [OAMEntry] {
    fn get8(&self, address: u16) -> u8 {
        self[(address - 0xfe00) as usize >> 2].get8(address)
    }

    fn set8(&mut self, address: u16, n: u8) {
        self[(address - 0xfe00) as usize >> 2].set8(address, n);
    }

    fn get16(&self, _: u16) -> u16 {
        unimplemented!("OAM doesn't support reading 2-byte data.")
    }

    fn set16(&mut self, _: u16, _: u16) {
        unimplemented!("OAM doesn't support writing 2-byte data.")
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

    pub fn get_color(&self, index: u8) -> (u8, u8, u8) {
        let r = self.data[index as usize] & 0x1f;
        let g = self.data[index as usize] & 0x3e;
        let b = self.data[index as usize] & 0x7c;
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
        unimplemented!("Color palette doesn't support reading 2-byte data.")
    }

    fn set16(&mut self, _: u16, _: u16) {
        unimplemented!("Color palette doesn't support writing 2-byte data.")
    }
}

pub struct Gpu {
    term: Term,
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

    // ??????????????????????????????????????????GameBoy
    bg_palette_data: u8, // ???????????????
    obj_palette_0: u8,   // ???????????????0
    obj_palette_1: u8,   // ???????????????1

    // ????????????????????????GameBoy??????????????????
    background_palette: ColorPalette,
    object_palette: ColorPalette,

    ram_bank: u8,
    // BGP, OBP0 and OBP1, and BCPS/BGPI, BCPD/BGPD, OCPS/OBPI and OCPD/OBPD (CGB Mode).
    mode: u8,
    prio: [(bool, usize); SCREEN_W],

    /// 1 dots ???1/4.19M??????
    dots: u32,
    interrupt: Rc<RefCell<Interrupt>>,

    pub data: [[[u8; 3]; SCREEN_W]; SCREEN_H],
}

impl Gpu {
    pub fn new() -> Self {
        Self {
            term: Term::GB,
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
            prio: [(true, 0); SCREEN_W],
            dots: 0,
            ram_bank: 0,
            background_palette: ColorPalette::new(),
            object_palette: ColorPalette::new(),
            interrupt: Rc::new(RefCell::new(Interrupt::new())),

            data: [[[0xffu8; 3]; SCREEN_W]; SCREEN_H],
        }
    }

    // Grey scale.
    fn set_gre(&mut self, x: usize, color: u8, palette_data: u8) {
        let g = match palette_data >> (2 * color) & 0x03 {
            0x00 => 0xff,
            0x01 => 0xc0,
            0x02 => 0x60,
            0x03 => 0x00,
            _ => 0,
        };
        self.data[self.lcd_y_coordinate as usize][x] = [g, g, g];
    }

    // When developing graphics on PCs, note that the RGB values will have different appearance on CGB displays as on
    // VGA/HDMI monitors calibrated to sRGB color. Because the GBC is not lit, the highest intensity will produce Light
    // Gray color rather than White. The intensities are not linear; the values 10h-1Fh will all appear very bright,
    // while medium and darker colors are ranged at 00h-0Fh.
    // The CGB display's pigments aren't perfectly saturated. This means the colors mix quite oddly; increasing
    // intensity of only one R,G,B color will also influence the other two R,G,B colors. For example, a color setting
    // of 03EFh (Blue=0, Green=1Fh, Red=0Fh) will appear as Neon Green on VGA displays, but on the CGB it'll produce a
    // decently washed out Yellow. See image on the right.
    fn set_rgb(&mut self, x: usize, r: u8, g: u8, b: u8) {
        assert!(r <= 0x1f);
        assert!(g <= 0x1f);
        assert!(b <= 0x1f);
        let r = u32::from(r);
        let g = u32::from(g);
        let b = u32::from(b);
        let lr = ((r * 13 + g * 2 + b) >> 1) as u8;
        let lg = ((g * 3 + b) << 1) as u8;
        let lb = ((r * 3 + g * 2 + b * 11) >> 1) as u8;
        self.data[self.lcd_y_coordinate as usize][x] = [lr, lg, lb];
    }

    /// ??????????????????????????????
    ///
    /// cycles????????????
    fn tick(&mut self, cycles: u32) {
        // ????????????LCD????????????????????????????????????????????????????????????
        if !self.lcd_control.lcd_and_ppu_enable {
            return;
        }

        // ???????????????0????????????????????????????????????
        if cycles == 0 {
            return;
        }

        // ?????????????????????????????????80 dots????????????80??????????????????????????????????????????
        let c = (cycles - 1) / 80 + 1; // ????????????
        for i in 0..c {
            if i == c - 1 {
                self.dots += cycles & 80;
            } else {
                self.dots += 80;
            }

            if self.dots != self.dots % 456 {
                self.lcd_y_coordinate = (self.lcd_y_coordinate + 1) % 154;
                if self.lcd_status.current_line_interrupt
                    && self.lcd_y_coordinate == self.ly_compare
                {
                    self.interrupt
                        .borrow_mut()
                        .request_interrupt(IntFlag::LCDSTAT);
                }
            }
            self.change_mode();
        }
    }

    /// ?????????????????????????????????????????????????????????
    fn change_mode(&mut self) {
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

        match self.mode {
            0 => {
                if self.lcd_status.is_mode0_interrupt_enabled {
                    self.interrupt
                        .borrow_mut()
                        .request_interrupt(IntFlag::LCDSTAT);
                }
                // Render scanline
                if self.term == Term::GBC || self.lcd_control.bg_and_window_enable {
                    self.draw_background();
                }
                if self.lcd_control.obj_enable {
                    self.draw_sprites();
                }
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

    fn draw_background(&mut self) {
        let show_window = self.lcd_control.window_enable && self.wndposy <= self.lcd_y_coordinate;
        let tile_base = self.lcd_control.bg_and_window_tile_base;

        let window_x = self.wndposx.wrapping_sub(7);
        let pixel_y = if show_window {
            self.lcd_y_coordinate.wrapping_sub(self.wndposy)
        } else {
            self.scrolly.wrapping_add(self.lcd_y_coordinate)
        };
        // ?????????????????????tile
        let tile_y = (pixel_y as u16 >> 3) & 0x1f;

        // ?????????????????????????????????x??????????????????x?????????
        for x in 0..SCREEN_W {
            let pixel_x = if show_window && x as u8 >= window_x {
                x as u8 - window_x
            } else {
                self.scrollx.wrapping_add(x as u8)
            };
            // ?????????????????????tile
            let tile_x = (pixel_x as u16 >> 3) & 0x1f;
            let background_base = if show_window && x as u8 > window_x {
                self.lcd_control.window_tile_base
            } else {
                self.lcd_control.bg_tile_base
            };

            // ???????????????tile?????????????????????????????????????????????tile???????????????????????????
            // ?????????????????????????????????tile?????????????????????????????????????????????????????????
            // ??????tile?????????????????????????????????????????????????????????????????????????????????????????????tile??????????????????????????????
            let tile_address = background_base + tile_y * 32 + tile_x;
            let tile_number = self.get8(tile_address);
            let tile_offset = if self.lcd_control.bg_and_window_tile_base == 0x8000 {
                tile_number
            } else {
                tile_number.wrapping_add(128)
            } as u16
                * 16;
            let tile_location = tile_base + tile_offset;
            // ??????tile???attribute??????CGB????????????????????????????????????????????????vram??????????????????????????????
            // ????????????????????????0??????????????????????????????
            let mut tile_attribute = Attributes::default();
            tile_attribute.set8(0, self.vram[tile_address as usize - 0x6000]);

            // tile??????????????????
            let tile_y = if tile_attribute.is_y_flipped {
                7 - pixel_y % 8
            } else {
                pixel_y % 8
            };
            let tile_y_data = if self.term == Term::GBC {
                let a = self.vram[(tile_location
                    + tile_y as u16 * 2
                    + (tile_attribute.tile_bank as u16)
                    << 13) as usize
                    - 0x8000];
                let b = self.vram[(tile_location
                    + tile_y as u16 * 2
                    + (tile_attribute.tile_bank as u16)
                    << 13) as usize
                    - 0x8000
                    + 1];
                (a, b)
            } else {
                let a = self.vram[(tile_location + tile_y as u16 * 2) as usize - 0x8000];
                let b = self.vram[(tile_location + tile_y as u16 * 2) as usize - 0x8000 + 1];
                (a, b)
            };
            let tile_x = if tile_attribute.is_x_flipped {
                7 - pixel_x % 8
            } else {
                pixel_x % 8
            };

            let color_l = if tile_y_data.0 & (0x80 >> tile_x) != 0 {
                1
            } else {
                0
            };
            let color_r = if tile_y_data.1 & (0x80 >> tile_x) != 0 {
                2
            } else {
                0
            };
            let color: u8 = color_l | color_r;

            // ??????????????????????????????????????????????????????sprite?????????????????????
            self.prio[x] = (tile_attribute.priority, color as usize);

            if self.term == Term::GBC {
                let (r, g, b) = self
                    .background_palette
                    .get_color(tile_attribute.palette_number_cgb * 4 + color);
                self.set_rgb(x, r, g, b);
            } else {
                self.set_gre(x, color, self.bg_palette_data);
            }
        }
    }

    fn draw_sprites(&mut self) {
        // sprite??????????????????
        let sprite_size = if self.lcd_control.obj_size { 16 } else { 8 };
        // ?????????40???sprite?????????????????????????????????????????????
        for i in 0..40 {
            let mut sprite = self.oam[i];
            sprite.tile_index &= if self.lcd_control.obj_size {
                0xfe
            } else {
                0xff
            };

            // ?????????if?????????????????????????????????????????????sprite
            if sprite.y_position <= 0xff - sprite_size + 1 {
                // ??????sprite?????????background?????????
                if self.lcd_y_coordinate < sprite.y_position
                    || self.lcd_y_coordinate > sprite.y_position + sprite_size - 1
                {
                    continue;
                }
            } else {
                // ???????????????background?????????
                if self.lcd_y_coordinate > sprite.y_position.wrapping_add(sprite_size) - 1 {
                    continue;
                }
            }

            // ????????????????????????sprite????????????????????????????????????tile????????????????????????
            let tile_y = if sprite.flags.is_y_flipped {
                sprite_size - 1 - self.lcd_y_coordinate.wrapping_sub(sprite.y_position)
            } else {
                self.lcd_y_coordinate.wrapping_sub(sprite.y_position)
            };
            let tile_location = 0x8000 + sprite.tile_index as u16 * 16 + tile_y as u16 * 2;
            let tile_y_data = if self.term == Term::GBC {
                let a =
                    self.vram[(tile_location + tile_y as u16 * 2 + (sprite.flags.tile_bank as u16)
                        << 13) as usize
                        - 0x8000];
                let b =
                    self.vram[(tile_location + tile_y as u16 * 2 + (sprite.flags.tile_bank as u16)
                        << 13) as usize
                        - 0x8000
                        + 1];
                (a, b)
            } else {
                let a = self.vram[(tile_location + tile_y as u16 * 2) as usize - 0x8000];
                let b = self.vram[(tile_location + tile_y as u16 * 2) as usize - 0x8000 + 1];
                (a, b)
            };

            for x in 0..8 {
                if sprite.x_position as usize + x as usize > SCREEN_W {
                    continue;
                }
                let tile_x = if sprite.flags.is_x_flipped { 7 - x } else { x };
                let color_l = if tile_y_data.0 & (0x80 >> tile_x) != 0 {
                    1
                } else {
                    0
                };
                let color_r = if tile_y_data.1 & (0x80 >> tile_x) != 0 {
                    2
                } else {
                    0
                };
                let color: u8 = color_l | color_r;

                // ??????????????????????????????????????????????????????sprite?????????????????????
                let prio = self.prio[x];
                let skip = if self.term == Term::GBC && !self.lcd_control.bg_and_window_enable {
                    // ???????????????????????????????????????????????????????????????
                    // ???????????????????????????????????????
                    prio.1 == 0
                } else if prio.0 || sprite.flags.priority {
                    // ????????????????????????????????????????????????????????????
                    prio.1 != 0
                } else {
                    // ?????????
                    false
                };
                if skip {
                    continue;
                }

                if self.term == Term::GBC {
                    let (r, g, b) = self
                        .background_palette
                        .get_color(sprite.flags.palette_number_cgb * 4 + color);
                    self.set_rgb(sprite.x_position.wrapping_add(x as u8) as usize, r, g, b);
                } else {
                    if sprite.flags.palette_number == 0 {
                        self.set_gre(
                            sprite.x_position.wrapping_add(x as u8) as usize,
                            color,
                            self.obj_palette_0,
                        );
                    } else {
                        self.set_gre(
                            sprite.x_position.wrapping_add(x as u8) as usize,
                            color,
                            self.obj_palette_1,
                        );
                    }
                }
            }
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
            0xff40 => {
                self.lcd_control.set8(address, n);
                if !self.lcd_control.lcd_and_ppu_enable {
                    self.dots = 0;
                    self.lcd_y_coordinate = 0;
                    self.lcd_status.mode = 0;
                    // Clean screen.
                    self.data = [[[0xffu8; 3]; SCREEN_W]; SCREEN_H];
                }
            }
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
