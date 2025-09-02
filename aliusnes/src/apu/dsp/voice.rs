use std::hint::unreachable_unchecked;

use crate::utils::int_traits::ManipulateU16;

bitfield! {
    #[derive(Clone, Copy)]
    struct Adsr(u16) {
        attack_rate: u8 @ 0..=3,
        decay_rate: u8 @ 4..=6,
        enabled: bool @ 7,
        sustain_rate: u8 @ 8..=12,
        sustain_level: u8 @ 13..=15,
    }
}

bitfield! {
    #[derive(Clone, Copy)]
    struct Gain(u8) {
        fixed_volume: u8 @ 0..=6,
        rate: u8 @ 0..=4,
        mode: u8 @ 5..=6,
        use_custom: bool @ 7,
    }
}

#[derive(Clone, Copy)]
pub struct Voice {
    left_channel_volume: i8,
    right_channel_volume: i8,
    sample_pitch: u16,
    sample_source_entry: u8,
    adsr: Adsr,
    gain: Gain,
    current_envelope: u8,
    current_sample: u8,
}

impl Voice {
    pub(crate) fn new() -> Voice {
        Voice {
            left_channel_volume: 0,
            right_channel_volume: 0,
            sample_pitch: 0,
            sample_source_entry: 0,
            adsr: Adsr(0),
            gain: Gain(0),
            current_envelope: 0,
            current_sample: 0,
        }
    }

    pub(crate) fn read(&self, low_nibble: usize) -> u8 {
        match low_nibble {
            0x0 => self.left_channel_volume as u8,
            0x1 => self.right_channel_volume as u8,
            0x2 => self.sample_pitch.low_byte(),
            0x3 => self.sample_pitch.high_byte(),
            0x4 => self.sample_source_entry,
            0x5 => self.adsr.0.low_byte(),
            0x6 => self.adsr.0.high_byte(),
            0x7 => self.gain.0,
            0x8 => self.current_envelope,
            0x9 => self.current_sample,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub(crate) fn write(&mut self, low_nibble: usize, data: u8) {
        match low_nibble {
            0x0 => self.left_channel_volume = data as i8,
            0x1 => self.right_channel_volume = data as i8,
            0x2 => self.sample_pitch.set_low_byte(data),
            0x3 => self.sample_pitch.set_high_byte(data),
            0x4 => self.sample_source_entry = data,
            0x5 => self.adsr.0.set_low_byte(data),
            0x6 => self.adsr.0.set_high_byte(data),
            0x7 => self.gain = Gain(data),
            _ => unsafe { unreachable_unchecked() },
        }
    }
}
