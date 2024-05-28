bitfield! {
    pub struct TileMapEntry(pub u16) {
        pub tile_index: u16 @ 0..=9,
        pub palette_selection: u8 @ 10..=12,
        pub priority: bool @ 13,
        pub flip_horizontal: bool @ 14,
        pub flip_vertical: bool @ 15,
    }
}

impl TileMapEntry {
    pub fn adjust_coords_to_flipping(&self, x: usize, y: usize) -> (usize, usize) {
        let h_shift = match self.flip_horizontal() {
            true => x % 8,
            false => 7 - (x % 8),
        };
        let v_shift = match self.flip_vertical() {
            true => 7 - (y % 8),
            false => y % 8,
        };
        (h_shift, v_shift)
    }
}

pub trait BitPlane {
    const PLANES: usize;
    const NUM_COLORS: u8;
    const WORDS_PER_ROW: usize;

    fn pixel(planes: [u8; Self::PLANES], px_index: usize) -> u8 {
        let mut pixel = 0;
        for (i, byte) in planes.iter().enumerate() {
            pixel |= ((byte >> px_index) & 1) << i;
        }
        pixel
    }
}

pub struct Bpp2 {}

pub struct Bpp4 {}

pub struct Bpp8 {}

impl BitPlane for Bpp2 {
    const PLANES: usize = 2;
    const NUM_COLORS: u8 = 4;
    const WORDS_PER_ROW: usize = 1;
}

impl BitPlane for Bpp4 {
    const PLANES: usize = 4;
    const NUM_COLORS: u8 = 16;
    const WORDS_PER_ROW: usize = 2;
}

impl BitPlane for Bpp8 {
    const PLANES: usize = 8;
    const NUM_COLORS: u8 = 255;
    const WORDS_PER_ROW: usize = 4;
}
