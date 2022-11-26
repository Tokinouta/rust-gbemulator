// TODO: I think i should finish writing the CPU, the interrupt system before starting the GPU. So this part will be continued after I finished the parts above.

// Actually, I lack a lot of necessary knowledge about a computer system, and that is the hardest part which always hampers me from smoothly writing.

use crate::memory::MemoryIO;

pub struct Gpu {
    vram: [u8; 0x2000],
    oam: [u8; 0xa0],
    scrollx: u8,
    scrolly: u8,
    wndposx: u8,
    wndposy: u8,
    lcd_control: u8,
    lcd_status: u8,
    lcd_y_coordinate: u8,
    ly_compare: u8,

    bg_palette_data: u8,
    obj_palette_0: u8,
    obj_palette_1: u8,

    bcps_bcpi: u8,
    bcpd_bgpd: u16,
    ocps_obpi: u16,
    ocpd_obpd: u16,
    // BGP, OBP0 and OBP1, and BCPS/BGPI, BCPD/BGPD, OCPS/OBPI and OCPD/OBPD (CGB Mode).
    mode: u8,
    mode_counter: u32,
}

impl Gpu {
    pub fn new() -> Self {
        Self {
            vram: [0; 0x2000],
            oam: [0; 0xa0],
            scrollx: 0,
            scrolly: 0,
            wndposx: 0,
            wndposy: 0,
            lcd_control: 0,
            lcd_status: 0,
            lcd_y_coordinate: 0,
            ly_compare: 0,
            bg_palette_data: 0,
            obj_palette_0: 0,
            obj_palette_1: 0,
            bcps_bcpi: 0,
            bcpd_bgpd: 0,
            ocps_obpi: 0,
            ocpd_obpd: 0,
            mode: 0,
            mode_counter: 0,
        }
    }
    fn lcd_and_ppu_enable(&self) -> bool {
        self.lcd_control & 0x80 != 0
    }
    fn window_tile_map_area(&self) -> bool {
        self.lcd_control & 0x40 != 0
    }
    fn window_enable(&self) -> bool {
        self.lcd_control & 0x20 != 0
    }
    fn bg_and_window_tile_data_area(&self) -> bool {
        self.lcd_control & 0x10 != 0
    }
    fn bg_tile_map_area(&self) -> bool {
        self.lcd_control & 0x8 != 0
    }
    fn obj_size(&self) -> bool {
        self.lcd_control & 0x4 != 0
    }
    fn obj_enable(&self) -> bool {
        self.lcd_control & 0x2 != 0
    }
    fn bg_and_window_enable(&self) -> bool {
        self.lcd_control & 0x1 != 0
    }

    // LYC=LY STAT Interrupt source         (1=Enable) (Read/Write)
    // Bit 5 - Mode 2 OAM STAT Interrupt source     (1=Enable) (Read/Write)
    // Bit 4 - Mode 1 VBlank STAT Interrupt source  (1=Enable) (Read/Write)
    // Bit 3 - Mode 0 HBlank STAT Interrupt source  (1=Enable) (Read/Write)
    // Bit 2 - LYC=LY Flag                          (0=Different, 1=Equal) (Read Only)
    // Bit 1-0 - Mode Flag

    fn Scanlines(&mut self) {}

    /// 这里主要控制中断，不同模式的中断不一样
    fn change_mode(&mut self, mode: u8) {
        match mode {
            0 => {
                self.mode = 0;
            }
            1 => {
                self.mode = 1;
            }
            2 => {
                self.mode = 2;
            }
            3 => {
                self.mode = 3;
            }
            _ => (),
        }
    }
}

struct OAMEntry {
    y_position: u8,
    x_position: u8,
    tile_index: u8,
    flags: u8,
}

impl OAMEntry {
    pub fn bg_over_obj(&self) -> bool {
        self.flags & 0x80 == 1
    }
}

impl MemoryIO for Gpu {
    fn get8(&self, address: u16) -> u8 {
        match address {
            0xff40 => self.lcd_control,
            0xff41 => self.lcd_status,
            0xff42 => self.scrolly,
            0xff43 => self.scrollx,
            0xff44 => self.lcd_y_coordinate,
            0xff45 => self.ly_compare,
            0xff4a => self.wndposy,
            0xff4b => self.wndposx,

            0xff47 => self.bg_palette_data,
            0xff48 => self.obj_palette_0,
            0xff49 => self.obj_palette_1,
            _ => 0,
        }
    }

    fn set8(&mut self, address: u16, n: u8) {
        match address {
            0xff40 => self.lcd_control = n,
            0xff41 => self.lcd_status = n,
            _ => (),
        }
    }

    fn get16(&self, address: u16) -> u16 {
        todo!()
    }

    fn set16(&mut self, address: u16, n: u16) {
        todo!()
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
