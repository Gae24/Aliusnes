use std::collections::HashMap;

use aliusnes::{bus::Bus, w65c816::addressing::Address};

#[derive(Default)]
pub struct TomHarteBus {
    cycles: usize,
    pub memory: HashMap<u32, u8>,
}

impl TomHarteBus {
    pub fn read(&self, addr: Address) -> u8 {
        self.memory.get(&addr.into()).copied().unwrap_or_default()
    }

    pub fn write(&mut self, addr: Address, data: u8) {
        self.memory.insert(addr.into(), data);
    }
}

impl Bus for TomHarteBus {
    fn read_and_tick(&mut self, addr: Address) -> u8 {
        self.read(addr)
    }

    fn write_and_tick(&mut self, addr: Address, data: u8) {
        self.write(addr, data);
    }

    fn add_io_cycles(&mut self, cycles: usize) {
        self.cycles += cycles * 6;
    }
}
