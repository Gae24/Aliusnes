use crate::bus::Bus;
use crate::cart::cart::Cart;
use crate::w65c816::cpu::Cpu;

pub struct Emu {
    cpu: Cpu,
    bus: Bus,
}

impl Emu {
    fn new(cart: Cart) -> Self {
        Self {
            bus: Bus::new(cart),
            cpu: Cpu::new(),
        }
    }
}
