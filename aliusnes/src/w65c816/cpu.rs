// use super::opcodes::OPCODES_MAP;
use crate::bus::Bus;
use std::collections::HashMap;

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
    pub register_a: u16,
    pub register_x: u16,
    pub register_y: u16,
    pub stack_pointer: u16,
    pub program_couter: u16,
    pub status: CpuFlags,
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
            register_a: 0x00,
            register_x: 0x00,
            register_y: 0x00,
            stack_pointer: 0x00,
            status: CpuFlags::from_bits_truncate(0b10010000),
            dpr: 0x00,
            pbr: 0x00,
            dbr: 0x00,
            program_couter: 0x00,
        }
    }

    pub fn set_low_a(&mut self, val: u8) {
        self.register_a = (self.register_a & 0xFF00) | (val as u16);
    }

    fn step(&mut self, bus: &mut Bus) -> usize {
        let op = self.pbr;

        // let opcode = OPCODES_MAP
        //     .get(&op)
        //     .expect(&format!("OpCode {:x} is not recognized", op));
        // let instr = opcode.function;

        // let data = bus.read(self.get_operand_address(&opcode.mode));

        // instr(self, bus, &opcode.mode);
        1
    }

    pub(super) fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Implied => self.program_couter,

            AddressingMode::Immediate => self.program_couter,

            AddressingMode::Relative => self.program_couter,
            AddressingMode::RelativeLong => todo!(),
            AddressingMode::Direct => todo!(),
            AddressingMode::DirectX => todo!(),
            AddressingMode::DirectY => todo!(),
            AddressingMode::Indirect => todo!(),
            AddressingMode::IndirectX => todo!(),
            AddressingMode::IndirectY => todo!(),
            AddressingMode::IndirectLong => todo!(),
            AddressingMode::IndirectLongY => todo!(),
            AddressingMode::Absolute => todo!(),
            AddressingMode::AbsoluteX => todo!(),
            AddressingMode::AbsoluteY => todo!(),
            AddressingMode::AbsoluteLong => todo!(),
            AddressingMode::AbsoluteLongX => todo!(),
            AddressingMode::AbsoluteIndirect => todo!(),
            AddressingMode::AbsoluteIndirectLong => todo!(),
            AddressingMode::AbsoluteIndirectX => todo!(),
            AddressingMode::StackRelative => todo!(),
            AddressingMode::StackRelativeIndirectY => todo!(),
            AddressingMode::BlockMove => todo!(),
        }
    }
}
