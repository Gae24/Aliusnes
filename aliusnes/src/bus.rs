use crate::cart::cart::Cart;

pub struct Bus {
    cart: Cart,
}

impl Bus {
    pub fn new(cart: Cart) -> Self {
        Self { cart }
    }

    pub fn read(&self, addr: u32) -> u8 {
        4
    }

    pub fn write(&self, addr: u32, data: u8) {}
}
