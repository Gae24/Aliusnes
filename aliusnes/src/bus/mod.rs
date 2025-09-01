pub(super) mod dma;
mod math;
pub mod system_bus;
mod wram;

#[derive(Clone, Copy)]
pub(crate) struct Address {
    pub bank: u8,
    pub offset: u16,
}

impl Address {
    pub fn new(offset: u16, bank: u8) -> Self {
        Self { bank, offset }
    }

    pub fn wrapping_offset_add(&self, rhs: u16) -> Self {
        Self {
            bank: self.bank,
            offset: self.offset.wrapping_add(rhs),
        }
    }

    pub fn wrapping_add(&self, rhs: u32) -> Self {
        (u32::from(*self).wrapping_add(rhs)).into()
    }
}

impl From<u16> for Address {
    fn from(value: u16) -> Self {
        Self {
            bank: 0,
            offset: value,
        }
    }
}

impl From<u32> for Address {
    fn from(value: u32) -> Self {
        Self {
            bank: (value >> 16) as u8,
            offset: value as u16,
        }
    }
}

impl From<Address> for u32 {
    fn from(value: Address) -> Self {
        (u32::from(value.bank) << 16) | u32::from(value.offset)
    }
}

impl From<Address> for usize {
    fn from(value: Address) -> Self {
        ((value.bank as usize) << 16) | value.offset as usize
    }
}

pub(crate) trait Bus {
    fn peek_at(&self, addr: Address) -> Option<u8>;
    fn read_and_tick(&mut self, addr: Address) -> u8;
    fn write_and_tick(&mut self, addr: Address, data: u8);
    fn add_io_cycles(&mut self, cycles: usize);
    fn fired_nmi(&mut self) -> bool;
    fn fired_irq(&mut self) -> bool;
}

pub(crate) trait Access {
    fn read(&mut self, addr: u16, time: u64) -> Option<u8>;
    fn write(&mut self, addr: u16, data: u8, time: u64);
}
