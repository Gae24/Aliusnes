use std::usize;

use crate::utils::int_traits::ManipulateU16;

pub(super) struct Cgram {
    ram: [u16; 0x100],
    cg_addr: u8,
    latch: Option<u8>,
}

impl Cgram {
    pub fn new() -> Self {
        Self {
            ram: [0; 0x100],
            cg_addr: 0x00,
            latch: None,
        }
    }

    pub fn cg_addr(&mut self, data: u8) {
        self.cg_addr = data;
        self.latch = None;
    }

    pub fn cg_addr_write(&mut self, data: u8) {
        match self.latch {
            None => self.latch = Some(data),
            Some(byte) => {
                self.ram[self.cg_addr as usize] = byte as u16 | (data as u16) << 8;
                self.cg_addr = self.cg_addr.wrapping_add(1);
                self.latch = None;
            }
        }
    }

    fn cg_addr_read(&mut self) -> u8 {
        match self.latch {
            Some(high_byte) => {
                self.cg_addr = self.cg_addr.wrapping_add(1);
                self.latch = None;
                high_byte
            }
            None => {
                let val = self.ram[self.cg_addr as usize];
                self.latch = Some(val.high_byte());
                val.low_byte()
            }
        }
    }
}
