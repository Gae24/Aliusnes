// use super::opcodes::OPCODES_MAP;
use crate::bus::Bus;

use super::regsize::RegSize;

bitflags! {
    pub struct CpuFlags: u8 {
        const NEGATIVE = 0b10000000;
        const OVERFLOW = 0b01000000;
        const A_REG_SIZE = 0b00100000;
        const INDEX_REGS_SIZE = 0b00010000;
        const DECIMAL = 0b00001000;
        const IRQ_DISABLE = 0b00000100;
        const ZERO = 0b00000010;
        const CARRY = 0b00000001;
    }
}

pub struct Cpu {
    pub accumulator: u16,
    pub index_x: u16,
    pub index_y: u16,
    pub stack_pointer: u16,
    pub program_couter: u16,
    pub status_register: CpuFlags,
    pub dpr: u16,
    pub pbr: u8,
    pub dbr: u8,
}

#[derive(Debug)]
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
    StackRelative,
    StackRelativeIndirectY,
    BlockMove,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            accumulator: 0x00,
            index_x: 0x00,
            index_y: 0x00,
            stack_pointer: 0x00,
            status_register: CpuFlags::from_bits_truncate(0b10010000),
            dpr: 0x00,
            pbr: 0x00,
            dbr: 0x00,
            program_couter: 0x00,
        }
    }

    pub fn set_low_a(&mut self, val: u16) {
        self.accumulator = (self.accumulator & 0xFF00) | val;
    }

    fn step(&mut self, bus: &mut Bus) -> usize {
        let op = self.pbr;

        // let opcode = OPCODES_MAP
        //     .get(&op)
        //     .expect(&format!("OpCode {:x} is not recognized", op));
        // let instr = opcode.function;

        // let data = bus.read(self.get_operand_address(&opcode.mode));

        // instr(self, bus, &opcode.mode);
        // get base cycle and add extras
        1
    }

    pub fn read_8(bus: &Bus, addr: u32) -> u8 {
        bus.read(addr)
    }

    pub fn write_8(bus: &Bus, addr: u32, data: u8) {
        bus.write(addr, data);
    }

    pub fn read_16(bus: &Bus, addr: u32) -> u16 {
        Self::read_8(bus, addr) as u16 | (Self::read_8(bus, addr + 1) as u16) << 8
    }

    pub fn write_16(bus: &Bus, addr: u32, data: u16) {
        Self::write_8(bus, addr, data as u8);
        Self::write_8(bus, addr.wrapping_add(1), (data >> 8) as u8);
    }

    pub fn get_direct_addr(&self, bus: &Bus) -> u16 {
        self.dpr | bus.read(self.program_couter.into()) as u16
    }

    pub fn get_indirect_addr(&self, bus: &Bus, addr: u16) -> u32 {
        (Self::read_16(bus, addr.into()) | self.dbr as u16).into()
    }

    pub fn get_indirect_long_addr(bus: &Bus, addr: u32) -> u32 {
        ((Self::read_16(bus, addr) | Self::read_8(bus, addr.wrapping_add(2)) as u16) as u32) << 16
    }

    pub fn get_absolute_addr(&self, bus: &Bus) -> u32 {
        (self.dbr | bus.read(self.program_couter.into())).into()
    }

    pub fn get_absolute_long_addr(&self, bus: &Bus) -> u32 {
        let addr1 = Self::read_16(bus, self.program_couter.into());
        let addr2 = Self::read_8(bus, (self.program_couter + 2).into()) as u16;
        (addr1 | addr2).into()
    }

    pub fn get_stack_relative_addr(&self, bus: &Bus) -> u16 {
        self.stack_pointer
            .wrapping_add(Self::read_8(bus, self.program_couter.into()).into())
    }

    fn get_address(&mut self, bus: &Bus, mode: &AddressingMode) -> u32 {
        match mode {
            AddressingMode::Implied => unreachable!(),
            AddressingMode::Immediate => todo!(),
            AddressingMode::Relative => {
                let addr = Self::read_8(bus, self.program_couter.into());
                self.program_couter.wrapping_add(addr.into()).into()
            }
            AddressingMode::RelativeLong => {
                let addr = Self::read_16(bus, self.program_couter.into());
                self.program_couter.wrapping_add(addr).into()
            }
            AddressingMode::Direct => self.get_direct_addr(bus) as u32,
            AddressingMode::DirectX => (self.get_direct_addr(bus) + self.index_x) as u32,
            AddressingMode::DirectY => (self.get_direct_addr(bus) + self.index_y) as u32,
            AddressingMode::Indirect => {
                let indirect = self.get_direct_addr(bus);
                self.get_indirect_addr(bus, indirect)
            }
            AddressingMode::IndirectX => {
                let indirect = self.get_direct_addr(bus).wrapping_add(self.index_x);
                self.get_indirect_addr(bus, indirect)
            }
            AddressingMode::IndirectY => {
                let indirect = self.get_direct_addr(bus);
                (self.get_indirect_addr(bus, indirect) + self.index_y as u32) & 0xFF_FFFF
            }
            AddressingMode::IndirectLong => {
                let indirect = self.get_direct_addr(bus) as u32;
                Self::get_indirect_long_addr(bus, indirect)
            }
            AddressingMode::IndirectLongY => {
                let indirect = self.get_direct_addr(bus) as u32;
                (Self::get_indirect_long_addr(bus, indirect) + self.index_y as u32) & 0xFF_FFFF
            }
            AddressingMode::Absolute => self.get_absolute_addr(bus),
            AddressingMode::AbsoluteX => {
                (self.get_absolute_addr(bus) + self.index_x as u32) & 0xFF_FFFF
            }
            AddressingMode::AbsoluteY => {
                (self.get_absolute_addr(bus) + self.index_y as u32) & 0xFF_FFFF
            }
            AddressingMode::AbsoluteLong => self.get_absolute_long_addr(bus),
            AddressingMode::AbsoluteLongX => {
                (self.get_absolute_long_addr(bus) + self.index_x as u32) & 0xFF_FFFF
            }
            AddressingMode::AbsoluteIndirect => todo!(),
            AddressingMode::AbsoluteIndirectLong => todo!(),
            AddressingMode::AbsoluteIndirectX => todo!(),
            AddressingMode::StackRelative => self.get_stack_relative_addr(bus).into(),
            AddressingMode::StackRelativeIndirectY => {
                let indirect = self.get_stack_relative_addr(bus);
                (self.get_indirect_addr(bus, indirect) + self.index_y as u32) & 0xFF_FFFF
            }
            AddressingMode::BlockMove => todo!(),
        }
    }

    pub fn get_operand<T: RegSize>(&mut self, bus: &Bus, mode: &AddressingMode) -> (T, u8) {
        let addr = self.get_address(bus, mode);
        if T::IS_U16 {
            (T::trunc_u16(Self::read_16(bus, addr)), 1)
        } else {
            (T::ext_u8(Self::read_8(bus, addr)), 0)
        }
    }

    pub fn do_rmw<T: RegSize>(
        &mut self,
        bus: &Bus,
        mode: &AddressingMode,
        f: fn(&mut Cpu, T) -> T,
    ) {
        let addr = self.get_address(bus, mode);
        if T::IS_U16 {
            let data = Self::read_16(bus, addr);
            let result = f(self, T::trunc_u16(data)).as_u16();
            Self::write_16(bus, addr, result);
        } else {
            let data = Self::read_8(bus, addr);
            let result = f(self, T::ext_u8(data)).as_u8();
            Self::write_8(bus, addr, result);
        }
    }
}
