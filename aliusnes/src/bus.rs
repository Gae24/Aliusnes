use crate::cart::cart::Cart;

pub struct Bus {
    cart: Cart,
}

impl Bus {
    pub fn new(cart: Cart) -> Self {
        Self { cart }
    }

    pub fn read_16bit(&self, addr: u16) -> u16 {
        (self.read(addr) as u16) | ((self.read(addr + 1) as u16) << 8)
    }

    pub fn read(&self, addr: u16) -> u8 {
        4
    }

    pub fn write_16bit(&self, addr: u16, data: u16) {
        self.write(addr, data as u8);
        self.write(addr + 1, (data >> 8) as u8);
    }

    pub fn write(&self, addr: u16, data: u8) {}
}
