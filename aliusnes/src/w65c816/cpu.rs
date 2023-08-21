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
    pub extra_cycles: u8,
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
            extra_cycles: 0,
        }
    }

    pub fn set_low_a(&mut self, val: u16) {
        self.accumulator = (self.accumulator & 0xFF00) | val;
    }

    fn step(&mut self, bus: &mut Bus) -> u8 {
        self.extra_cycles = 0;
        let op = self.get_imm::<u8>(bus);

        // let opcode = OPCODES_MAP
        //     .get(&op)
        //     .expect(&format!("OpCode {:x} is not recognized", op));

        // let instr = opcode.function;
        // instr(self, bus, &opcode.mode);

        // let cycles = opcode.cycles + self.extra_cycles;
        // cycles
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

    fn add_extra_cycles<const WRITE: bool>(&mut self, unindexed: u32, indexed: u32) {
        if WRITE || unindexed >> 8 != indexed >> 8 {
            self.extra_cycles += 1;
        }
    }

    pub fn get_imm<T: RegSize>(&mut self, bus: &Bus) -> T {
        if T::IS_U16 {
            self.extra_cycles += 1;
            let pbr = self.pbr as u16;
            let res = Self::read_16(bus, (pbr | self.program_couter).into());
            self.program_couter = self.program_couter.wrapping_add(2);
            T::trunc_u16(res)
        } else {
            let pbr = self.pbr as u16;
            let res = Self::read_8(bus, (pbr | self.program_couter).into());
            self.program_couter = self.program_couter.wrapping_add(1);
            T::ext_u8(res)
        }
    }

    fn get_direct_addr(&mut self, bus: &Bus) -> u16 {
        let dpr = self.dpr;
        if dpr as u8 != 0 {
            self.extra_cycles += 1;
        }
        dpr.wrapping_add(self.get_imm::<u8>(bus).into())
    }

    fn get_indirect_addr(&self, bus: &Bus, addr: u16) -> u32 {
        (Self::read_16(bus, addr.into()) | self.dbr as u16).into()
    }

    fn get_indirect_long_addr(bus: &Bus, addr: u32) -> u32 {
        ((Self::read_16(bus, addr) | Self::read_8(bus, addr.wrapping_add(2)) as u16) as u32) << 16
    }

    fn get_absolute_addr(&mut self, bus: &Bus) -> u16 {
        self.dbr as u16 | self.get_imm::<u16>(bus)
    }

    fn get_absolute_long_addr(&mut self, bus: &Bus) -> u32 {
        self.get_imm::<u16>(bus) as u32 | self.get_imm::<u8>(bus) as u32
    }

    fn get_stack_relative_addr(&mut self, bus: &Bus) -> u16 {
        self.stack_pointer
            .wrapping_add(self.get_imm::<u8>(bus).into())
    }

    pub fn get_address<const WRITE: bool>(&mut self, bus: &Bus, mode: &AddressingMode) -> u32 {
        match mode {
            AddressingMode::Implied => unreachable!(),
            AddressingMode::Immediate => unreachable!(),
            AddressingMode::Relative => unreachable!(),
            AddressingMode::RelativeLong => unreachable!(),
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
                let unindexed = self.get_indirect_addr(bus, indirect);
                let indexed = (unindexed + self.index_y as u32) & 0xFF_FFFF;
                self.add_extra_cycles::<WRITE>(unindexed, indexed);
                indexed
            }
            AddressingMode::IndirectLong => {
                let indirect = self.get_direct_addr(bus) as u32;
                Self::get_indirect_long_addr(bus, indirect)
            }
            AddressingMode::IndirectLongY => {
                let indirect = self.get_direct_addr(bus) as u32;
                (Self::get_indirect_long_addr(bus, indirect) + self.index_y as u32) & 0xFF_FFFF
            }
            AddressingMode::Absolute => self.get_absolute_addr(bus) as u32,
            AddressingMode::AbsoluteX => {
                let unindexed = self.get_absolute_addr(bus) as u32;
                let indexed = unindexed + (self.index_x as u32) & 0xFF_FFFF;
                self.add_extra_cycles::<WRITE>(unindexed, indexed);
                indexed
            }
            AddressingMode::AbsoluteY => {
                let unindexed = self.get_absolute_addr(bus) as u32;
                let indexed = unindexed + (self.index_y as u32) & 0xFF_FFFF;
                self.add_extra_cycles::<WRITE>(unindexed, indexed);
                indexed
            }
            AddressingMode::AbsoluteLong => self.get_absolute_long_addr(bus),
            AddressingMode::AbsoluteLongX => {
                (self.get_absolute_long_addr(bus) + self.index_x as u32) & 0xFF_FFFF
            }
            AddressingMode::AbsoluteIndirect => {
                let indirect = self.get_absolute_addr(bus) as u16;
                self.get_indirect_addr(bus, indirect)
            }
            AddressingMode::AbsoluteIndirectLong => todo!(),
            AddressingMode::AbsoluteIndirectX => {
                let indirect = self.get_absolute_addr(bus) + self.index_x;
                self.get_indirect_addr(bus, indirect)
            }
            AddressingMode::StackRelative => self.get_stack_relative_addr(bus).into(),
            AddressingMode::StackRelativeIndirectY => {
                let indirect = self.get_stack_relative_addr(bus);
                (self.get_indirect_addr(bus, indirect) + self.index_y as u32) & 0xFF_FFFF
            }
            AddressingMode::BlockMove => unreachable!(),
        }
    }

    pub fn get_operand<T: RegSize>(&mut self, bus: &Bus, mode: &AddressingMode) -> T {
        match mode {
            AddressingMode::Immediate
            | AddressingMode::Relative
            | AddressingMode::RelativeLong
            | AddressingMode::BlockMove => self.get_imm(bus),
            _ => {
                let addr = self.get_address::<false>(bus, mode);
                if T::IS_U16 {
                    T::trunc_u16(Self::read_16(bus, addr))
                } else {
                    T::ext_u8(Self::read_8(bus, addr))
                }
            }
        }
    }

    pub fn do_write<T: RegSize>(&mut self, bus: &mut Bus, mode: &AddressingMode, val: T) {
        let addr = self.get_address::<true>(bus, mode);
        if T::IS_U16 {
            Cpu::write_16(bus, addr, val.as_u16());
        } else {
            Cpu::write_8(bus, addr, val.as_u8());
        }
    }

    pub fn do_rmw<T: RegSize>(
        &mut self,
        bus: &Bus,
        mode: &AddressingMode,
        f: fn(&mut Cpu, T) -> T,
    ) {
        let addr = self.get_address::<true>(bus, mode);
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
