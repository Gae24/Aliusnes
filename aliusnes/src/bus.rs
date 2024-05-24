use self::{access::Access, dma::Dma, math::Math, wram::Wram};
use crate::w65c816::addressing::Address;
use crate::w65c816::addressing::Address;
use crate::{cart::Cart, ppu::Ppu, utils::int_traits::ManipulateU16};

pub mod access;
pub mod dma;
mod math;
mod wram;

pub struct Bus {
    mdr: u8,
    fast_rom_enabled: bool,
    cart: Cart,
    pub dma: Dma,
    math: Math,
    ppu: Ppu,
    wram: Wram,
}

impl Bus {
    pub fn new(cart: Cart) -> Self {
        Self {
            mdr: 0,
            fast_rom_enabled: false,
            ppu: Ppu::new(cart.model),
            cart,
            dma: Dma::new(),
            math: Math::new(),
            wram: Wram::new(),
        }
    }

    pub fn tick(&mut self) {
        //todo apu, joypad, hdma

        self.ppu.tick();
    }

    pub fn read_b(&mut self, addr: u16) -> u8 {
        if let Some(val) = match addr.low_byte() {
            0x34..=0x3F => self.ppu.read(addr),
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

    pub fn read<const DMA: bool>(&mut self, full_addr: Address) -> u8 {
        let bank = full_addr.bank;
        let addr = full_addr.offset;

        if let Some(val) = match bank {
            0x00..=0x3F | 0x80..=0xBF => match addr.high_byte() {
                0x00..=0x1F => Some(self.wram.ram[addr as usize & 0x1FFF]),
                0x21 => return self.read_b(addr),
                0x40..=0x43 => {
                    if DMA {
                        Some(0)
                    } else {
                        match addr {
                            0x4210 => Some(self.ppu.read_nmi_flag() | (self.mdr & 0x70)),
                            0x4211 => Some(self.ppu.read_irq_flag() | (self.mdr & 0x7F)),
                            0x4212 => {
                                let joypad_autoread_status = false; // todo
                                Some(
                                    self.ppu.read_hv_status()
                                        | joypad_autoread_status as u8
                                        | (self.mdr & 0x3E),
                                )
                            }
                            0x4214..=0x4217 => self.math.read(addr),
                            0x4300..=0x437F => self.dma.read(addr),
                            _ => {
                                println!("Tried to read at {:#0x}", addr);
                                None
                            }
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
            0x00..=0x33 => self.ppu.write(addr, data),
            0x40..=0x43 => todo!("apu area"),
            0x80..=0x83 => self.wram.write(addr, data),
            _ => println!("Tried to write at {:#0x} val: {:#04x}", addr, data),
        }
    }

    pub fn write<const DMA: bool>(&mut self, full_addr: Address, data: u8) {
        self.mdr = data;
        let bank = full_addr.bank;
        let addr = full_addr.offset;
        match bank {
            0x00..=0x3F | 0x80..=0xBF => match addr.high_byte() {
                0x00..=0x1F => return self.wram.ram[addr as usize & 0x1FFF] = data,
                0x21 | 0x40..=0x43 => {
                    return match addr {
                        0x4200 => self.ppu.write_nmitien(data),
                        0x4202..=0x4206 => self.math.write(addr, data),
                        0x4207 => self.ppu.set_h_timer_low(data),
                        0x4208 => self.ppu.set_h_timer_high(data),
                        0x4209 => self.ppu.set_v_timer_low(data),
                        0x420A => self.ppu.set_v_timer_high(data),
                        0x420B | 0x420C | 0x4300..=0x437f => self.dma.write(addr, data),
                        0x420D => self.fast_rom_enabled = data & 1 != 0,
                        _ => println!("Tried to write at {:#0x} val: {:#04x}", addr, data),
                    }
                }
                _ => {}
            },

            0x7E..=0x7F => return self.wram.ram[full_addr as usize & 0x1_FFFF] = data,
            _ => {}
        }
        self.cart.write(bank, addr, data);
    }

    pub fn memory_access_cycles(&self, addr: Address) -> u32 {
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

    pub fn requested_nmi(&self) -> bool {
        self.ppu.nmi_requested
    }

    pub fn requested_irq(&self) -> bool {
        self.ppu.is_in_irq()
    }
}
