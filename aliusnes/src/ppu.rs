use self::{
    background::{Background, BgMode, Mosaic},
    color::{Cgadsub, Cgwsel, Color, ColorData},
    counters::Counters,
    mode7::Mode7,
    oam::{Oam, Objsel},
    vram::{VideoPortControl, Vram},
};
use crate::{bus::Access, cart::info::Model, utils::int_traits::ManipulateU16};

mod background;
mod color;
mod counters;
mod mode7;
mod oam;
mod render;
mod tile;
mod vram;

const SCANLINE_CYCLES: u16 = 1364;

const NTSC_SCANLINES: usize = 262;
const PAL_SCANLINES: usize = 312;

pub const WIDTH: usize = 256;

pub const NTSC_HEIGHT: usize = 224;
pub const PAL_HEIGHT: usize = 239;

//pub const FB_WIDTH: usize = WIDTH << 1;
//pub const FB_HEIGHT: usize = PAL_HEIGHT << 1;

bitfield! {
    struct IniDisplay(pub u8) {
        screen_brightness: u8 @ 0..=3,
        force_blanking: bool @ 7,
    }
}

bitfield! {
    struct SetIni(pub u8) {
        screen_interlacing: bool @ 0,
        obj_interlacing: bool @ 1,
        overscan_mode: bool @ 2,
        high_res_mode: bool @ 3,
        extbg_mode: bool @ 6,
        external_sync: bool @ 7,
    }
}

pub struct Ppu {
    background: Background,
    color: Color,
    counters: Counters,
    mode7: Mode7,
    oam: Oam,
    vram: Vram,
    ppu1_mdr: u8,
    ppu2_mdr: u8,

    ini_display: IniDisplay,
    set_ini: SetIni,
    pub screen_width: usize,
    pub screen_height: usize,
    pub frame_buffer: Box<[[u8; 3]; WIDTH * PAL_HEIGHT]>,
    pub nmi_requested: bool,
}

impl Ppu {
    pub fn new(model: Model) -> Self {
        let stat78 = counters::Stat78(0)
            .with_ppu2_version(2)
            .with_is_pal(model == Model::Pal);
        Self {
            background: Background::new(),
            color: Color::new(),
            counters: Counters::new(stat78),
            mode7: Mode7::new(),
            oam: Oam::new(),
            vram: Vram::new(),
            ppu1_mdr: 0,
            ppu2_mdr: 0,
            ini_display: IniDisplay(0),
            set_ini: SetIni(0),
            screen_width: WIDTH,
            screen_height: if model == Model::Pal {
                PAL_HEIGHT
            } else {
                NTSC_HEIGHT
            },
            frame_buffer: Box::new([[0; 3]; WIDTH * PAL_HEIGHT]),
            nmi_requested: false,
        }
    }

    pub fn tick(&mut self) {
        self.counters.update_status(
            self.set_ini.overscan_mode(),
            self.set_ini.screen_interlacing(),
        );

        if self.counters.in_hdraw() {
            self.render_scanline(self.counters.vertical_counter);
            self.counters.last_scanline = self.counters.vertical_counter;
        }
    }

    fn set_ini_write(&mut self, set_ini: SetIni) {
        self.screen_width <<= usize::from(set_ini.high_res_mode());
        self.screen_height = if set_ini.overscan_mode() {
            PAL_HEIGHT
        } else {
            NTSC_HEIGHT
        };
        self.screen_height <<= usize::from(set_ini.screen_interlacing());
        self.set_ini = set_ini;
    }

    pub fn main_screen_layer_enable(&mut self, data: u8) {
        for idx in 0..4 {
            self.background.backgrounds[idx].enabled_on_main_screen = (data >> idx) & 1 != 0;
        }
        self.oam.enabled_on_main_screen = (data >> 4) & 1 != 0;
    }

    pub fn frame_counter(&self) -> u64 {
        self.counters.frame_counter
    }

    pub fn frame_ready(&self) -> bool {
        self.counters.frame_ready
    }
}

impl Access for Ppu {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr.low_byte() {
            0x34 => Some((self.mode7.do_multiplication()) as u8),
            0x35 => Some((self.mode7.do_multiplication() >> 8) as u8),
            0x36 => Some((self.mode7.do_multiplication() >> 16) as u8),
            0x37 => {
                self.counters.software_latch();
                None
            }
            0x38 => {
                self.ppu1_mdr = self.oam.oa_addr_read();
                Some(self.ppu1_mdr)
            }
            0x39 => {
                self.ppu1_mdr = self.vram.vm_addl_read();
                Some(self.ppu1_mdr)
            }
            0x3A => {
                self.ppu1_mdr = self.vram.vm_addh_read();
                Some(self.ppu1_mdr)
            }
            0x3B => Some(self.cg_addr_read()),
            0x3C => Some(self.ophct_read()),
            0x3D => Some(self.opvct_read()),
            0x3F => Some(self.status78_read()),
            _ => {
                println!("Tried to read at {addr:#0x}");
                Some(self.ppu1_mdr)
            }
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        let nibble = addr.low_byte() as usize;
        match nibble {
            0x00 => self.ini_display = IniDisplay(data),
            0x01 => self.oam.objsel = Objsel(data),
            0x02 => self.oam.oa_addl(data),
            0x03 => self.oam.oa_addh(data),
            0x04 => self.oam.oa_addr_write(data),
            0x05 => self.background.bg_mode = BgMode(data),
            0x06 => self.background.mosaic = Mosaic(data),
            0x07..=0x0A => self.background.set_bg_sc(nibble, data),
            0x0B => self.background.set_bg_tileset_addr(0, data),
            0x0C => self.background.set_bg_tileset_addr(2, data),
            // todo 0D and 0E are also mode 7 regs
            0x0D | 0x0F | 0x11 | 0x13 => {
                self.background
                    .set_bg_h_scroll_offset(nibble, u16::from(data));
            }
            0x0E | 0x10 | 0x12 | 0x14 => {
                self.background
                    .set_bg_v_scroll_offset(nibble, u16::from(data));
            }
            0x15 => self.vram.video_port_control = VideoPortControl(data),
            0x16 => self.vram.vm_addl(data),
            0x17 => self.vram.vm_addh(data),
            0x18 => self.vram.vm_addl_write(data),
            0x19 => self.vram.vm_addh_write(data),
            0x1B => self.mode7.set_mode_7_matrix_a(data),
            0x1C => self.mode7.set_mode_7_matrix_b(data),
            0x21 => self.color.cg_addr(data),
            0x22 => self.color.cg_addr_write(data),
            0x2C => self.main_screen_layer_enable(data),
            0x30 => self.color.cgwsel = Cgwsel(data),
            0x31 => self.color.cgadsub = Cgadsub(data),
            0x32 => self.color.color_data_write(ColorData(data)),
            0x33 => self.set_ini_write(SetIni(data)),
            _ => println!("Tried to write at {addr:#0x} val: {data:#04x}"),
        }
    }
}
