use crate::{cart::Cart, ppu::Ppu, utils::int_traits::ManipulateU16, w65c816::addressing::Address};

use super::{dma::Dma, math::Math, wram::Wram, Access, Bus};

pub struct SystemBus {
    mdr: u8,
    fast_rom_enabled: bool,
    cycles: usize,
    cart: Cart,
    pub dma: Dma,
    math: Math,
    pub ppu: Ppu,
    wram: Wram,
    dummy_apu: [u8; 4],
}

impl SystemBus {
    pub fn new(cart: Cart) -> Self {
        Self {
            mdr: 0,
            fast_rom_enabled: false,
            cycles: 0,
            ppu: Ppu::new(cart.model),
            cart,
            dma: Dma::new(),
            math: Math::new(),
            wram: Wram::new(),
            dummy_apu: [0xAA, 0, 0, 0],
        }
    }

    pub fn tick(&mut self) {
        //todo apu, joypad, hdma

        let ticks = self.cycles;
        self.cycles = 0;

        for _ in 0..ticks {
            self.ppu.tick();
        }
    }

    pub fn add_cycles(&mut self, cycles: usize) {
        self.cycles += cycles;
    }

    pub fn read_b(&mut self, addr: u16) -> u8 {
        if let Some(val) = match addr.low_byte() {
            0x34..=0x3F => self.ppu.read(addr),
            0x40..=0x43 => {
                let ch = ((addr - 0x2140) % 4) as usize;

                let value = self.dummy_apu[ch];
                self.dummy_apu[ch] = match ch {
                    0 => 0xAA,
                    1 => 0xBB,
                    _ => 0,
                };

                Some(value)
            }
            0x80 => self.wram.read(addr),
            _ => None,
        } {
            val
        } else {
            self.mdr
        }
    }

    fn peek(&self, addr: Address) -> Option<u8> {
        let bank = addr.bank;
        let page = addr.offset;

        match bank {
            0x00..=0x3F | 0x80..=0xBF => {
                if let 0x00..=0x1F = page.high_byte() {
                    return Some(self.wram.ram[page as usize & 0x1FFF]);
                }
            }
            0x7E..=0x7F => return Some(self.wram.ram[u32::from(addr) as usize & 0x1_FFFF]),
            _ => {}
        }

        self.cart.read(bank.into(), page.into())
    }

    pub fn read<const DMA: bool>(&mut self, addr: Address) -> u8 {
        let bank = addr.bank;
        let page = addr.offset;

        if DMA && (bank & 0x40) == 0 && matches!(page.high_byte(), 0x21 | 0x40 | 0x42 | 0x43) {
            self.mdr = 0;
            return self.mdr;
        }

        if let Some(val) = match bank {
            0x00..=0x3F | 0x80..=0xBF => match page.high_byte() {
                0x00..=0x1F => Some(self.wram.ram[page as usize & 0x1FFF]),
                0x21 => Some(self.read_b(page)),
                0x40..=0x43 => {
                    match page {
                        0x4210 => Some(self.ppu.read_nmi_flag() | (self.mdr & 0x70)),
                        0x4211 => Some(self.ppu.read_irq_flag() | (self.mdr & 0x7F)),
                        0x4212 => {
                            let joypad_autoread_status = false; // todo
                            Some(
                                self.ppu.read_hv_status()
                                    | u8::from(joypad_autoread_status)
                                    | (self.mdr & 0x3E),
                            )
                        }
                        0x4214..=0x4217 => self.math.read(page),
                        // TODO joypad registers
                        0x4218..=0x421F => Some(0),
                        0x4300..=0x437F => self.dma.read(page),
                        _ => {
                            println!("Tried to read at {page:#0x}");
                            None
                        }
                    }
                }
                _ => None,
            },

            0x7E..=0x7F => Some(self.wram.ram[u32::from(addr) as usize & 0x1_FFFF]),
            _ => None,
        } {
            self.mdr = val;
            return self.mdr;
        }

        if let Some(val) = self.cart.read(bank.into(), page.into()) {
            self.mdr = val;
        }
        self.mdr
    }

    pub fn write_b(&mut self, addr: u16, data: u8) {
        match addr.low_byte() {
            0x00..=0x33 => self.ppu.write(addr, data),
            0x40..=0x43 => {
                let ch = ((addr - 0x2140) % 4) as usize;
                self.dummy_apu[ch] = data;
            }
            0x80..=0x83 => self.wram.write(addr, data),
            _ => println!("Tried to write at {addr:#0x} val: {data:#04x}"),
        }
    }

    pub fn write<const DMA: bool>(&mut self, addr: Address, data: u8) {
        self.mdr = data;
        let bank = addr.bank;
        let page = addr.offset;
        match bank {
            0x00..=0x3F | 0x80..=0xBF => match page.high_byte() {
                0x00..=0x1F => return self.wram.ram[page as usize & 0x1FFF] = data,
                0x21 if !DMA => self.write_b(page, data),
                0x40..=0x43 if !DMA => {
                    return match page {
                        0x4200 => self.ppu.write_nmitien(data),
                        0x4202..=0x4206 => self.math.write(page, data),
                        0x4207 => self.ppu.set_h_timer_low(data),
                        0x4208 => self.ppu.set_h_timer_high(data),
                        0x4209 => self.ppu.set_v_timer_low(data),
                        0x420A => self.ppu.set_v_timer_high(data),
                        0x420B | 0x420C | 0x4300..=0x437f => self.dma.write(page, data),
                        0x420D => self.fast_rom_enabled = data & 1 != 0,
                        _ => println!("Tried to write at {page:#0x} val: {data:#04x}"),
                    }
                }
                _ => {}
            },

            0x7E..=0x7F => return self.wram.ram[u32::from(addr) as usize & 0x1_FFFF] = data,
            _ => {}
        }
        self.cart.write(bank.into(), page.into(), data);
    }

    pub fn memory_access_cycles(&self, addr: &Address) -> u32 {
        const FAST: u32 = 6;
        const SLOW: u32 = 8;
        const XSLOW: u32 = 12;

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
                0x0000..=0x1FFF | 0x6000..=0x7FFF => SLOW,
                0x2000..=0x3FFF | 0x4200..=0x5FFF => FAST,
                0x4000..=0x41FF => XSLOW,
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

impl Bus for SystemBus {
    fn peek_at(&self, addr: Address) -> Option<u8> {
        self.peek(addr)
    }

    fn read_and_tick(&mut self, addr: Address) -> u8 {
        self.cycles += self.memory_access_cycles(&addr) as usize;
        self.read::<false>(addr)
    }

    fn write_and_tick(&mut self, addr: Address, data: u8) {
        self.cycles += self.memory_access_cycles(&addr) as usize;
        self.write::<false>(addr, data);
    }

    fn add_io_cycles(&mut self, cycles: usize) {
        self.cycles += cycles * 6;
    }

    fn fired_nmi(&mut self) -> bool {
        self.ppu.nmi_requested()
    }

    fn fired_irq(&mut self) -> bool {
        self.ppu.is_in_irq()
    }
}
