mod dma;
mod math;
mod mmio;

use self::math::Math;
use crate::{cart::Cart, wram::Wram};

pub struct Bus {
    mdr: u8,
    fast_rom_enabled: bool,
    cart: Cart,
    math: Math,
}

impl Bus {
    pub fn new(cart: Cart) -> Self {
        Self {
            mdr: 0,
            fast_rom_enabled: false,
            cart,
            math: Math::new(),
        }
    }

    pub fn read_b(&mut self, addr: u16) -> u8 {
        if let Some(val) = match addr.low_byte() {
            0x34..=0x3F => todo!("ppu area"),
            0x40..=0x43 => todo!("apu area"),
            0x80 => self.wram.read(addr),
            _ => None,
        } {
            self.mdr = val;
        } else {
            self.mdr = 0;
        }
        self.mdr
    }

    pub fn read<const DMA: bool>(&mut self, full_addr: u32) -> u8 {
        let bank = (full_addr >> 16) as u8;
        let addr = full_addr as u16;

        if let Some(val) = match bank {
            0x00..=0x3F | 0x80..=0xBF => match addr.high_byte() {
                0x00..=0x1F => Some(self.wram.ram[addr as usize & 0x1FFF]),
                0x21 => return self.read_b(addr),
                0x40..=0x43 => {
                    if DMA {
                        Some(0)
                    } else {
                        match addr {
                            0x4214..=0x4217 => self.math.read(addr),
                            0x4300..=0x437F => self.dma.read(addr),
                            _ => panic!("tried to read at {:#0x}", addr),
                        }
                    }
                }
                _ => None,
            },

            0x7E..=0x7F => Some(self.wram.ram[full_addr as usize & 0x1_FFFF]),
            _ => None,
        } {
            self.mdr = val;
            return self.mdr;
        }

        if let Some(val) = self.cart.read(bank, addr) {
            self.mdr = val;
        }
        self.mdr
    }

    pub fn write_b(&mut self, addr: u16, data: u8) {
        match addr.low_byte() {
            0x00..=0x33 => todo!("ppu area"),
            0x40..=0x43 => todo!("apu area"),
            0x80..=0x83 => self.wram.write(addr, data),
            _ => panic!("tried to write {:#0x} at {:#0x}", data, addr),
        }
    }

    pub fn write<const DMA: bool>(&mut self, full_addr: u32, data: u8) {
        self.mdr = data;
        let bank = (full_addr >> 16) as u8;
        let addr = full_addr as u16;

        match bank {
            0x00..=0x3F | 0x80..=0xBF => match addr.high_byte() {
                0x00..=0x1F => return self.wram.ram[addr as usize & 0x1FFF] = data,
                0x21 => return self.write_b(addr, data),
                0x40..=0x43 if !DMA => {
                    return match addr {
                        0x4202..=0x4206 => self.math.write(addr, data),
                        0x420B | 0x420C | 0x4300..=0x437f => self.dma.write(addr, data),
                        _ => panic!("tried to write {:#0x} at {:#0x}", data, addr),
                    }
                }
                _ => {}
            },
            0x7E..=0x7F => return self.wram.ram[full_addr as usize & 0x1_FFFF] = data,
            _ => {}
        }
        self.cart.write(bank, addr, data);
    }

    pub fn memory_access_cycles(&self, addr: u32) -> u32 {
        static FAST: u32 = 6;
        static SLOW: u32 = 8;
        static XSLOW: u32 = 12;

        let bank = (addr >> 16) as u8;
        let offset = addr as u16;

        match bank {
            0x40..=0x7F => SLOW,
            0xC0..=0xFF => {
                if self.fast_rom_enabled {
                    FAST
                } else {
                    SLOW
                }
            }
            _ => match offset {
                0x0000..=0x1FFF => SLOW,
                0x2000..=0x3FFF => FAST,
                0x4000..=0x41FF => XSLOW,
                0x4200..=0x5FFF => FAST,
                0x6000..=0x7FFF => SLOW,
                0x8000..=0xFFFF => {
                    if (0x80..0xBF).contains(&bank) && self.fast_rom_enabled {
                        FAST
                    } else {
                        SLOW
                    }
                }
            },
        }
    }
}
