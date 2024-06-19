use aliusnes::{bus::Bus, w65c816::addressing::Address};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cycle {
    Read(u32, Option<u8>),
    Write(u32, u8),
    Internal,
}

#[derive(Default)]
pub struct TomHarteBus {
    pub cycles: Vec<Cycle>,
    pub memory: std::collections::HashMap<u32, u8>,
}

impl TomHarteBus {
    pub fn read(&self, addr: Address) -> Option<u8> {
        self.memory.get(&addr.into()).copied()
    }

    pub fn write(&mut self, addr: Address, data: u8) {
        self.memory.insert(addr.into(), data);
    }
}

impl Bus for TomHarteBus {
    fn peek_at(&self, addr: Address) -> Option<u8> {
        self.read(addr)
    }

    fn read_and_tick(&mut self, addr: Address) -> u8 {
        let val = self.read(addr);
        self.cycles.push(Cycle::Read(addr.into(), val));
        val.unwrap_or_default()
    }

    fn write_and_tick(&mut self, addr: Address, data: u8) {
        self.cycles.push(Cycle::Write(addr.into(), data));
        self.write(addr, data);
    }

    fn add_io_cycles(&mut self, cycles: usize) {
        for _ in 0..cycles {
            self.cycles.push(Cycle::Internal)
        }
    }
}
