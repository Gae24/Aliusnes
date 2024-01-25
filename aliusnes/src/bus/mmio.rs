use crate::bus::Bus;

use super::access::Access;

impl Bus {
    pub fn read_mmio(&self, addr: u16) -> u8 {
        match addr {
            0x2134..=0x213F => todo!("ppu area"),
            0x2140..=0x2143 =>  todo!("apu area"),
            0x2180 => todo!("wram area"),
            0x4214..=0x4217 => self.math.read(addr),
            0x4300..=0x437F => self.dma.read(addr),
            _ => panic!("tried to read at {:#0x}", addr),
        }
    }

    pub fn write_mmio(&mut self, addr: u16, val: u8) {
        match addr {
            0x2100..=0x2133 => todo!("ppu area"),
            0x2140..=0x2143 =>  todo!("apu area"),
            0x2180..=0x2183 => todo!("wram area"),
            0x4204..=0x4206 => self.math.write(addr, val),
            0x4300..=0x437f => todo!("dma area"),
            _ => panic!("tried to write {:#0x} at {:#0x}", val, addr),
        }
    }
}
