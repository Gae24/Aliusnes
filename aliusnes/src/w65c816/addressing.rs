use super::{cpu::Cpu, regsize::RegSize};
use crate::{bus::Bus, utils::int_traits::ManipulateU16};

#[derive(Clone, Copy, Debug)]
pub enum AddressingMode {
    Accumulator,
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
    StackRelative,
    StackRelIndirectY,
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
        (u32::from(value.bank) << 16) | u32::from(value.offset)
    }
}

impl From<Address> for usize {
    fn from(value: Address) -> Self {
        ((value.bank as usize) << 16) | value.offset as usize
    }
}

impl Cpu {
    pub fn read_bank0<B: Bus>(&mut self, bus: &mut B, page: u16) -> u16 {
        u16::from_le_bytes([
            bus.read_and_tick(page.into()),
            bus.read_and_tick(page.wrapping_add(1).into()),
        ])
    }

    pub fn write_bank0<B: Bus>(&mut self, bus: &mut B, page: u16, data: u16) {
        bus.write_and_tick(page.into(), data.low_byte());
        bus.write_and_tick(page.wrapping_add(1).into(), data.high_byte());
    }

    pub fn get_imm<T: RegSize, B: Bus>(&mut self, bus: &mut B) -> T {
        let addr = Address::new(self.program_counter, self.pbr);
        if T::IS_U16 {
            let res = u16::from_le_bytes([
                bus.read_and_tick(addr),
                bus.read_and_tick(addr.wrapping_offset_add(1)),
            ]);
            self.program_counter = self.program_counter.wrapping_add(2);
            T::from_u16(res)
        } else {
            let res = bus.read_and_tick(addr);
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

    fn direct_offset<B: Bus>(&mut self, bus: &mut B) -> u16 {
        let dpr = self.dpr;
        if dpr.low_byte() != 0 {
            bus.add_io_cycles(1);
        }
        dpr.wrapping_add(u16::from(self.get_imm::<u8, B>(bus)))
    }

    fn direct_page_indexed<B: Bus>(&mut self, bus: &mut B, index: u16) -> u16 {
        bus.add_io_cycles(1);
        self.direct_offset(bus).wrapping_add(index)
    }

    fn indirect_long_address<B: Bus>(&mut self, bus: &mut B, indirect: u16) -> Address {
        let addr = Address::new(indirect.wrapping_add(2), 0);
        Address::new(self.read_bank0(bus, indirect), bus.read_and_tick(addr))
    }

    fn absolute_address<B: Bus>(&mut self, bus: &mut B) -> Address {
        Address::new(self.get_imm(bus), self.dbr)
    }

    fn absolute_long_address<B: Bus>(&mut self, bus: &mut B) -> Address {
        Address::new(self.get_imm(bus), self.get_imm(bus))
    }

    fn stack_relative_address<B: Bus>(&mut self, bus: &mut B) -> u16 {
        bus.add_io_cycles(1);
        self.stack_pointer
            .wrapping_add(u16::from(self.get_imm::<u8, B>(bus)))
    }

    pub fn decode_addressing_mode<const WRITE: bool, B: Bus>(
        &mut self,
        bus: &mut B,
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
                let indexed = unindexed.wrapping_add(u32::from(self.index_y));
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
                    .wrapping_add(u32::from(self.index_y))
            }
            AddressingMode::Absolute => self.absolute_address(bus),
            AddressingMode::AbsoluteX => {
                let unindexed = self.absolute_address(bus);
                let indexed = unindexed.wrapping_add(u32::from(self.index_x));
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
                let indexed = unindexed.wrapping_add(u32::from(self.index_y));
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
                .wrapping_add(u32::from(self.index_x)),
            AddressingMode::AbsoluteIndirect => {
                let indirect = self.get_imm(bus);
                Address::new(self.read_bank0(bus, indirect), 0)
            }
            AddressingMode::AbsoluteIndirectX => {
                bus.add_io_cycles(1);
                Address::new(
                    self.get_imm::<u16, B>(bus).wrapping_add(self.index_x),
                    self.pbr,
                )
            }
            AddressingMode::AbsoluteIndirectLong => {
                let indirect = self.get_imm(bus);
                self.indirect_long_address(bus, indirect)
            }
            AddressingMode::StackRelIndirectY => {
                bus.add_io_cycles(1);
                let indirect = self.stack_relative_address(bus);
                let offset = self.read_bank0(bus, indirect);
                Address::new(offset, self.dbr).wrapping_add(u32::from(self.index_y))
            }
            _ => unreachable!(),
        }
    }

    pub fn get_operand<T: RegSize, B: Bus>(&mut self, bus: &mut B, mode: &AddressingMode) -> T {
        match mode {
            AddressingMode::RelativeLong => {
                bus.add_io_cycles(1);
                let offset = self.get_imm::<u16, _>(bus) as i16;
                T::from_u16(self.program_counter.wrapping_add(offset as u16))
            }
            AddressingMode::Immediate | AddressingMode::Relative | AddressingMode::BlockMove => {
                self.get_imm(bus)
            }
            AddressingMode::Direct
            | AddressingMode::DirectX
            | AddressingMode::DirectY
            | AddressingMode::StackRelative => self.read_from_direct_page(bus, mode).0,
            _ => {
                let addr = self.decode_addressing_mode::<false, B>(bus, *mode);
                self.read(bus, addr)
            }
        }
    }

    pub fn direct_page_address<B: Bus>(&mut self, bus: &mut B, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::DirectX => self.direct_page_indexed(bus, self.index_x),
            AddressingMode::DirectY => self.direct_page_indexed(bus, self.index_y),
            AddressingMode::StackRelative => self.stack_relative_address(bus),
            _ => self.direct_offset(bus),
        }
    }

    pub fn read_from_direct_page<T: RegSize, B: Bus>(
        &mut self,
        bus: &mut B,
        mode: &AddressingMode,
    ) -> (T, u16) {
        let page = self.direct_page_address(bus, mode);
        if T::IS_U16 {
            (T::from_u16(self.read_bank0(bus, page)), page)
        } else {
            (T::from_u8(bus.read_and_tick(page.into())), page)
        }
    }
}
