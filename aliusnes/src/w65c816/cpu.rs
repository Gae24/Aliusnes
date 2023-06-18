// use super::opcodes::OPCODES_MAP;
use crate::bus::Bus;

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

    pub fn set_low_a(&mut self, val: u8) {
        self.accumulator = (self.accumulator & 0xFF00) | (val as u16);
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

    pub fn get_operand_address(
        &mut self,
        mode: &AddressingMode,
        is_regs_8bit: bool,
    ) -> (u16, u8) {
        match mode {
            AddressingMode::Implied => {
                self.program_couter += 1;
                (0, 0)
            }
            AddressingMode::Immediate => {
                if is_regs_8bit {
                    self.program_couter += 1;
                    return ((self.pbr << 16 | (self.program_couter as u8)).into(), 0);
                } else {
                    self.program_couter += 2;
                    return ((self.pbr << 16 | ((self.program_couter - 1) as u8)).into(), 1);
                }
            }
            AddressingMode::Relative => {
                self.program_couter += 1;
                return ((self.pbr << 16 | (self.program_couter as u8)).into(), 0);
            }
            AddressingMode::RelativeLong => {
                self.program_couter += 2;
                return ((self.pbr << 16 | ((self.program_couter - 1) as u8)).into(), 0);
            }
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
