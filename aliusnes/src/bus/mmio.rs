use crate::bus::Bus;

impl Bus {
    pub fn read_mmio(&self, addr: u16) -> u8 {
        match addr {
            _ => panic!("tried to read at {:#0x}", addr),
        }
    }

    pub fn write_mmio(&mut self, addr: u16, val: u8) {}
}
