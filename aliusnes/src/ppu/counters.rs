#[cfg(feature = "log")]
use std::time::Instant;

use crate::{
    ppu::{Ppu, NTSC_HEIGHT, NTSC_SCANLINES, PAL_HEIGHT, PAL_SCANLINES, SCANLINE_CYCLES},
    utils::int_traits::ManipulateU16,
};

bitfield! {
    pub struct Nmitimen(pub u8) {
        joypad_enable: bool @ 0,
        hv_timer_mode: u8 @ 4..=5,
        nmi_enabled: bool @ 7,
    }
}

bitfield! {
    pub struct Rdnmi(pub u8) {
        cpu_version: u8 @ 0..=3,
        in_nmi: bool @ 7,
    }
}

bitfield! {
    pub struct HvStatus(pub u8) {
        pub in_hblank: bool @ 6,
        pub in_vblank: bool @ 7,
    }
}

bitfield! {
    pub struct Stat78(pub u8) {
        pub ppu2_version: u8 @ 0..=3,
        pub is_pal: bool @ 4,
        counter_latch: bool @ 6,
        odd_frame: bool @ 7,
    }
}

pub struct Counters {
    pub vertical_counter: usize,
    pub vblank_start: usize,
    pub vblank_end: usize,
    pub cycles_per_scanline: u16,
    pub frame_counter: u64,

    ophct_latch: bool,
    opvct_latch: bool,
    output_horizontal_counter: u16,
    output_vertical_counter: u16,
    h_timer_target: u16,
    v_timer_target: u16,

    nmitimen: Nmitimen,
    rdnmi: Rdnmi,
    stat78: Stat78,
    hv_status: HvStatus,
    in_irq: bool,
    pub nmi_requested: bool,

    #[cfg(feature = "log")]
    vblank_count: f32,
    #[cfg(feature = "log")]
    log_time: Instant,
}

impl Counters {
    pub fn new(stat78: Stat78) -> Self {
        let (vblank_start, vblank_end) = if stat78.is_pal() {
            (PAL_HEIGHT + 1, PAL_SCANLINES)
        } else {
            (NTSC_HEIGHT + 1, NTSC_SCANLINES)
        };
        Self {
            vertical_counter: 0,
            vblank_start,
            vblank_end,
            cycles_per_scanline: super::SCANLINE_CYCLES,
            frame_counter: 0,
            ophct_latch: false,
            opvct_latch: false,
            output_horizontal_counter: 0,
            output_vertical_counter: 0,
            h_timer_target: 0x1FF,
            v_timer_target: 0x1FF,
            nmitimen: Nmitimen(0),
            rdnmi: Rdnmi(0x2),
            stat78,
            hv_status: HvStatus(0),
            in_irq: false,
            nmi_requested: false,
            #[cfg(feature = "log")]
            vblank_count: 0.0,
            #[cfg(feature = "log")]
            log_time: Instant::now(),
        }
    }

    pub(crate) fn set_hblank(&mut self, in_hblank: bool) {
        self.hv_status.set_in_hblank(in_hblank);
    }

    pub(crate) fn h_dot(&self, time: u64) -> u16 {
        let as_master_cycles = time % (self.cycles_per_scanline as u64);
        (as_master_cycles / 4) as u16
    }

    pub(crate) fn software_latch(&mut self, time: u64) {
        if !self.stat78.counter_latch() {
            self.output_vertical_counter = self.vertical_counter as u16;
            self.output_horizontal_counter = self.h_dot(time);
        }
        self.stat78.set_counter_latch(true);
    }

    pub(crate) fn reset_latches(&mut self) {
        self.ophct_latch = false;
        self.opvct_latch = false;
        self.stat78.set_counter_latch(false);
    }

    pub(crate) fn check_counters_timer_hit(&mut self, time: u64) {
        let h_dot = self.h_dot(time);
        self.in_irq = match self.nmitimen.hv_timer_mode() {
            0b00 => false,
            0b01 => h_dot == self.h_timer_target,
            0b10 => self.vertical_counter as u16 == self.v_timer_target && h_dot == 0,
            0b11 => {
                self.vertical_counter as u16 == self.v_timer_target && h_dot == self.h_timer_target
            }
            _ => unreachable!(),
        }
    }

    pub(crate) fn start_frame(&mut self, overscan: bool, interlacing: bool) {
        self.frame_counter += 1;
        self.stat78.set_odd_frame(self.frame_counter & 1 == 1);
        self.vertical_counter = 0;
        self.rdnmi.set_in_nmi(false);
        self.hv_status.set_in_vblank(false);
        self.vblank_start = if overscan { PAL_HEIGHT } else { NTSC_HEIGHT } + 1;
        self.vblank_end = if self.stat78.is_pal() {
            PAL_SCANLINES
        } else {
            NTSC_SCANLINES
        } + usize::from(interlacing && !self.stat78.odd_frame());

        #[cfg(feature = "log")]
        {
            self.vblank_count += 1.0;
            if self.log_time.elapsed().as_secs() >= 2 {
                log::warn!(
                    "PPU: {:0.2} VBlank/sec",
                    self.vblank_count / self.log_time.elapsed().as_secs_f32()
                );
                self.vblank_count = 0.0;
                self.log_time = Instant::now();
            }
        }
    }

    pub(crate) fn start_scanline(&mut self, interlacing: bool) {
        if self.vertical_counter == 311
            && self.stat78.odd_frame()
            && self.stat78.is_pal()
            && interlacing
        {
            self.cycles_per_scanline = SCANLINE_CYCLES + 4;
        } else if self.vertical_counter == 240
            && self.stat78.odd_frame()
            && !self.stat78.is_pal()
            && !interlacing
        {
            self.cycles_per_scanline = SCANLINE_CYCLES - 4;
        } else {
            self.cycles_per_scanline = SCANLINE_CYCLES;
        }
    }

    pub(crate) fn enter_vblank(&mut self) {
        self.hv_status.set_in_vblank(true);
        self.rdnmi.set_in_nmi(true);

        if self.nmitimen.nmi_enabled() {
            self.nmi_requested = true;
        }
    }
}

impl Ppu {
    pub fn ophct_read(&mut self) -> u8 {
        if self.counters.ophct_latch {
            self.ppu2_mdr =
                (self.ppu2_mdr & 0xFE) | (self.counters.output_horizontal_counter.high_byte() & 1);
        } else {
            self.ppu2_mdr = self.counters.output_horizontal_counter.low_byte();
        }
        self.counters.ophct_latch = !self.counters.ophct_latch;
        self.ppu2_mdr
    }

    pub fn opvct_read(&mut self) -> u8 {
        if self.counters.opvct_latch {
            self.ppu2_mdr =
                (self.ppu2_mdr & 0xFE) | (self.counters.output_vertical_counter.high_byte() & 1);
        } else {
            self.ppu2_mdr = self.counters.output_vertical_counter.low_byte();
        }
        self.counters.opvct_latch = !self.counters.opvct_latch;
        self.ppu2_mdr
    }

    pub fn set_h_timer_low(&mut self, val: u8) {
        self.counters.h_timer_target.set_low_byte(val);
    }

    pub fn set_h_timer_high(&mut self, val: u8) {
        self.counters.h_timer_target.set_high_byte(val);
    }

    pub fn set_v_timer_low(&mut self, val: u8) {
        self.counters.v_timer_target.set_low_byte(val);
    }

    pub fn set_v_timer_high(&mut self, val: u8) {
        self.counters.v_timer_target.set_high_byte(val);
    }

    pub fn write_nmitien(&mut self, val: u8) {
        let nmitimen = Nmitimen(val);
        let _joypad_enable = nmitimen.joypad_enable(); // todo when implementing joypad
        self.counters.nmitimen = nmitimen;
        // self.counters.check_counters_timer_hit();
    }

    pub fn read_nmi_flag(&mut self) -> u8 {
        let result = self.counters.rdnmi.0;
        self.counters.rdnmi.set_in_nmi(false);
        result
    }

    pub fn read_hv_status(&self) -> u8 {
        self.counters.hv_status.0
    }

    pub fn read_irq_flag(&mut self) -> u8 {
        let result = u8::from(self.counters.in_irq) << 7;
        self.counters.in_irq = false;
        result
    }

    pub fn status78_read(&mut self) -> u8 {
        self.ppu2_mdr = (self.ppu2_mdr & 0x20) | self.counters.stat78.0;
        self.counters.reset_latches();
        self.ppu2_mdr
    }

    pub fn nmi_requested(&mut self) -> bool {
        let nmi_requested = self.counters.nmi_requested;
        self.counters.nmi_requested = false;
        nmi_requested
    }

    pub fn is_in_irq(&mut self) -> bool {
        let in_irq = self.counters.in_irq;
        self.counters.in_irq = false;
        in_irq
    }
}
