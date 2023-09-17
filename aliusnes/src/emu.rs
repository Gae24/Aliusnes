use crate::bus::Bus;
use crate::cart::Cart;
use crate::w65c816::cpu::Cpu;

pub struct Emu {
    cpu: Cpu,
    bus: Bus,
}

impl Emu {
    pub fn new(cart: Cart) -> Self {
        let mut emu = Emu {
            bus: Bus::new(cart),
            cpu: Cpu::new(),
        };
        emu.reset();
        emu
    }

    pub fn reset(&mut self) {
        self.cpu.reset(&mut self.bus);
    }

    pub fn step(&mut self) {
        let Emu {
            ref mut bus,
            ref mut cpu,
        } = self;
        let _ticks = cpu.step(bus);
    }
}
