use crate::{utils::int_traits::ManipulateU16, w65c816::addressing::Address};

use super::{system_bus::SystemBus, Access};

bitfield! {
    #[derive(Copy, Clone)]
    pub struct Parameters(u8) {
        transfer_pattern: u8 @ 0..=2,
        address_adjust_mode: u8 @ 3..=4,
        h_indirect: bool @ 6,
        direction: bool @ 7,
    }
}

#[derive(Copy, Clone)]
pub struct Channel {
    parameters: Parameters,
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
    pub enable_channels: u8,
    h_enable_channels: u8,
}

impl Dma {
    pub(super) fn new() -> Self {
        Dma {
            channels: [Channel {
                parameters: Parameters(0xFF),
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

    pub fn do_dma(bus: &mut SystemBus) {
        for index in 0..8 {
            if bus.dma.enable_channels & (1 << index) == 0 {
                continue;
            }
            let channel = bus.dma.channels[index];
            let count = if channel.byte_count_or_h_indirect_addr == 0 {
                0x10000
            } else {
                channel.byte_count_or_h_indirect_addr as usize
            };

            let pattern: Vec<u8> = match channel.parameters.transfer_pattern() {
                0 => vec![0],
                1 => vec![0, 1],
                2 | 6 => vec![0, 0],
                3 | 7 => vec![0, 0, 1, 1],
                4 => vec![0, 1, 2, 3],
                5 => vec![0, 1, 0, 1],
                _ => unreachable!(),
            };
            // log::warn!("Channel {index} will transfer {count} Bytes");
            for i in 0..count {
                let bank = bus.dma.channels[index].a_bank_or_h_table_bank;
                let offset = bus.dma.channels[index].a_addr_or_h_table_addr;
                let a_addr = Address::new(offset, bank);
                let byte = channel.b_addr.wrapping_add(pattern[i % pattern.len()]);

                //WRAM to WRAM is invalid
                if byte == 0x80
                    && ((u32::from(a_addr) & 0x00FE_0000) == 0x007E_0000
                        || (u32::from(a_addr) & 0x0040_E000) == 0)
                {
                    continue;
                }

                let b_addr = 0x2100 | u16::from(byte);

                if channel.parameters.direction() {
                    let data = bus.read_b(b_addr);
                    bus.write::<true>(a_addr, data);
                } else {
                    let data = bus.read::<true>(a_addr);
                    bus.write_b(b_addr, data);
                }

                match channel.parameters.address_adjust_mode() {
                    0 => bus.dma.channels[index].a_addr_or_h_table_addr = offset.wrapping_add(1),
                    2 => bus.dma.channels[index].a_addr_or_h_table_addr = offset.wrapping_sub(1),
                    _ => (),
                }
            }
            bus.dma.channels[index].byte_count_or_h_indirect_addr = 0;
            bus.add_cycles(8 + (8 * count) + 2);
        }
        bus.dma.enable_channels = 0;
    }
}

impl Access for Dma {
    fn read(&mut self, addr: u16) -> Option<u8> {
        let channel = self.channels[(addr >> 4 & 7) as usize];
        match addr & 0xF {
            0x0 => Some(channel.parameters.0),
            0x1 => Some(channel.b_addr),
            0x2 => Some(channel.a_addr_or_h_table_addr.low_byte()),
            0x3 => Some(channel.a_addr_or_h_table_addr.high_byte()),
            0x4 => Some(channel.a_bank_or_h_table_bank),
            0x5 => Some(channel.byte_count_or_h_indirect_addr.low_byte()),
            0x6 => Some(channel.byte_count_or_h_indirect_addr.high_byte()),
            0x7 => Some(channel.h_indirect_bank),
            0x8 => Some(channel.h_curr_addr.low_byte()),
            0x9 => Some(channel.h_curr_addr.high_byte()),
            0xA => Some(channel.h_reload_or_scanline_count),
            0xB | 0xF => Some(channel.unused),
            _ => None,
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x420B => self.enable_channels = data,
            0x420C => self.h_enable_channels = data,
            _ => {
                let channel = &mut self.channels[(addr >> 4 & 7) as usize];
                match addr & 0xF {
                    0x0 => channel.parameters = Parameters(data),
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
