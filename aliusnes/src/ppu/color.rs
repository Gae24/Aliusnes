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
                self.cgram[self.cg_addr as usize] = byte as u16 | (data as u16) << 8;
                self.cg_addr = self.cg_addr.wrapping_add(1);
                self.latch = None;
            }
        }
    }

    pub fn color_data_write(&mut self, color_data: ColorData) {
        if color_data.write_to_red_channel() {
            self.fixed_color = (color_data.val() as u16 & 0x1F) | (self.fixed_color & !0x1F);
        }
        if color_data.write_to_green_channel() {
            self.fixed_color =
                ((color_data.val() as u16 & 0x1F) << 5) | (self.fixed_color & !(0x1F << 5))
        }
        if color_data.write_to_blue_channel() {
            self.fixed_color =
                ((color_data.val() as u16 & 0x1F) << 10) | (self.fixed_color & !(0x1F << 10))
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
        let palette = palette as u16;
        let index = index as u16;

        ((index & 0xC0) << 7 | (palette & 4) << 10)
            | ((index & 0x38) << 4 | (palette & 2) << 5)
            | ((index & 7) << 2 | (palette & 1) << 1)
    }
}

impl Ppu {
    pub fn cg_addr_read(&mut self) -> u8 {
        match self.color.latch {
            Some(high_byte) => {
                self.color.cg_addr = self.color.cg_addr.wrapping_add(1);
                self.color.latch = None;
                self.ppu2_mdr = (high_byte & 0x7F) | (self.ppu2_mdr & 0x80);
                self.ppu2_mdr
            }
            None => {
                let val = self.color.cgram[self.color.cg_addr as usize];
                self.color.latch = Some(val.high_byte());
                self.ppu2_mdr = val.low_byte();
                self.ppu2_mdr
            }
        }
    }
}
