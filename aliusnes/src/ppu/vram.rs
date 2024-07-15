use super::tile::BitPlane;
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
    read_latch: u16,
}

impl Vram {
    pub fn new() -> Self {
        Self {
            ram: [0; 0x8000],
            video_port_control: VideoPortControl(0),
            vm_addr: 0x00,
            read_latch: 0x00,
        }
    }

    fn remap_vm_addr(&self) -> usize {
        let addr = self.vm_addr as usize;
        match self.video_port_control.addr_remapping() {
            0 => addr,
            1 => (addr & 0xFF00) | ((addr & 0xE0) >> 5) | ((addr & 0x1F) << 3),
            2 => (addr & 0xFE00) | ((addr & 0x1C0) >> 6) | ((addr & 0x3F) << 3),
            3 => (addr & 0xFC00) | ((addr & 0x380) >> 7) | ((addr & 0x7F) << 3),
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

    pub fn vm_addl(&mut self, data: u8) {
        self.vm_addr.set_low_byte(data);
        self.read_latch = self[self.remap_vm_addr()];
    }

    pub fn vm_addh(&mut self, data: u8) {
        self.vm_addr.set_high_byte(data);
        self.read_latch = self[self.remap_vm_addr()];
    }

    pub fn vm_addl_write(&mut self, data: u8) {
        let addr = self.remap_vm_addr();
        self[addr].set_low_byte(data);
        if !self.video_port_control.increment_on_high_byte_access() {
            self.vm_addr += self.get_increment_amount();
        }
    }

    pub fn vm_addh_write(&mut self, data: u8) {
        let addr = self.remap_vm_addr();
        self[addr].set_high_byte(data);
        if self.video_port_control.increment_on_high_byte_access() {
            self.vm_addr += self.get_increment_amount();
        }
    }

    pub fn vm_addl_read(&mut self) -> u8 {
        let result = self.read_latch.low_byte();
        if !self.video_port_control.increment_on_high_byte_access() {
            self.read_latch = self[self.remap_vm_addr()];
            self.vm_addr += self.get_increment_amount();
        }
        result
    }

    pub fn vm_addh_read(&mut self) -> u8 {
        let result = self.read_latch.high_byte();
        if self.video_port_control.increment_on_high_byte_access() {
            self.read_latch = self[self.remap_vm_addr()];
            self.vm_addr += self.get_increment_amount();
        }
        result
    }

    pub fn planes<T: BitPlane>(&self, addr: usize) -> [u8; T::PLANES] {
        let mut planes = [0; T::PLANES];
        for index in 0..(T::PLANES >> 1) {
            let bitplane = self[addr + (index << 3)];
            planes[index << 1] = bitplane.low_byte();
            planes[(index << 1) + 1] = bitplane.high_byte();
        }
        planes
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
