use crate::utils::int_traits::ManipulateU16;

use super::Ppu;

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

pub struct Counters {
    pub vertical_counter: u16,
    pub vblank_start: usize,
    pub vblank_end: u16,
    pub elapsed_cycles: u16,
    pub cycles_per_scanline: u16,

    counter_latch: bool,
    ophct_latch: bool,
    opvct_latch: bool,
    output_horizontal_counter: u16,
    output_vertical_counter: u16,
    h_timer_target: u16,
    v_timer_target: u16,

    nmitimen: Nmitimen,
    rdnmi: Rdnmi,
    hv_status: HvStatus,
    in_irq: bool,
}

impl Counters {
    pub fn new(vblank_start: usize, vblank_end: u16) -> Self {
        Self {
            vertical_counter: 0,
            vblank_start,
            vblank_end,
            elapsed_cycles: 0,
            cycles_per_scanline: super::SCANLINE_CYCLES,
            counter_latch: false,
            ophct_latch: false,
            opvct_latch: false,
            output_horizontal_counter: 0,
            output_vertical_counter: 0,
            h_timer_target: 0x1FF,
            v_timer_target: 0x1FF,
            nmitimen: Nmitimen(0),
            rdnmi: Rdnmi(0x2),
            hv_status: HvStatus(0),
            in_irq: false,
        }
    }

    fn h_dot(&self) -> u16 {
        self.elapsed_cycles % (self.cycles_per_scanline / 4)
    }

    pub fn software_latch(&mut self) {
        if !self.counter_latch {
            self.output_vertical_counter = self.vertical_counter;
            self.output_horizontal_counter = self.h_dot();
        }
        self.counter_latch = true;
    }

    pub fn reset_latches(&mut self) {
        self.ophct_latch = false;
        self.opvct_latch = false;
        self.counter_latch = false;
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
        let joypad_enable = nmitimen.joypad_enable(); // todo when implementing joypad
        self.nmi_requested = !self.counters.nmitimen.nmi_enabled()
            && nmitimen.nmi_enabled()
            && self.counters.rdnmi.in_nmi();
        self.counters.nmitimen = nmitimen;
    }

    pub fn read_nmi_flag(&mut self) -> u8 {
        let result = self.counters.rdnmi.0;
        self.counters.rdnmi.set_in_nmi(false);
        result
    }

    pub fn read_hv_status(&mut self) -> u8 {
        self.counters.hv_status.0
    }

    pub fn read_irq_flag(&mut self) -> u8 {
        let result = (self.counters.in_irq as u8) << 7;
        self.counters.in_irq = false;
        result
    }

    pub fn is_in_irq(&self) -> bool {
        self.counters.in_irq
    }
}
