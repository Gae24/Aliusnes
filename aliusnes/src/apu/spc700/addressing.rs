use crate::{apu::spc700::Cpu, bus::Bus, w65c816::addressing::Address};
use std::marker::ConstParamTy;

#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]
pub enum AddressingMode {
    Implied,
    /// A
    Accumulator,
    /// X
    X,
    /// Y
    Y,
    /// SP
    Sp,
    /// PSW
    Psw,
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

impl AddressingMode {
    pub const fn is_register_access(&self) -> bool {
        matches!(
            self,
            Self::Accumulator | Self::X | Self::Y | Self::Psw | Self::Sp
        )
    }
}

impl Cpu {
    pub fn get_imm<B: Bus>(&mut self, bus: &mut B) -> u8 {
        let addr = Address::new(self.program_counter, 0);
        self.program_counter = self.program_counter.wrapping_add(1);
        bus.read_and_tick(addr)
    }

    pub fn abs<B: Bus>(&mut self, bus: &mut B) -> u16 {
        u16::from_le_bytes([self.get_imm(bus), self.get_imm(bus)])
    }

    pub fn decode_addressing_mode<B: Bus>(&mut self, bus: &mut B, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::DirectPage => {
                u16::from_le_bytes([self.get_imm(bus), self.direct_page()])
            }
            AddressingMode::Absolute => self.abs(bus),
            AddressingMode::IndirectX => {
                // An extra discarded read is performed in indirect addressing
                let _ = bus.read_and_tick(Address::new(self.program_counter, 0));

                u16::from_le_bytes([self.index_x, self.direct_page()])
            }
            AddressingMode::IndirectY => u16::from_le_bytes([self.index_y, self.direct_page()]),
            AddressingMode::XIndirect => {
                bus.add_io_cycles(1);
                let offset = self.get_imm(bus).wrapping_add(self.index_x);
                self.word_from_direct_page(bus, offset)
            }
            AddressingMode::DirectX => {
                bus.add_io_cycles(1);
                let offset = self.get_imm(bus).wrapping_add(self.index_x);
                u16::from_le_bytes([offset, self.direct_page()])
            }
            AddressingMode::AbsoluteX => {
                bus.add_io_cycles(1);
                self.abs(bus).wrapping_add(self.index_x.into())
            }
            AddressingMode::AbsoluteY => {
                bus.add_io_cycles(1);
                self.abs(bus).wrapping_add(self.index_y.into())
            }
            AddressingMode::DirectPageIndirectY => {
                bus.add_io_cycles(1);

                let offset = self.get_imm(bus);
                let page = self.word_from_direct_page(bus, offset);
                page.wrapping_add(self.index_y.into())
            }
            AddressingMode::DirectY => todo!(),
            AddressingMode::DirectXPostIncrement => todo!(),
            AddressingMode::Implied => unreachable!(),
            AddressingMode::Immediate => unreachable!(),
            AddressingMode::AbsoluteBooleanBit => unreachable!(),
            AddressingMode::Accumulator => unreachable!(),
            AddressingMode::X => unreachable!(),
            AddressingMode::Y => unreachable!(),
            AddressingMode::Sp => unreachable!(),
            AddressingMode::Psw => unreachable!(),
        }
    }

    pub fn operand<B: Bus>(&mut self, bus: &mut B, mode: AddressingMode) -> u8 {
        match mode {
            AddressingMode::Accumulator => self.accumulator,
            AddressingMode::X => self.index_x,
            AddressingMode::Y => self.index_y,
            AddressingMode::Sp => self.stack_pointer,
            AddressingMode::Psw => self.status.0,
            AddressingMode::Immediate => self.get_imm(bus),
            AddressingMode::AbsoluteBooleanBit => {
                let addr_bit = u16::from_le_bytes([self.get_imm(bus), self.get_imm(bus)]);
                let val = bus.read_and_tick(Address::new(addr_bit & 0x1FFF, 0));

                val & 1 << (addr_bit >> 13)
            }
            _ => {
                let page = self.decode_addressing_mode(bus, mode);
                bus.read_and_tick(Address::new(page, 0))
            }
        }
    }
}
