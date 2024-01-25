use proc_bitfield::bitfield;

use crate::utils::int_traits::ManipulateU16;

use super::access::Access;

bitfield! {
    #[derive(Copy, Clone)]
    pub struct Configuration(u8) {
        pub transfer_pattern: u8 @ 0..=2,
        pub address_adjust_mode: u8 @ 3..=4,
        pub h_indirect: bool @ 6,
        pub direction: bool @ 7,
    }
}

#[derive(Copy, Clone)]
pub struct Channel {
    configuration: Configuration,
    b_addr: u8,
    a_addr_or_h_table_addr: u16,
    a_bank_or_h_table_bank: u8,
    byte_count_or_h_indirect_addr: u16,
    h_indirect_bank: u8,
    h_curr_addr: u16,
    h_reload_or_scanline_count: u8,
    unused: u8,
}

pub struct Dma {
    channels: [Channel; 8],
    enable_channels: u8,
    h_enable_channels: u8,
}

impl Dma {
    pub fn new() -> Self {
        Dma {
            channels: [Channel {
                configuration: Configuration(0xFF),
                b_addr: 0xFF,
                a_addr_or_h_table_addr: 0xFFFF,
                a_bank_or_h_table_bank: 0xFF,
                byte_count_or_h_indirect_addr: 0xFFFF,
                h_indirect_bank: 0xFF,
                h_curr_addr: 0xFF,
                h_reload_or_scanline_count: 0xFF,
                unused: 0xFF,
            }; 8],
            enable_channels: 0,
            h_enable_channels: 0,
        }
    }
}

impl Access for Dma {
    fn read(&mut self, addr: u16) -> u8 {
        let channel = self.channels[(addr >> 4 & 7) as usize];
        match addr & 0xF {
            0x0 => channel.configuration.0,
            0x1 => channel.b_addr,
            0x2 => channel.a_addr_or_h_table_addr.low_byte(),
            0x3 => channel.a_addr_or_h_table_addr.high_byte(),
            0x4 => channel.a_bank_or_h_table_bank,
            0x5 => channel.byte_count_or_h_indirect_addr.low_byte(),
            0x6 => channel.byte_count_or_h_indirect_addr.high_byte(),
            0x7 => channel.h_indirect_bank,
            0x8 => channel.h_curr_addr.low_byte(),
            0x9 => channel.h_curr_addr.high_byte(),
            0xA => channel.h_reload_or_scanline_count,
            0xB | 0xF => channel.unused,
            _ => 0, // todo addr as 0x43nC 0x43nD are open bus and should return mdr
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x420B => self.enable_channels = data,
            0x420C => self.h_enable_channels = data,
            _ => {
                let mut channel = self.channels[(addr >> 4 & 7) as usize];
                match addr & 0xF {
                    0x0 => channel.configuration = Configuration(data),
                    0x1 => channel.b_addr = data,
                    0x2 => channel.a_addr_or_h_table_addr.set_low_byte(data),
                    0x3 => channel.a_addr_or_h_table_addr.set_high_byte(data),
                    0x4 => channel.a_bank_or_h_table_bank = data,
                    0x5 => channel.byte_count_or_h_indirect_addr.set_low_byte(data),
                    0x6 => channel.byte_count_or_h_indirect_addr.set_high_byte(data),
                    0x7 => channel.h_indirect_bank = data,
                    0x8 => channel.h_curr_addr.set_low_byte(data),
                    0x9 => channel.h_curr_addr.set_high_byte(data),
                    0xA => channel.h_reload_or_scanline_count = data,
                    0xB | 0xF => channel.unused = data,
                    _ => {}
                }
            }
        }
    }
}
