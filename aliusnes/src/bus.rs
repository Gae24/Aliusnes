use std::collections::HashMap;

use crate::w65c816::addressing::Address;

#[derive(Default)]
pub struct Bus {
    fast_rom_enabled: bool,
    pub memory: HashMap<u32, u8>,
}

impl Bus {
    pub fn read(&self, addr: Address) -> u8 {
        self.memory.get(&addr.into()).copied().unwrap_or_default()
    }

    pub fn write(&mut self, addr: Address, data: u8) {
        self.memory.insert(addr.into(), data);
    }

    pub fn memory_access_cycles(&self, addr: &Address) -> u32 {
        static FAST: u32 = 6;
        static SLOW: u32 = 8;
        static XSLOW: u32 = 12;

        match addr.bank {
            0x40..=0x7F => SLOW,
            0xC0..=0xFF => {
                if self.fast_rom_enabled {
                    FAST
                } else {
                    SLOW
                }
            }
            _ => match addr.offset {
                0x0000..=0x1FFF => SLOW,
                0x2000..=0x3FFF => FAST,
                0x4000..=0x41FF => XSLOW,
                0x4200..=0x5FFF => FAST,
                0x6000..=0x7FFF => SLOW,
                0x8000..=0xFFFF => {
                    if (0x80..0xBF).contains(&addr.bank) && self.fast_rom_enabled {
                        FAST
                    } else {
                        SLOW
                    }
                }
            },
        }
    }
}
