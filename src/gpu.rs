struct Gpu {
    vram: [u8; 0x2000],
    scrollx: u8,
    scrolly: u8,
    wndposx: u8,
    wndposy: u8,
    lcdc: u8
}