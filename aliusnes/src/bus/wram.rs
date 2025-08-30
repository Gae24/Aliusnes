use super::Access;
use crate::{utils::int_traits::ManipulateU16, w65c816::addressing::Address};

pub struct Wram {
    pub ram: [u8; 0x20000],
    wm_addr: Address,
}

impl Wram {
    pub fn new() -> Self {
        Self {
            ram: [0; 0x20000],
            wm_addr: Address::new(0, 0),
        }
    }

    #[inline]
    fn write_to_wm_addr(&mut self, data: u8) {
        self.ram[usize::from(self.wm_addr)] = data;
        let raw_addr = (u32::from(self.wm_addr) + 1) & 0x1_FFFF;
        self.wm_addr = Address::from(raw_addr);
    }

    #[inline]
    fn wm_addl(&mut self, data: u8) {
        self.wm_addr.offset.set_low_byte(data);
    }

    #[inline]
    fn wm_addm(&mut self, data: u8) {
        self.wm_addr.offset.set_high_byte(data);
    }

    #[inline]
    fn wm_addh(&mut self, data: u8) {
        self.wm_addr.bank = data & 1;
    }
}

impl Access for Wram {
    fn read(&mut self, _addr: u16, _: u64) -> Option<u8> {
        let data = self.ram[usize::from(self.wm_addr)];
        let raw_addr = (u32::from(self.wm_addr) + 1) & 0x1_FFFF;
        self.wm_addr = Address::from(raw_addr);
        Some(data)
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x2180 => self.write_to_wm_addr(data),
            0x2181 => self.wm_addl(data),
            0x2182 => self.wm_addm(data),
            0x2183 => self.wm_addh(data),
            _ => unreachable!(),
        }
    }
}
