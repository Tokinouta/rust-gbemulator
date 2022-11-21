use crate::memory::MemoryIO;

struct Gpu {
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
        assert_eq!(tile, [0, 2, 3, 3, 3, 3, 2, 0, 0, 3, 0, 0, 0, 0, 3, 0, 0, 3, 0, 0, 0, 0, 3, 0, 0, 3, 0, 0, 0, 0, 3, 0, 0, 3, 1, 3, 3, 3, 3, 0, 0, 1, 1, 1, 3, 1, 3, 0, 0, 3, 1, 3, 1, 3, 2, 0, 0, 2, 3, 3, 3, 2, 0, 0]);
    }
}
