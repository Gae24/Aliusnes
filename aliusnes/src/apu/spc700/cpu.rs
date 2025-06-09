use crate::apu::spc700::addressing::AddressingMode;
use crate::utils::int_traits::ManipulateU16;
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

    pub fn direct_page(&self) -> u8 {
        self.status.direct_page().into()
    }

    pub fn ya(&self) -> u16 {
        u16::from_le_bytes([self.accumulator, self.index_y])
    }

    pub fn set_ya(&mut self, value: u16) {
        self.accumulator = value.low_byte();
        self.index_y = value.high_byte();
    }

    pub fn set_nz(&mut self, value: u8) {
        self.status.set_negative(value >> 7 != 0);
        self.status.set_zero(value == 0);
    }

    pub fn read_16<B: Bus>(&mut self, bus: &mut B, addr: u16) -> u16 {
        let addr = Address::new(addr, 0);
        u16::from_le_bytes([
            bus.read_and_tick(addr),
            bus.read_and_tick(addr.wrapping_add(1)),
        ])
    }

    pub fn write<B: Bus>(&mut self, bus: &mut B, mode: AddressingMode, data: u8) {
        match mode {
            AddressingMode::Accumulator => self.accumulator = data,
            AddressingMode::X => self.index_x = data,
            AddressingMode::Y => self.index_y = data,
            AddressingMode::Sp => self.stack_pointer = data,
            AddressingMode::Psw => self.status = Status(data),
            _ => {
                let page = self.decode_addressing_mode(bus, mode);
                bus.write_and_tick(Address::new(page, 0), data);
            }
        }
    }

    pub fn word_from_direct_page<B: Bus>(&mut self, bus: &mut B, offset: u8) -> u16 {
        let low_byte_addr = u16::from_le_bytes([offset, self.direct_page()]);
        let high_byte_addr = u16::from_le_bytes([offset.wrapping_add(1), self.direct_page()]);

        u16::from_le_bytes([
            bus.read_and_tick(Address::new(low_byte_addr, 0)),
            bus.read_and_tick(Address::new(high_byte_addr, 0)),
        ])
    }

    pub fn do_adc(&mut self, a: u8, b: u8) -> u8 {
        let result = u16::from(a) + u16::from(b) + u16::from(self.status.carry());
        self.status.set_carry(result >> 8 != 0);

        let result = result as u8;
        self.status.set_half_carry((a ^ b ^ result) & 0x10 != 0);
        self.status
            .set_overflow(!(a ^ b) & (a ^ result) & 0x80 != 0);
        self.set_nz(result);

        result
    }

    pub fn do_test_bit<B: Bus>(&mut self, bus: &mut B, mode: AddressingMode, clear: bool) {
        let addr = Address::new(self.decode_addressing_mode(bus, mode), 0);
        let operand = bus.read_and_tick(addr);
        self.set_nz(self.accumulator.wrapping_sub(operand));

        let value = if clear {
            !self.accumulator & operand
        } else {
            self.accumulator | operand
        };

        // Dummy read
        let _ = bus.read_and_tick(addr);
        bus.write_and_tick(addr, value);
    }

    pub fn do_branch<B: Bus>(&mut self, bus: &mut B, cond: bool) {
        let offset = self.get_imm(bus) as i8;
        if cond {
            bus.add_io_cycles(2);
            self.program_counter = self.program_counter.wrapping_add(offset as u16);
        }
    }

    pub fn do_pop<B: Bus>(&mut self, bus: &mut B) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let stack_addr = u16::from_le_bytes([self.stack_pointer, 0x01]);
        bus.read_and_tick(Address::new(stack_addr, 0))
    }

    pub fn do_push<B: Bus>(&mut self, bus: &mut B, data: u8) {
        let stack_addr = u16::from_le_bytes([self.stack_pointer, 0x01]);
        bus.write_and_tick(Address::new(stack_addr, 0), data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    pub fn do_rmw<B: Bus, const VOID_READ: bool>(
        &mut self,
        bus: &mut B,
        mode: AddressingMode,
        f: impl FnOnce(&mut Cpu, u8) -> u8,
    ) {
        if mode.is_register_access() && VOID_READ {
            let _ = bus.read_and_tick(Address::new(self.program_counter, 0));
        }
        match mode {
            AddressingMode::Accumulator => self.accumulator = f(self, self.accumulator),
            AddressingMode::X => self.index_x = f(self, self.index_x),
            AddressingMode::Y => self.index_y = f(self, self.index_y),
            AddressingMode::AbsoluteBooleanBit => {
                let addr_bit = u16::from_le_bytes([self.get_imm(bus), self.get_imm(bus)]);
                let addr = Address::new(addr_bit & 0x1FFF, 0);
                let bit = addr_bit >> 13;
                let data = bus.read_and_tick(addr);
                let result = (data & !(1 << bit)) | f(self, data) << bit;
                bus.write_and_tick(addr, result);
            }
            _ => {
                let addr = Address::new(self.decode_addressing_mode(bus, mode), 0);
                let data = bus.read_and_tick(addr);
                let result = f(self, data);
                bus.write_and_tick(addr, result);
            }
        }
    }

    pub fn do_rmw_word<B: Bus>(&mut self, bus: &mut B, f: impl FnOnce(&mut Cpu, u16) -> u16) {
        let offset = self.get_imm(bus);
        let low_byte_page = u16::from_le_bytes([offset, self.direct_page()]);
        let high_byte_page = u16::from_le_bytes([offset.wrapping_add(1), self.direct_page()]);

        let low_byte_addr = Address::new(low_byte_page, 0);
        let high_byte_addr = Address::new(high_byte_page, 0);

        let low_byte = bus.read_and_tick(low_byte_addr);
        let high_byte = bus.read_and_tick(high_byte_addr);

        let data = u16::from_le_bytes([low_byte, high_byte]);

        let result = f(self, data);
        bus.write_and_tick(low_byte_addr, result.low_byte());
        bus.write_and_tick(high_byte_addr, result.high_byte());
    }
}
