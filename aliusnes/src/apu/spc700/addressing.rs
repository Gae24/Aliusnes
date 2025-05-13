use crate::{
    apu::spc700::Cpu, bus::Bus, utils::int_traits::ManipulateU16, w65c816::addressing::Address,
};

#[derive(Clone, Copy)]
pub enum AddressingMode {
    Implied,
    Immediate,
    DirectPage,
    Absolute,
    AbsoluteBooleanBit,
    DirectX,
    DirectY,
    DirectXPostIncrement,
    DirectYPostIncrement,
    IndirectX,
    IndirectY,
    XIndirect,
    YIndirect,
}

impl Cpu {
    pub fn get_imm<B: Bus>(&mut self, bus: &mut B) -> u8 {
        let addr = Address::new(self.program_counter, 0);
        self.program_counter = self.program_counter.wrapping_add(1);
        bus.read_and_tick(addr)
    }

    fn direct_page<B: Bus>(&mut self, bus: &mut B) -> u16 {
        let base_addr: u16 = if self.status.direct_page() {
            0x0100
        } else {
            0x0000
        };

        base_addr.wrapping_add(u16::from(self.get_imm(bus)))
    }

    pub fn decode_addressing_mode<B: Bus>(&mut self, bus: &mut B, mode: AddressingMode) -> Address {
        match mode {
            AddressingMode::DirectPage => Address::new(self.direct_page(bus), 0),
            AddressingMode::Absolute => Address::new(
                u16::from_le_bytes([self.get_imm(bus), self.get_imm(bus)]),
                0,
            ),
            AddressingMode::IndirectX => {
                // An extra discarded read is performed in indirect addressing
                let _ = bus.read_and_tick(Address::new(self.program_counter, 0));

                Address::new(
                    u16::from_le_bytes([self.index_x, self.status.direct_page().into()]),
                    0,
                )
            }
            AddressingMode::XIndirect => {
                bus.add_io_cycles(1);
                let mut page = self.direct_page(bus);
                let offset = page.low_byte().wrapping_add(self.index_x);
                page.set_low_byte(offset);

                let high_byte = bus.read_and_tick(Address::new(page, 0));
                page.set_low_byte(offset.wrapping_add(1));
                let low_byte = bus.read_and_tick(Address::new(page, 0));
                Address::new(u16::from_le_bytes([high_byte, low_byte]), 0)
            }
            AddressingMode::DirectX => {
                let page = self.direct_page(bus).wrapping_add(self.index_x.into());
                Address::new(page, 0)
            }
            _ => todo!(),
        }
    }

    pub fn operand<B: Bus>(&mut self, bus: &mut B, mode: AddressingMode) -> u8 {
        match mode {
            AddressingMode::Immediate => self.get_imm(bus),
            AddressingMode::AbsoluteBooleanBit => {
                let addr_bit = u16::from_le_bytes([self.get_imm(bus), self.get_imm(bus)]);
                let val = bus.read_and_tick(Address::new(addr_bit & 0x1FFF, 0));

                val & 1 << (addr_bit >> 13)
            }
            _ => {
                let addr = self.decode_addressing_mode(bus, mode);
                bus.read_and_tick(addr)
            }
        }
    }
}
