use crate::apu::spc700::Spc700;
use crate::bus::{Access, Bus};
use crate::w65c816::addressing::Address;

pub mod spc700;

pub struct Apu {
    spc700: Spc700<ApuBus>,
    bus: ApuBus,
}

struct ApuBus {
    aram: [u8; 0x10000],
    apuio: [u8; 4],
    cpuio: [u8; 4],
}

impl Apu {
    pub fn new() -> Apu {
        Apu {
            spc700: Spc700::new(),
            bus: ApuBus {
                aram: [0; 0x10000],
                apuio: [0; 4],
                cpuio: [0; 4],
            },
        }
    }

    pub fn step(&mut self) {
        let Apu {
            ref mut bus,
            ref mut spc700,
        } = self;

        spc700.step(bus);
    }
}

impl Access for Apu {
    fn read(&mut self, addr: u16, _: u64) -> Option<u8> {
        match addr {
            0x2140..=0x2143 => Some(self.bus.cpuio[addr as usize & 3]),
            _ => None,
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        if let 0x2140..=0x2143 = addr {
            self.bus.apuio[addr as usize & 3] = data
        }
    }
}

impl ApuBus {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            // write-only area
            0x00F0 | 0x00F1 | 0x00FA..=0x00FC => 0,
            0x00F2 => todo!(),
            0x00F3 => todo!(),
            0x00F4..=0x00F7 => self.apuio[addr as usize & 3],
            0x00FD => todo!(),
            0x00FE => todo!(),
            0x00FF => todo!(),
            _ => self.aram[addr as usize],
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x00F0 => todo!(),
            0x00F1 => todo!(),
            0x00F2 => todo!(),
            0x00F3 => todo!(),
            0x00F4..=0x00F7 => self.cpuio[addr as usize & 3] = data,
            0x00FA => todo!(),
            0x00FB => todo!(),
            0x00FC => todo!(),
            // read-only area
            0x00FD..=0x00FF => (),
            _ => self.aram[addr as usize] = data,
        }
    }
}

impl Bus for ApuBus {
    fn peek_at(&self, _addr: Address) -> Option<u8> {
        todo!()
    }

    fn read_and_tick(&mut self, addr: Address) -> u8 {
        self.read(addr.offset)
    }

    fn write_and_tick(&mut self, addr: Address, data: u8) {
        self.write(addr.offset, data)
    }

    fn add_io_cycles(&mut self, _cycles: usize) {
        todo!()
    }

    fn fired_nmi(&mut self) -> bool {
        todo!()
    }

    fn fired_irq(&mut self) -> bool {
        todo!()
    }
}
