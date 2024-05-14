use crate::utils::int_traits::ManipulateU16;

bitfield! {
    #[derive(Clone, Copy)]
    pub(super) struct Objsel(pub u8) {
        name_base_addr: u8 @ 0..=2,
        name_select: u8 @ 3..=4,
        object_size: u8 @ 5..=7,
    }
}

pub(super) struct Oam {
    ram: [u8; 0x220],
    pub(super) objsel: Objsel,
    oa_addr: u16,
    internal_addr: u16,
    latch: u8,
}

impl Oam {
    pub fn new() -> Self {
        Self {
            ram: [0; 0x220],
            objsel: Objsel(0),
            oa_addr: 0x0000,
            internal_addr: 0x0000,
            latch: 0x00,
        }
    }

    pub fn oa_addl(&mut self, data: u8) {
        self.oa_addr.set_low_byte(data);
        self.internal_addr = (self.oa_addr & 0x1FF) << 1;
    }

    pub fn oa_addh(&mut self, data: u8) {
        self.oa_addr.set_high_byte(data);
        self.internal_addr = (self.oa_addr & 0x1FF) << 1;
    }

    pub fn oa_addr_write(&mut self, data: u8) {
        if (self.internal_addr & 1) == 0 {
            self.latch = data;
        }
        if self.internal_addr < 0x200 && (self.internal_addr & 1) == 1 {
            self.ram[(self.internal_addr - 1) as usize] = self.latch;
            self.ram[self.internal_addr as usize] = data;
        }
        if self.internal_addr >= 0x200 {
            self.ram[self.internal_addr as usize] = data;
        }
        self.internal_addr += 1;
    }

    pub fn oa_addr_read(&mut self) -> u8 {
        let result = self.ram[self.internal_addr as usize];
        self.internal_addr += 1;
        result
    }
}
