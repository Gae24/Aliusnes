use super::{cpu::Cpu, regsize::RegSize};
use crate::{bus::Bus, utils::int_traits::ManipulateU16};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AddressingMode {
    Implied,
    Immediate,
    Relative,
    RelativeLong,
    Direct,
    DirectX,
    DirectY,
    Indirect,
    IndirectX,
    IndirectY,
    IndirectLong,
    IndirectLongY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    AbsoluteLong,
    AbsoluteLongX,
    AbsoluteIndirect,
    AbsoluteIndirectLong,
    AbsoluteIndirectX,
    AbsoluteJMP,
    AbsoluteLongJSL,
    StackRelative,
    StackRelativeIndirectY,
    StackPEI,
    BlockMove,
}

#[derive(Clone, Copy)]
pub struct Address {
    pub bank: u8,
    pub offset: u16,
}

impl Address {
    pub fn new(bank: u8, offset: u16) -> Self {
        Self { bank, offset }
    }

    pub fn wrapping_add(self, rhs: u32) -> Self {
        (u32::from(self).wrapping_add(rhs)).into()
    }
}

impl From<u16> for Address {
    fn from(value: u16) -> Self {
        Self {
            bank: 0,
            offset: value,
        }
    }
}

impl From<u32> for Address {
    fn from(value: u32) -> Self {
        Self {
            bank: (value >> 16) as u8,
            offset: value as u16,
        }
    }
}

impl From<Address> for u32 {
    fn from(value: Address) -> Self {
        (value.bank as u32) << 16 | value.offset as u32
    }
}

impl Cpu {
    pub fn get_imm<T: RegSize>(&mut self, bus: &mut Bus) -> T {
        let addr = Address::new(self.pbr, self.program_counter);
        if T::IS_U16 {
            let res = self.read_16(bus, addr);
            self.program_counter = self.program_counter.wrapping_add(2);
            T::from_u16(res)
        } else {
            let res = self.read_8(bus, addr);
            self.program_counter = self.program_counter.wrapping_add(1);
            T::from_u8(res)
        }
    }

    /// Based on <http://www.unusedino.de/ec64/technical/aay/c64/addr12a.htm>
    fn add_penalty_cycle<const WRITE: bool>(&mut self, unindexed_page: u8, indexed_page: u8) {
        if WRITE || !self.status.index_regs_size() || unindexed_page != indexed_page {
            self.add_additional_cycles(1);
        }
    }

    fn direct_offset(&mut self, bus: &mut Bus) -> u16 {
        let dpr = self.dpr;
        if dpr as u8 != 0 {
            self.add_additional_cycles(1);
        }
        dpr.wrapping_add(self.get_imm::<u8>(bus) as u16)
    }

    fn indirect_long_address(&mut self, bus: &mut Bus, addr: Address) -> Address {
        Address {
            bank: self.read_8(bus, addr.wrapping_add(2)),
            offset: self.read_16(bus, addr),
        }
    }

    fn absolute_address(&mut self, bus: &mut Bus) -> Address {
        Address::new(self.dbr, self.get_imm(bus))
    }

    fn absolute_long_address(&mut self, bus: &mut Bus) -> Address {
        Address::new(self.get_imm(bus), self.get_imm(bus))
    }

    fn stack_relative_address(&mut self, bus: &mut Bus) -> u16 {
        self.add_additional_cycles(1);
        self.stack_pointer
            .wrapping_add(self.get_imm::<u8>(bus) as u16)
    }

    pub fn decode_addressing_mode<const WRITE: bool>(
        &mut self,
        bus: &mut Bus,
        mode: AddressingMode,
    ) -> Address {
        match mode {
            AddressingMode::Direct => Address::new(0, self.direct_offset(bus)),
            AddressingMode::DirectX => {
                self.add_additional_cycles(1);
                Address::new(0, self.direct_offset(bus).wrapping_add(self.index_x))
            }
            AddressingMode::DirectY => {
                self.add_additional_cycles(1);
                Address::new(0, self.direct_offset(bus).wrapping_add(self.index_y))
            }
            AddressingMode::Indirect => {
                let indirect = self.direct_offset(bus);
                Address::new(self.dbr, self.read_16(bus, Address::from(indirect)))
            }
            AddressingMode::IndirectX => {
                self.add_additional_cycles(1);
                let indirect = self.direct_offset(bus).wrapping_add(self.index_x);
                Address::new(self.dbr, self.read_16(bus, Address::from(indirect)))
            }
            AddressingMode::IndirectY => {
                let indirect = self.direct_offset(bus);
                let unindexed = Address::new(self.dbr, self.read_16(bus, Address::from(indirect)));
                let indexed = unindexed.wrapping_add(self.index_y as u32);
                self.add_penalty_cycle::<WRITE>(
                    unindexed.offset.high_byte(),
                    indexed.offset.high_byte(),
                );
                indexed
            }
            AddressingMode::IndirectLong => {
                let indirect = self.direct_offset(bus);
                self.indirect_long_address(bus, Address::from(indirect))
            }
            AddressingMode::IndirectLongY => {
                let indirect = Address::new(0, self.direct_offset(bus));
                self.indirect_long_address(bus, indirect)
                    .wrapping_add(self.index_y as u32)
            }
            AddressingMode::Absolute => self.absolute_address(bus),
            AddressingMode::AbsoluteX => {
                let unindexed = self.absolute_address(bus);
                let indexed = unindexed.wrapping_add(self.index_x as u32);
                self.add_penalty_cycle::<WRITE>(
                    unindexed.offset.high_byte(),
                    indexed.offset.high_byte(),
                );
                indexed
            }
            AddressingMode::AbsoluteY => {
                let unindexed = self.absolute_address(bus);
                let indexed = unindexed.wrapping_add(self.index_y as u32);
                self.add_penalty_cycle::<WRITE>(
                    unindexed.offset.high_byte(),
                    indexed.offset.high_byte(),
                );
                indexed
            }
            AddressingMode::AbsoluteLong => self.absolute_long_address(bus),
            AddressingMode::AbsoluteLongX => self
                .absolute_long_address(bus)
                .wrapping_add(self.index_x as u32),
            AddressingMode::AbsoluteIndirect => Address::new(0, self.get_imm(bus)),
            AddressingMode::AbsoluteIndirectLong => todo!(),
            AddressingMode::AbsoluteIndirectX => Address::new(
                self.pbr,
                self.get_imm::<u16>(bus).wrapping_add(self.index_x),
            ),
            AddressingMode::StackRelative => Address::new(0, self.stack_relative_address(bus)),
            AddressingMode::StackRelativeIndirectY => {
                self.add_additional_cycles(1);
                let indirect = self.stack_relative_address(bus);
                Address::new(self.dbr, self.read_16(bus, indirect.into()))
                    .wrapping_add(self.index_y as u32)
            }
            AddressingMode::StackPEI => Address::new(0, self.direct_offset(bus)),
            _ => unreachable!(),
        }
    }

    pub fn get_operand<T: RegSize>(&mut self, bus: &mut Bus, mode: &AddressingMode) -> T {
        match mode {
            AddressingMode::Immediate
            | AddressingMode::Implied
            | AddressingMode::Relative
            | AddressingMode::RelativeLong
            | AddressingMode::AbsoluteJMP
            | AddressingMode::AbsoluteLongJSL
            | AddressingMode::AbsoluteIndirectLong
            | AddressingMode::BlockMove => self.get_imm(bus),
            _ => {
                let addr = self.decode_addressing_mode::<false>(bus, *mode);
                if T::IS_U16 {
                    T::from_u16(self.read_16(bus, addr))
                } else {
                    T::from_u8(self.read_8(bus, addr))
                }
            }
        }
    }
}
