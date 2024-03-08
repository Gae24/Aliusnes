bitfield! {
    struct BgMode(u8) {
        bg_mode: u8 @ 0..=2,
        mode1_bg3_has_priority: bool @ 3,
        bg1_tile_size: bool @ 4,
        bg2_tile_size: bool @ 5,
        bg3_tile_size: bool @ 6,
        bg4_tile_size: bool @ 7,
    }
}

bitfield! {
    struct Mosaic(u8) {
        bg1_enabled: bool @ 0,
        bg2_enabled: bool @ 1,
        bg3_enabled: bool @ 2,
        bg4_enabled: bool @ 3,
        mosaic_size: u8 @ 4..=7,
    }
}

bitfield! {
    #[derive(Clone, Copy)]
    struct BgSc(u8) {
       h_tilemap_count: bool @ 0,
       v_tilemap_count: bool @ 1,
       tilemap_addr: u8 @ 2..=7,
    }
}

#[derive(Clone, Copy)]
struct Bg {
    bg_sc: BgSc,
    tileset_addr: u16,
    bg_hofs: u16,
    bg_vofs: u16,
}

struct Background {
    backgrounds: [Bg; 4],
    bg_mode: BgMode,
    mosaic: Mosaic,
    h_offset_latch: u16,
    offset_latch: u16,
}

impl Background {
    pub fn new() -> Self {
        Self {
            backgrounds: [Bg {
                bg_sc: BgSc(0),
                tileset_addr: 0,
                bg_hofs: 0,
                bg_vofs: 0,
            }; 4],
            bg_mode: BgMode(0),
            mosaic: Mosaic(0),
            h_offset_latch: 0,
            offset_latch: 0,
        }
    }

    fn set_bg_tileset_addr(&mut self, idx: usize, data: u8) {
        self.backgrounds[idx].tileset_addr = ((data & 0x0F) as u16) << 12;
        self.backgrounds[idx + 1].tileset_addr = (((data >> 4) & 0x0F) as u16) << 12;
    }

    fn set_bg_h_scroll_offset(&mut self, addr_low_byte: usize, data: u16) {
        let idx = (addr_low_byte - 0x0D) / 2;
        self.backgrounds[idx].bg_hofs =
            data << 8 | (self.offset_latch & !7) | (self.h_offset_latch & 7);
        self.h_offset_latch = data;
        self.offset_latch = data;
    }

    fn set_bg_v_scroll_offset(&mut self, addr_low_byte: usize, data: u16) {
        let idx = (addr_low_byte - 0x0D) / 2;
        self.backgrounds[idx].bg_vofs = data << 8 | self.offset_latch;
        self.offset_latch = data;
    }
}
