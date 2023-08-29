use crate::bus::Bus;
use crate::cart::cart::Cart;
use crate::w65c816::cpu::Cpu;

pub struct Emu {
    cpu: Cpu,
    bus: Bus,
}

impl Emu {
    pub fn new(cart: Cart) -> Self {
        Self {
            bus: Bus::new(cart),
            cpu: Cpu::new(),
        }
    }

    pub fn step(&mut self) {
        let Emu {
            ref mut bus,
            ref mut cpu,
        } = self;
        let ticks = cpu.step(bus);
    }
}
