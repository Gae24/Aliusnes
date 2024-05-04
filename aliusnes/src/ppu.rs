use self::background::{BgMode, Mosaic};
use self::color_math::{Cgadsub, Cgwsel, ColorData, ColorMath};
use self::counters::Counters;
use self::oam::Objsel;
use self::vram::VideoPortControl;
use self::{background::Background, cgram::Cgram, oam::Oam, vram::Vram};
use crate::bus::access::Access;
use crate::utils::int_traits::ManipulateU16;

mod background;
mod cgram;
mod color_math;
mod counters;
mod oam;
mod vram;

const SCANLINE_CYCLES: u16 = 1364;

pub struct Ppu {
    background: Background,
    cgram: Cgram,
    color_math: ColorMath,
    counters: Counters,
    oam: Oam,
    vram: Vram,
    ppu1_mdr: u8,
    ppu2_mdr: u8,
    pub nmi_requested: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            background: Background::new(),
            cgram: Cgram::new(),
            color_math: ColorMath::new(),
            counters: Counters::new(),
            oam: Oam::new(),
            vram: Vram::new(),
            ppu1_mdr: 0,
            ppu2_mdr: 0,
            nmi_requested: false,
        }
    }

    pub fn tick(&mut self) {
        self.counters.elapsed_cycles += 1;

        if self.counters.elapsed_cycles >= self.counters.target_cycles {
            //todo render scanline
        }
    }
}

impl Access for Ppu {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr.low_byte() {
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
            _ => Some(self.ppu1_mdr),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        let nibble = addr.low_byte() as usize;
        match nibble {
            0x01 => self.oam.objsel = Objsel(data),
            0x02 => self.oam.oa_addl(data),
            0x03 => self.oam.oa_addh(data),
            0x04 => self.oam.oa_addr_write(data),
            0x05 => self.background.bg_mode = BgMode(data),
            0x06 => self.background.mosaic = Mosaic(data),
            0x07 | 0x08 | 0x09 | 0x0A => self.background.set_bg_sc(nibble, data),
            0x0B => self.background.set_bg_tileset_addr(0, data),
            0x0C => self.background.set_bg_tileset_addr(2, data),
            // todo 0D and 0E are also mode 7 regs
            0x0D | 0x0F | 0x11 | 0x13 => {
                self.background.set_bg_h_scroll_offset(nibble, data as u16)
            }
            0x0E | 0x10 | 0x12 | 0x14 => {
                self.background.set_bg_v_scroll_offset(nibble, data as u16)
            }
            0x15 => self.vram.video_port_control = VideoPortControl(data),
            0x16 => self.vram.vm_addl(data),
            0x17 => self.vram.vm_addh(data),
            0x18 => self.vram.vm_addl_write(data),
            0x19 => self.vram.vm_addh_write(data),
            0x21 => self.cgram.cg_addr(data),
            0x22 => self.cgram.cg_addr_write(data),
            0x30 => self.color_math.cgwsel = Cgwsel(data),
            0x31 => self.color_math.cgadsub = Cgadsub(data),
            0x32 => self.color_math.color_data_write(ColorData(data)),
            _ => unreachable!(),
        }
    }
}
