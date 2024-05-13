use crate::utils::int_traits::ManipulateU16;

bitfield! {
    #[derive(Clone, Copy)]
    pub struct VideoPortControl(pub u8) {
        increment_amount: u8 @ 0..=1,
        addr_remapping: u8 @ 2..=3,
        increment_on_high_byte_access: bool @ 7,
    }
}

pub struct Vram {
    ram: [u16; 0x8000],
    pub video_port_control: VideoPortControl,
    vm_addr: u16,
    latch: u16,
}

impl Vram {
    pub fn new() -> Self {
        Self {
            ram: [0; 0x8000],
            video_port_control: VideoPortControl(0),
            vm_addr: 0x00,
            latch: 0x00,
        }
    }

    fn translate_vm_addr(&self) -> u16 {
        match self.video_port_control.addr_remapping() {
            0 => self.vm_addr,
            1 => {
                (self.vm_addr & 0xFF00)
                    | ((self.vm_addr << 3) & 0x00F8)
                    | ((self.vm_addr >> 5) & 0x07)
            }
            2 => {
                (self.vm_addr & 0xFE00)
                    | ((self.vm_addr << 3) & 0x01F8)
                    | ((self.vm_addr >> 6) & 0x07)
            }
            3 => {
                (self.vm_addr & 0xFC00)
                    | ((self.vm_addr << 3) & 0x03F8)
                    | ((self.vm_addr >> 7) & 0x07)
            }
            _ => unreachable!(),
        }
    }

    fn get_increment_amount(&self) -> u16 {
        match self.video_port_control.increment_amount() {
            0 => 1,
            1 => 32,
            2 | 3 => 128,
            _ => unreachable!(),
        }
    }

    fn update_latch(&mut self) {
        let addr_latch = self.translate_vm_addr();
        self.latch = self[addr_latch as usize];
    }

    pub fn vm_addl(&mut self, data: u8) {
        self.vm_addr.set_low_byte(data);
        self.update_latch();
    }

    pub fn vm_addh(&mut self, data: u8) {
        self.vm_addr.set_high_byte(data);
        self.update_latch();
    }

    pub fn vm_addl_write(&mut self, data: u8) {
        let addr = self.vm_addr as usize;
        self[addr].set_low_byte(data);
        if !self.video_port_control.increment_on_high_byte_access() {
            self.vm_addr += self.get_increment_amount();
        }
    }

    pub fn vm_addh_write(&mut self, data: u8) {
        let addr = self.vm_addr as usize;
        self[addr].set_high_byte(data);
        if self.video_port_control.increment_on_high_byte_access() {
            self.vm_addr += self.get_increment_amount();
        }
    }

    pub fn vm_addl_read(&mut self) -> u8 {
        let result = self.latch.low_byte();
        if !self.video_port_control.increment_on_high_byte_access() {
            self.update_latch();
            self.vm_addr += self.get_increment_amount();
        }
        result
    }

    pub fn vm_addh_read(&mut self) -> u8 {
        let result = self.latch.high_byte();
        if self.video_port_control.increment_on_high_byte_access() {
            self.update_latch();
            self.vm_addr += self.get_increment_amount();
        }
        result
    }
}

impl std::ops::Index<usize> for Vram {
    type Output = u16;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ram[index & 0x7FFF]
    }
}

impl std::ops::IndexMut<usize> for Vram {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.ram[index & 0x7FFF]
    }
}
