use crate::{apu::spc700::Cpu, bus::Bus, w65c816::addressing::Address};
use std::marker::ConstParamTy;

#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]
pub enum AddressingMode {
    Implied,
    /// #imm
    Immediate,
    /// dp
    DirectPage,
    /// !abs
    Absolute,
    /// !abs+X
    AbsoluteX,
    /// !abs+Y
    AbsoluteY,
    /// mem.bit
    AbsoluteBooleanBit,
    /// dp+X
    DirectX,
    /// dp+Y
    DirectY,
    DirectXPostIncrement,
    /// (X)
    IndirectX,
    /// (Y)
    IndirectY,
    /// [dp]+Y
    DirectPageIndirectY,
    /// [dp+X]
    XIndirect,
}

#[derive(ConstParamTy, PartialEq, Eq)]
pub enum Source {
    A,
    X,
    Y,
    PSW,
}

impl Cpu {
    pub fn get_imm<B: Bus>(&mut self, bus: &mut B) -> u8 {
        let addr = Address::new(self.program_counter, 0);
        self.program_counter = self.program_counter.wrapping_add(1);
        bus.read_and_tick(addr)
    }

    pub fn decode_addressing_mode<B: Bus>(&mut self, bus: &mut B, mode: AddressingMode) -> Address {
        match mode {
            AddressingMode::DirectPage => Address::new(
                u16::from_le_bytes([self.get_imm(bus), self.direct_page()]),
                0,
            ),
            AddressingMode::Absolute => Address::new(
                u16::from_le_bytes([self.get_imm(bus), self.get_imm(bus)]),
                0,
            ),
            AddressingMode::IndirectX => {
                // An extra discarded read is performed in indirect addressing
                let _ = bus.read_and_tick(Address::new(self.program_counter, 0));

                Address::new(u16::from_le_bytes([self.index_x, self.direct_page()]), 0)
            }
            AddressingMode::IndirectY => {
                Address::new(u16::from_le_bytes([self.index_y, self.direct_page()]), 0)
            }
            AddressingMode::XIndirect => {
                bus.add_io_cycles(1);
                let offset = self.get_imm(bus).wrapping_add(self.index_x);
                let page = self.word_from_direct_page(bus, offset);
                Address::new(page, 0)
            }
            AddressingMode::DirectX => {
                bus.add_io_cycles(1);
                let offset = self.get_imm(bus).wrapping_add(self.index_x);
                let page = u16::from_le_bytes([offset, self.direct_page()]);

                Address::new(page, 0)
            }
            AddressingMode::AbsoluteX => {
                let absolute = u16::from_le_bytes([self.get_imm(bus), self.get_imm(bus)]);
                bus.add_io_cycles(1);
                Address::new(absolute.wrapping_add(self.index_x.into()), 0)
            }
            AddressingMode::AbsoluteY => {
                let absolute = u16::from_le_bytes([self.get_imm(bus), self.get_imm(bus)]);
                bus.add_io_cycles(1);
                Address::new(absolute.wrapping_add(self.index_y.into()), 0)
            }
            AddressingMode::DirectPageIndirectY => {
                bus.add_io_cycles(1);

                let offset = self.get_imm(bus);
                let page = self.word_from_direct_page(bus, offset);
                Address::new(page.wrapping_add(self.index_y.into()), 0)
            }
            AddressingMode::Implied => todo!(),
            AddressingMode::Immediate => todo!(),
            AddressingMode::AbsoluteBooleanBit => todo!(),
            AddressingMode::DirectY => todo!(),
            AddressingMode::DirectXPostIncrement => todo!(),
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
