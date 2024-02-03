mod access;
pub mod dma;
mod math;
mod wram;

use self::{access::Access, dma::Dma, math::Math, wram::Wram};
use crate::{cart::Cart, utils::int_traits::ManipulateU16};

pub struct Bus {
    mdr: u8,
    cart: Cart,
    pub dma: Dma,
    math: Math,
    wram: Wram,
}

impl Bus {
    pub fn new(cart: Cart) -> Self {
        Self {
            mdr: 0,
            cart,
            dma: Dma::new(),
            math: Math::new(),
            wram: Wram::new(),
        }
    }

    pub fn read(&mut self, full_addr: u32) -> u8 {
        let bank = (full_addr >> 16) as u8;
        let addr = full_addr as u16;

        if let Some(val) = match bank {
            0x00..=0x3F | 0x80..=0xBF => match addr.high_byte() {
                0x00..=0x1F => Some(self.wram.ram[addr as usize & 0x1FFF]),
                0x21 | 0x40..=0x43 => match addr {
                    0x2134..=0x213F => todo!("ppu area"),
                    0x2140..=0x2143 => todo!("apu area"),
                    0x2180 => self.wram.read(addr),
                    0x4214..=0x4217 => self.math.read(addr),
                    0x4300..=0x437F => self.dma.read(addr),
                    _ => panic!("tried to read at {:#0x}", addr),
                },
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

    pub fn write(&mut self, full_addr: u32, data: u8) {
        let bank = (full_addr >> 16) as u8;
        let addr = full_addr as u16;
        match bank {
            0x00..=0x3F | 0x80..=0xBF => match addr.high_byte() {
                0x00..=0x1F => return self.wram.ram[addr as usize & 0x1FFF] = data,
                0x21 | 0x40..=0x43 => {
                    return match addr {
                        0x2100..=0x2133 => todo!("ppu area"),
                        0x2140..=0x2143 => todo!("apu area"),
                        0x2180..=0x2183 => self.wram.write(addr, data),
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
}
