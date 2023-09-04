use crate::cart::cart::Cart;

pub struct Bus {
    cart: Cart,
}

impl Bus {
    pub fn new(cart: Cart) -> Self {
        Self { cart }
    }

    pub fn read(&self, addr: u32) -> u8 {
        let bank = (addr >> 16) as u8;
        match bank {
            0x00..=0x3F | 0x80..=0xBF => match (addr >> 8) as u8 {
                0x00..=0x1F => todo!(),
                0x21 => return self.read_mmio(addr as u16),
                0x40..=0x43 => return self.read_mmio(addr as u16),
                _ => {}
            },

            0x7E..=0x7F => todo!(),
            _ => {}
        }
        return self.cart.read(bank, addr as u16);
    }

    pub fn write(&mut self, addr: u32, data: u8) {
        let bank = (addr >> 16) as u8;
        match bank {
            0x00..=0x3F | 0x80..=0xBF => match (addr >> 8) as u8 {
                0x00..=0x1F => todo!(),
                0x21 => return self.write_mmio(addr as u16, data),
                0x40..=0x43 => return self.write_mmio(addr as u16, data),
                _ => {}
            },

            0x7E..=0x7F => todo!(),
            _ => {}
        }
        self.cart.write(bank, addr as u16, data);
    }
}
