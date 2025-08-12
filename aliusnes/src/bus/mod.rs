use crate::w65c816::addressing::Address;

pub(super) mod dma;
mod math;
pub mod system_bus;
mod wram;

pub trait Bus {
    fn peek_at(&self, addr: Address) -> Option<u8>;
    fn read_and_tick(&mut self, addr: Address) -> u8;
    fn write_and_tick(&mut self, addr: Address, data: u8);
    fn add_io_cycles(&mut self, cycles: usize);
    fn fired_nmi(&mut self) -> bool;
    fn fired_irq(&mut self) -> bool;
}

pub trait Access {
    fn read(&mut self, addr: u16) -> Option<u8>;
    fn write(&mut self, addr: u16, data: u8);
}
