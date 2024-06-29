bitfield! {
    pub struct BgMode(pub u8) {
        pub bg_mode: u8 @ 0..=2,
        pub bg3_has_priority: bool @ 3,
        bg1_tile_size: bool @ 4,
        bg2_tile_size: bool @ 5,
        bg3_tile_size: bool @ 6,
        bg4_tile_size: bool @ 7,
    }
}

bitfield! {
    #[allow(dead_code)]
    pub struct Mosaic(pub u8) {
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
pub struct Bg {
    bg_sc: BgSc,
    pub tileset_addr: u16,
    pub bg_hofs: u16,
    pub bg_vofs: u16,
    pub enabled_on_main_screen: bool,
}

impl Bg {
    pub fn tile_map_addr(&self, tile_x: usize, tile_y: usize) -> usize {
        let tilemap_idx = match (self.bg_sc.h_tilemap_count(), self.bg_sc.v_tilemap_count()) {
            (true, true) => (tile_x / 32) % 2 + ((tile_y / 32) % 2) * 2,
            (true, false) => (tile_x / 32) % 2,
            (false, true) => (tile_y / 32) % 2,
            (false, false) => 0,
        };
        let tile_idx = tilemap_idx * 1024 + (tile_y % 32) * 32 + (tile_x % 32);
        let tilemap_addr = (self.bg_sc.tilemap_addr() as usize) << 10;
        tilemap_addr + tile_idx
    }
}

pub(super) struct Background {
    pub backgrounds: [Bg; 4],
    pub bg_mode: BgMode,
    pub mosaic: Mosaic,
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
                enabled_on_main_screen: false,
            }; 4],
            bg_mode: BgMode(0),
            mosaic: Mosaic(0),
            h_offset_latch: 0,
            offset_latch: 0,
        }
    }

    pub fn set_bg_sc(&mut self, addr_low_byte: usize, data: u8) {
        let idx = addr_low_byte - 0x07;
        self.backgrounds[idx].bg_sc = BgSc(data);
    }

    pub fn set_bg_tileset_addr(&mut self, idx: usize, data: u8) {
        self.backgrounds[idx].tileset_addr = ((data & 0x0F) as u16) << 12;
        self.backgrounds[idx + 1].tileset_addr = (((data >> 4) & 0x0F) as u16) << 12;
    }

    pub fn set_bg_h_scroll_offset(&mut self, addr_low_byte: usize, data: u16) {
        let idx = (addr_low_byte - 0x0D) >> 1;
        self.backgrounds[idx].bg_hofs =
            data << 8 | (self.offset_latch & !7) | (self.h_offset_latch & 7);
        self.h_offset_latch = data;
        self.offset_latch = data;
    }

    pub fn set_bg_v_scroll_offset(&mut self, addr_low_byte: usize, data: u16) {
        let idx = (addr_low_byte - 0x0E) >> 1;
        self.backgrounds[idx].bg_vofs = data << 8 | self.offset_latch;
        self.offset_latch = data;
    }
}
