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
    pub fn new(offset: u16, bank: u8) -> Self {
        Self { bank, offset }
    }

    pub fn wrapping_offset_add(&self, rhs: u16) -> Self {
        Self {
            bank: self.bank,
            offset: self.offset.wrapping_add(rhs),
        }
    }

    pub fn wrapping_add(&self, rhs: u32) -> Self {
        (u32::from(*self).wrapping_add(rhs)).into()
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

impl From<Address> for usize {
    fn from(value: Address) -> Self {
        (value.bank as usize) << 16 | value.offset as usize
    }
}

impl Cpu {
    pub fn read_bank0(&mut self, bus: &mut Bus, offset: u16) -> u16 {
        let addr = Address::new(offset, 0);
        self.read_8(bus, addr) as u16 | (self.read_8(bus, addr.wrapping_offset_add(1)) as u16) << 8
    }

    pub fn write_bank0(&mut self, bus: &mut Bus, offset: u16, data: u16) {
        let addr = Address::new(offset, 0);
        self.write_8(bus, addr, data.low_byte());
        self.write_8(bus, addr.wrapping_offset_add(1), data.high_byte());
    }

    pub fn get_imm<T: RegSize>(&mut self, bus: &mut Bus) -> T {
        let addr = Address::new(self.program_counter, self.pbr);
        if T::IS_U16 {
            let res = self.read_8(bus, addr) as u16
                | (self.read_8(bus, addr.wrapping_offset_add(1)) as u16) << 8;
            self.program_counter = self.program_counter.wrapping_add(2);
            T::from_u16(res)
        } else {
            let res = self.read_8(bus, addr);
            self.program_counter = self.program_counter.wrapping_add(1);
            T::from_u8(res)
        }
    }

    /// Based on <http://www.unusedino.de/ec64/technical/aay/c64/addr12a.htm>
    fn detect_penalty_cycle<const WRITE: bool>(
        &mut self,
        unindexed_page: u8,
        indexed_page: u8,
    ) -> bool {
        WRITE || !self.status.index_regs_size() || unindexed_page != indexed_page
    }

    fn direct_offset(&mut self, bus: &mut Bus) -> u16 {
        let dpr = self.dpr;
        if dpr as u8 != 0 {
            bus.add_io_cycles(1);
        }
        dpr.wrapping_add(self.get_imm::<u8>(bus) as u16)
    }

    fn direct_page_indexed(&mut self, bus: &mut Bus, index: u16) -> u16 {
        bus.add_io_cycles(1);
        self.direct_offset(bus).wrapping_add(index)
    }

    fn indirect_long_address(&mut self, bus: &mut Bus, indirect: u16) -> Address {
        let addr = Address::new(indirect.wrapping_add(2), 0);
        Address::new(self.read_bank0(bus, indirect), self.read_8(bus, addr))
    }

    fn absolute_address(&mut self, bus: &mut Bus) -> Address {
        Address::new(self.get_imm(bus), self.dbr)
    }

    fn absolute_long_address(&mut self, bus: &mut Bus) -> Address {
        Address::new(self.get_imm(bus), self.get_imm(bus))
    }

    fn stack_relative_address(&mut self, bus: &mut Bus) -> u16 {
        bus.add_io_cycles(1);
        self.stack_pointer
            .wrapping_add(self.get_imm::<u8>(bus) as u16)
    }

    pub fn decode_addressing_mode<const WRITE: bool>(
        &mut self,
        bus: &mut Bus,
        mode: AddressingMode,
    ) -> Address {
        match mode {
            AddressingMode::Indirect => {
                let indirect = self.direct_offset(bus);
                let offset = self.read_bank0(bus, indirect);
                Address::new(offset, self.dbr)
            }
            AddressingMode::IndirectX => {
                bus.add_io_cycles(1);
                let indirect = self.direct_offset(bus).wrapping_add(self.index_x);
                let offset = self.read_bank0(bus, indirect);
                Address::new(offset, self.dbr)
            }
            AddressingMode::IndirectY => {
                let indirect = self.direct_offset(bus);
                let offset = self.read_bank0(bus, indirect);
                let unindexed = Address::new(offset, self.dbr);
                let indexed = unindexed.wrapping_add(self.index_y as u32);
                if self.detect_penalty_cycle::<WRITE>(
                    unindexed.offset.high_byte(),
                    indexed.offset.high_byte(),
                ) {
                    bus.add_io_cycles(1);
                }
                indexed
            }
            AddressingMode::IndirectLong => {
                let indirect = self.direct_offset(bus);
                self.indirect_long_address(bus, indirect)
            }
            AddressingMode::IndirectLongY => {
                let indirect = self.direct_offset(bus);
                self.indirect_long_address(bus, indirect)
                    .wrapping_add(self.index_y as u32)
            }
            AddressingMode::Absolute => self.absolute_address(bus),
            AddressingMode::AbsoluteX => {
                let unindexed = self.absolute_address(bus);
                let indexed = unindexed.wrapping_add(self.index_x as u32);
                if self.detect_penalty_cycle::<WRITE>(
                    unindexed.offset.high_byte(),
                    indexed.offset.high_byte(),
                ) {
                    bus.add_io_cycles(1);
                }
                indexed
            }
            AddressingMode::AbsoluteY => {
                let unindexed = self.absolute_address(bus);
                let indexed = unindexed.wrapping_add(self.index_y as u32);
                if self.detect_penalty_cycle::<WRITE>(
                    unindexed.offset.high_byte(),
                    indexed.offset.high_byte(),
                ) {
                    bus.add_io_cycles(1);
                }
                indexed
            }
            AddressingMode::AbsoluteLong => self.absolute_long_address(bus),
            AddressingMode::AbsoluteLongX => self
                .absolute_long_address(bus)
                .wrapping_add(self.index_x as u32),
            AddressingMode::AbsoluteIndirect => Address::new(self.get_imm(bus), 0),
            AddressingMode::AbsoluteIndirectX => {
                bus.add_io_cycles(1);
                Address::new(
                    self.get_imm::<u16>(bus).wrapping_add(self.index_x),
                    self.pbr,
                )
            }
            AddressingMode::StackRelativeIndirectY => {
                bus.add_io_cycles(1);
                let indirect = self.stack_relative_address(bus);
                let offset = self.read_bank0(bus, indirect);
                Address::new(offset, self.dbr).wrapping_add(self.index_y as u32)
            }
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
            AddressingMode::Direct
            | AddressingMode::DirectX
            | AddressingMode::DirectY
            | AddressingMode::StackRelative
            | AddressingMode::StackPEI => self.read_from_direct_page(bus, mode).0,
            _ => {
                let addr = self.decode_addressing_mode::<false>(bus, *mode);
                match T::IS_U16 {
                    true => T::from_u16(self.read_16(bus, addr)),
                    false => T::from_u8(self.read_8(bus, addr)),
                }
            }
        }
    }

    pub fn read_from_direct_page<T: RegSize>(
        &mut self,
        bus: &mut Bus,
        mode: &AddressingMode,
    ) -> (T, u16) {
        let page = match mode {
            AddressingMode::DirectX => self.direct_page_indexed(bus, self.index_x),
            AddressingMode::DirectY => self.direct_page_indexed(bus, self.index_y),
            AddressingMode::StackRelative => self.stack_relative_address(bus),
            _ => self.direct_offset(bus),
        };
        match T::IS_U16 {
            true => (T::from_u16(self.read_bank0(bus, page)), page),
            false => (T::from_u8(self.read_8(bus, page.into())), page),
        }
    }
}
