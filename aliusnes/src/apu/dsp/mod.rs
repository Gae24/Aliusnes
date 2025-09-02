use crate::apu::dsp::voice::Voice;

mod voice;

bitfield! {
    #[derive(Clone, Copy)]
    struct Flags(u8) {
        pub noise_frequency: u8 @ 0..=4,
        pub disable_echo_write: bool @ 5,
        pub mute_all: bool @ 6,
        pub soft_reset: bool @ 7,
    }
}

pub(crate) struct Dsp {
    dsp_addr: usize,
    voices: [Voice; 8],
    left_main_channel_volume: i8,
    right_main_channel_volume: i8,
    left_echo_volume: i8,
    right_echo_volume: i8,
    key_on: u8,
    key_off: u8,
    flags: Flags,
    end_flag_channel_mask: u8,
    echo_feedback: i8,

    unused: u8,

    pitch_modulation_channel_mask: u8,
    noise_enabled_channel_mask: u8,
    echo_enabled_channel_mask: u8,

    pointer_to_sample_directory: u8,
    pointer_to_echo_buffer: u8,
    echo_delay: u8,
    echo_filter_coefficients: [i8; 8],
}

impl Dsp {
    pub(crate) fn new() -> Dsp {
        Dsp {
            dsp_addr: 0,
            voices: [Voice::new(); 8],
            left_main_channel_volume: 0,
            right_main_channel_volume: 0,
            left_echo_volume: 0,
            right_echo_volume: 0,
            key_on: 0,
            key_off: 0,
            flags: Flags(0),
            end_flag_channel_mask: 0,
            echo_feedback: 0,
            unused: 0,
            pitch_modulation_channel_mask: 0,
            noise_enabled_channel_mask: 0,
            echo_enabled_channel_mask: 0,
            pointer_to_sample_directory: 0,
            pointer_to_echo_buffer: 0,
            echo_delay: 0,
            echo_filter_coefficients: [0; 8],
        }
    }

    pub(crate) fn set_dsp_addr(&mut self, value: u8) {
        self.dsp_addr = value.into();
    }

    pub(crate) fn read_dsp_addr(&self) -> u8 {
        self.dsp_addr as u8
    }

    pub(crate) fn read(&self) -> u8 {
        let addr = self.dsp_addr & 0x7F;
        let index = addr >> 4;

        match addr & 0xF {
            low_nibble @ 0x0..=0x9 => return self.voices[index].read(low_nibble),
            0xC => match index {
                0x0 => return self.left_main_channel_volume as u8,
                0x1 => return self.right_main_channel_volume as u8,
                0x2 => return self.left_echo_volume as u8,
                0x3 => return self.right_echo_volume as u8,
                0x4 => return self.key_on,
                0x5 => return self.key_off,
                0x6 => return self.flags.0,
                0x7 => return self.end_flag_channel_mask,
                _ => (),
            },
            0xD => match index {
                0x0 => return self.echo_feedback as u8,
                0x1 => return self.unused,
                0x2 => return self.pitch_modulation_channel_mask,
                0x3 => return self.noise_enabled_channel_mask,
                0x4 => return self.echo_enabled_channel_mask,
                0x5 => return self.pointer_to_sample_directory,
                0x6 => return self.pointer_to_echo_buffer,
                0x7 => return self.echo_delay,
                _ => (),
            },
            0xF => return self.echo_filter_coefficients[index] as u8,
            _ => (),
        }

        println!("Read from invalid DSP register: {:#04X}", self.dsp_addr);
        0
    }

    pub(crate) fn write(&mut self, data: u8) {
        if self.dsp_addr >= 0x80 {
            return;
        }
        let addr = self.dsp_addr & 0x7F;
        let index = addr >> 4;

        match addr & 0xF {
            low_nibble @ 0x0..=0x7 => self.voices[index].write(low_nibble, data),
            0xC => match index {
                0x0 => self.left_main_channel_volume = data as i8,
                0x1 => self.right_main_channel_volume = data as i8,
                0x2 => self.left_echo_volume = data as i8,
                0x3 => self.right_echo_volume = data as i8,
                0x4 => self.key_on = data,
                0x5 => self.key_off = data,
                0x6 => self.flags = Flags(data),
                _ => (),
            },
            0xD => match index {
                0x0 => self.echo_feedback = data as i8,
                0x1 => self.unused = data,
                0x2 => self.pitch_modulation_channel_mask = data,
                0x3 => self.noise_enabled_channel_mask = data,
                0x4 => self.echo_enabled_channel_mask = data,
                0x5 => self.pointer_to_sample_directory = data,
                0x6 => self.pointer_to_echo_buffer = data,
                0x7 => self.echo_delay = data,
                _ => (),
            },
            0xF => self.echo_filter_coefficients[index] = data as i8,
            _ => (),
        }
    }
}
