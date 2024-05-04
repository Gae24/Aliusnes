bitfield! {
    pub struct Cgwsel(pub u8) {
        direct_color_mode: bool @ 0,
        addend_is_sub_screen: bool @ 1,
        sub_screen_transparent_region: u8 @ 4..=5,
        main_screen_black_region: u8 @ 6..=7,
    }
}

bitfield! {
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

pub struct ColorMath {
    pub cgwsel: Cgwsel,
    pub cgadsub: Cgadsub,
    fixed_color: u16,
}

impl ColorMath {
    pub fn new() -> Self {
        Self {
            cgwsel: Cgwsel(0),
            cgadsub: Cgadsub(0),
            fixed_color: 0,
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
}
