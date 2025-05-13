use crate::apu::spc700::addressing::AddressingMode;
use crate::{bus::Bus, w65c816::addressing::Address};

bitfield!(
    pub struct Status(pub u8) {
        pub carry: bool @ 0,
        pub zero: bool @ 1,
        pub irq_enabled: bool @ 2,
        pub half_carry: bool @ 3,
        pub break_: bool @ 4,
        pub direct_page: bool @ 5,
        pub overflow: bool @ 6,
        pub negative: bool @ 7,
    }
);

pub struct Cpu {
    pub accumulator: u8,
    pub index_x: u8,
    pub index_y: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub status: Status,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            accumulator: 0x00,
            index_x: 0x00,
            index_y: 0x00,
            program_counter: 0x00,
            stack_pointer: 0x00,
            status: Status(0),
        }
    }

    pub fn read_16<B: Bus>(&mut self, bus: &mut B, addr: u16) -> u16 {
        let addr = Address::new(addr, 0);
        u16::from_le_bytes([
            bus.read_and_tick(addr),
            bus.read_and_tick(addr.wrapping_add(1)),
        ])
    }

    pub fn do_push<B: Bus>(&mut self, bus: &mut B, data: u8) {
        let stack_addr = u16::from_le_bytes([self.stack_pointer, 0x01]);
        bus.write_and_tick(Address::new(stack_addr, 0), data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    pub fn do_rmw<B: Bus>(
        &mut self,
        bus: &mut B,
        mode: &AddressingMode,
        f: fn(&mut Cpu, u8) -> u8,
    ) {
        let addr = self.decode_addressing_mode(bus, *mode);
        let data = bus.read_and_tick(addr);
        let result = f(self, data);
        bus.write_and_tick(addr, result);
    }
}
