use super::{tile::BitPlane, Ppu};
use crate::utils::int_traits::ManipulateU16;

bitfield! {
    pub struct Cgwsel(pub u8) {
        direct_color_mode: bool @ 0,
        addend_is_sub_screen: bool @ 1,
        sub_screen_transparent_region: u8 @ 4..=5,
        main_screen_black_region: u8 @ 6..=7,
    }
}

bitfield! {
    #[allow(dead_code)]
    pub struct Cgadsub(pub u8) {
        bg1_color_math_enabled: bool @ 0,
        bg2_color_math_enabled: bool @ 1,
        bg3_color_math_enabled: bool @ 2,
        bg4_color_math_enabled: bool @ 3,
        obj_color_math_enabled: bool @ 4,
        backdrop_color_math_enabled: bool @ 5,
        halve_color_math_result: bool @ 6,
        operation_is_sub: bool @ 7,
    }
}

bitfield! {
    pub struct ColorData(pub u8) {
        val: u8 @ 0..=4,
        write_to_red_channel: bool @ 5,
        write_to_green_channel: bool @ 6,
        write_to_blue_channel: bool @ 7,
    }
}

const FIVEBIT_TO_EIGHTBIT_LUT: [[u8; 0x20]; 0x10] = [
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ],
    [
        0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 12, 12, 13, 13, 14,
        14, 15, 15, 16, 17,
    ],
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 24, 25, 26,
        27, 28, 29, 30, 31, 32, 34,
    ],
    [
        0, 1, 3, 4, 6, 8, 9, 11, 13, 14, 16, 18, 19, 21, 23, 24, 26, 27, 29, 31, 32, 34, 36, 37,
        39, 41, 42, 44, 46, 47, 49, 51,
    ],
    [
        0, 2, 4, 6, 8, 10, 13, 15, 17, 19, 21, 24, 26, 28, 30, 32, 34, 37, 39, 41, 43, 45, 48, 50,
        52, 54, 56, 59, 61, 63, 65, 68,
    ],
    [
        0, 2, 5, 8, 10, 13, 16, 19, 21, 24, 27, 30, 32, 35, 38, 41, 43, 46, 49, 52, 54, 57, 60, 63,
        65, 68, 71, 74, 76, 79, 82, 85,
    ],
    [
        0, 3, 6, 9, 12, 16, 19, 22, 26, 29, 32, 36, 39, 42, 46, 49, 52, 55, 59, 62, 65, 68, 72, 75,
        78, 82, 85, 88, 92, 95, 98, 102,
    ],
    [
        0, 3, 7, 11, 14, 19, 22, 26, 30, 34, 38, 42, 45, 49, 53, 57, 61, 64, 69, 72, 76, 80, 84,
        88, 91, 95, 99, 103, 107, 111, 114, 119,
    ],
    [
        0, 4, 8, 12, 17, 21, 26, 30, 34, 39, 43, 48, 52, 56, 61, 65, 69, 74, 78, 83, 87, 91, 96,
        100, 105, 109, 113, 118, 122, 126, 131, 136,
    ],
    [
        0, 4, 9, 14, 19, 24, 29, 34, 39, 44, 49, 54, 58, 63, 69, 73, 78, 83, 88, 93, 98, 103, 108,
        113, 118, 123, 127, 133, 138, 142, 147, 153,
    ],
    [
        0, 5, 10, 16, 21, 27, 32, 38, 43, 49, 54, 60, 65, 70, 76, 82, 87, 92, 98, 104, 109, 114,
        120, 126, 131, 136, 142, 148, 153, 158, 164, 170,
    ],
    [
        0, 5, 11, 17, 23, 30, 35, 41, 47, 54, 60, 66, 71, 77, 84, 90, 96, 101, 108, 114, 120, 126,
        132, 138, 144, 150, 156, 162, 168, 174, 180, 187,
    ],
    [
        0, 6, 12, 19, 25, 32, 39, 45, 52, 59, 65, 72, 78, 84, 92, 98, 104, 111, 118, 124, 131, 137,
        144, 151, 157, 164, 170, 177, 184, 190, 196, 204,
    ],
    [
        0, 6, 13, 20, 27, 35, 42, 49, 56, 64, 71, 78, 84, 91, 99, 106, 113, 120, 128, 135, 142,
        149, 156, 163, 170, 177, 184, 192, 199, 206, 213, 221,
    ],
    [
        0, 7, 14, 22, 29, 38, 45, 53, 60, 69, 76, 84, 91, 98, 107, 114, 122, 129, 138, 145, 153,
        160, 168, 176, 183, 191, 198, 207, 214, 222, 229, 238,
    ],
    [
        0, 8, 16, 24, 33, 41, 49, 57, 66, 74, 82, 90, 99, 107, 115, 123, 132, 140, 148, 156, 165,
        173, 181, 189, 198, 206, 214, 222, 231, 239, 247, 255,
    ],
];

pub(super) struct Color {
    pub cgram: [u16; 0x100],
    cg_addr: u8,
    latch: Option<u8>,

    pub cgwsel: Cgwsel,
    pub cgadsub: Cgadsub,
    fixed_color: u16,
}

impl Color {
    pub fn new() -> Self {
        Self {
            cgram: [0; 0x100],
            cg_addr: 0x00,
            latch: None,
            cgwsel: Cgwsel(0),
            cgadsub: Cgadsub(0),
            fixed_color: 0,
        }
    }

    pub fn cg_addr(&mut self, data: u8) {
        self.cg_addr = data;
        self.latch = None;
    }

    pub fn cg_addr_write(&mut self, data: u8) {
        match self.latch {
            None => self.latch = Some(data),
            Some(byte) => {
                self.cgram[self.cg_addr as usize] = u16::from(byte) | u16::from(data) << 8;
                self.cg_addr = self.cg_addr.wrapping_add(1);
                self.latch = None;
            }
        }
    }

    pub fn color_data_write(&mut self, color_data: ColorData) {
        if color_data.write_to_red_channel() {
            self.fixed_color = (u16::from(color_data.val()) & 0x1F) | (self.fixed_color & !0x1F);
        }
        if color_data.write_to_green_channel() {
            self.fixed_color =
                ((u16::from(color_data.val()) & 0x1F) << 5) | (self.fixed_color & !(0x1F << 5));
        }
        if color_data.write_to_blue_channel() {
            self.fixed_color =
                ((u16::from(color_data.val()) & 0x1F) << 10) | (self.fixed_color & !(0x1F << 10));
        }
    }

    pub fn pixel_color<T: BitPlane, const BG_MODE: u8>(
        &self,
        bg_index: u8,
        palette: u8,
        index: u8,
    ) -> u16 {
        let starting_palette_entry = match T::PLANES {
            2 if BG_MODE == 0 => bg_index * 32 + (palette * T::NUM_COLORS),
            8 => 0,
            _ => palette * T::NUM_COLORS,
        };

        if T::PLANES == 8 && self.cgwsel.direct_color_mode() {
            Self::direct_color(palette, index)
        } else {
            self.cgram[(starting_palette_entry + index) as usize]
        }
    }

    /// palette: 3 bit from a tilemap entry that is treated as bgr
    /// index: character data that is treated as BBGGGRRR
    /// Returns an RGB555 in this format BBb00:GGGg0:RRRr0
    fn direct_color(palette: u8, index: u8) -> u16 {
        let palette = u16::from(palette);
        let index = u16::from(index);

        ((index & 0xC0) << 7 | (palette & 4) << 10)
            | ((index & 0x38) << 4 | (palette & 2) << 5)
            | ((index & 7) << 2 | (palette & 1) << 1)
    }
}

impl Ppu {
    pub fn cg_addr_read(&mut self) -> u8 {
        if let Some(high_byte) = self.color.latch {
            self.color.cg_addr = self.color.cg_addr.wrapping_add(1);
            self.color.latch = None;
            self.ppu2_mdr = (high_byte & 0x7F) | (self.ppu2_mdr & 0x80);
            self.ppu2_mdr
        } else {
            let val = self.color.cgram[self.color.cg_addr as usize];
            self.color.latch = Some(val.high_byte());
            self.ppu2_mdr = val.low_byte();
            self.ppu2_mdr
        }
    }

    pub fn rgb555_to_rgb888(value: u16) -> [u8; 3] {
        let r = FIVEBIT_TO_EIGHTBIT_LUT[0xF][usize::from(value & 0x1F)];
        let g = FIVEBIT_TO_EIGHTBIT_LUT[0xF][usize::from(value >> 5 & 0x1F)];
        let b = FIVEBIT_TO_EIGHTBIT_LUT[0xF][usize::from(value >> 10 & 0x1F)];
        [r, g, b]
    }
}
