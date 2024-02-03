use super::access::Access;

pub struct Wram {
    pub ram: [u8; 0x20000],
    wm_addr: u32,
}

impl Wram {
    pub fn new() -> Self {
        Self {
            ram: [0; 0x20000],
            wm_addr: 0,
        }
    }

    #[inline]
    fn write_to_wm_addr(&mut self, data: u8) {
        self.ram[self.wm_addr as usize] = data;
        self.wm_addr = (self.wm_addr + 1) & 0x1_FFFF;
    }

    #[inline]
    fn wm_addl(&mut self, data: u8) {
        self.wm_addr = ((self.wm_addr & !0xFF) | data as u32) & 0x1_FFFF;
    }

    #[inline]
    fn wm_addm(&mut self, data: u8) {
        self.wm_addr = ((self.wm_addr & !(0xFF << 8)) | data as u32) & 0x1_FFFF;
    }

    #[inline]
    fn wm_addh(&mut self, data: u8) {
        self.wm_addr = ((self.wm_addr & !(0xFF << 16)) | data as u32) & 0x1_FFFF;
    }
}

impl Access for Wram {
    fn read(&mut self, _addr: u16) -> Option<u8> {
        let data = self.ram[self.wm_addr as usize];
        self.wm_addr = (self.wm_addr + 1) & 0x1_FFFF;
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
