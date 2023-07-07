use crate::{bus::Bus, w65c816::cpu::AddressingMode};
use std::collections::HashMap;

use super::{cpu::Cpu, instruction::brk};

pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub bytes: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
    pub function: for<'a, 'b, 'c> fn(&mut Cpu, &mut Bus, &AddressingMode) -> u8,
}

impl OpCode {
    fn new(
        code: u8,
        mnemonic: &'static str,
        bytes: u8,
        cycles: u8,
        mode: AddressingMode,
        function: for<'a> fn(&mut Cpu, &mut Bus, &AddressingMode) -> u8,
    ) -> Self {
        OpCode {
            code,
            mnemonic,
            bytes,
            cycles,
            mode,
            function,
        }
    }
}

lazy_static! {
    pub static ref CPU_OP_CODES: Vec<OpCode> = vec![
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::Implied, brk), //check addressingmode, PC+2

        OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x65, "ADC", 2, 3, AddressingMode::Direct),
        OpCode::new(0x75, "ADC", 2, 4, AddressingMode::DirectX),
        OpCode::new(0x6d, "ADC", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x7d, "ADC", 3, 4, AddressingMode::AbsoluteX), // * +1 penalty if cross page boundary
        OpCode::new(0x79, "ADC", 3, 4, AddressingMode::AbsoluteY), // * penalty +1
        OpCode::new(0x61, "ADC", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x71, "ADC", 2, 5, AddressingMode::IndirectY), // * penalty +1
        OpCode::new(0x6f, "ADC", 4, 5, AddressingMode::AbsoluteLong),
        OpCode::new(0x7f, "ADC", 4, 5, AddressingMode::AbsoluteLongX),
        OpCode::new(0x72, "ADC", 2, 5, AddressingMode::Indirect),
        OpCode::new(0x67, "ADC", 2, 6, AddressingMode::IndirectLong),
        OpCode::new(0x77, "ADC", 2, 6, AddressingMode::IndirectLongY),
        OpCode::new(0x63, "ADC", 2, 4, AddressingMode::StackRelative),
        OpCode::new(0x73, "ADC", 2, 7, AddressingMode::StackRelativeIndirectY),

        OpCode::new(0x09, "ORA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x05, "ORA", 2, 3, AddressingMode::Direct),
        OpCode::new(0x15, "ORA", 2, 4, AddressingMode::DirectX),
        OpCode::new(0x0d, "ORA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x1d, "ORA", 3, 4, AddressingMode::AbsoluteX), // *
        OpCode::new(0x19, "ORA", 3, 4, AddressingMode::AbsoluteY), // *
        OpCode::new(0x01, "ORA", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x11, "ORA", 2, 5, AddressingMode::IndirectY), // *
        OpCode::new(0x0f, "ORA", 4, 5, AddressingMode::AbsoluteLong),
        OpCode::new(0x1f, "ORA", 4, 5, AddressingMode::AbsoluteLongX),
        OpCode::new(0x12, "ORA", 2, 5, AddressingMode::Indirect),
        OpCode::new(0x07, "ORA", 2, 6, AddressingMode::IndirectLong),
        OpCode::new(0x17, "ORA", 2, 6, AddressingMode::IndirectLongY),
        OpCode::new(0x03, "ORA", 2, 4, AddressingMode::StackRelative),
        OpCode::new(0x13, "ORA", 2, 7, AddressingMode::StackRelativeIndirectY),

        OpCode::new(0x02, "COP", 2, 7, AddressingMode::Immediate), // maybe 8 cycles

        OpCode::new(0x0c, "TSB", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x04, "TSB", 2, 3, AddressingMode::Direct),

        OpCode::new(0x0a, "ASL", 1, 2, AddressingMode::Implied),
        OpCode::new(0x06, "ASL", 2, 5, AddressingMode::Direct),
        OpCode::new(0x16, "ASL", 2, 6, AddressingMode::DirectX),
        OpCode::new(0x0e, "ASL", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x1e, "ASL", 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x10, "BPL", 2, 2, AddressingMode::Implied), // +1 if branch same page, +2 otherwise, check addr mode

        OpCode::new(0x1c, "TRB", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x14, "TRB", 2, 3, AddressingMode::Direct),

        OpCode::new(0x20, "JSR", 3, 6, AddressingMode::Absolute),
        OpCode::new(0xfc, "JSR", 3, 6, AddressingMode::AbsoluteIndirectX),

        OpCode::new(0x29, "AND", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x25, "AND", 2, 3, AddressingMode::Direct),
        OpCode::new(0x35, "AND", 2, 4, AddressingMode::DirectX),
        OpCode::new(0x2d, "AND", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x3d, "AND", 3, 4, AddressingMode::AbsoluteX), // *
        OpCode::new(0x39, "AND", 3, 4, AddressingMode::AbsoluteY), // *
        OpCode::new(0x21, "AND", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x31, "AND", 2, 5, AddressingMode::IndirectY), // *
        OpCode::new(0x2f, "AND", 4, 5, AddressingMode::AbsoluteLong),
        OpCode::new(0x3f, "AND", 4, 5, AddressingMode::AbsoluteLongX),
        OpCode::new(0x32, "AND", 2, 5, AddressingMode::Indirect),
        OpCode::new(0x27, "AND", 2, 6, AddressingMode::IndirectLong),
        OpCode::new(0x37, "AND", 2, 6, AddressingMode::IndirectLongY),
        OpCode::new(0x23, "AND", 2, 4, AddressingMode::StackRelative),
        OpCode::new(0x33, "AND", 2, 7, AddressingMode::StackRelativeIndirectY),

        OpCode::new(0x22, "JSL", 4, 8, AddressingMode::AbsoluteLong),

        OpCode::new(0x24, "BIT", 2, 3, AddressingMode::Direct),
        OpCode::new(0x2c, "BIT", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x89, "BIT", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x34, "BIT", 2, 3, AddressingMode::DirectX),
        OpCode::new(0x3c, "BIT", 3, 4, AddressingMode::AbsoluteX),

        OpCode::new(0x2a, "ROL", 1, 2, AddressingMode::Implied),
        OpCode::new(0x26, "ROL", 2, 5, AddressingMode::Direct),
        OpCode::new(0x36, "ROL", 2, 6, AddressingMode::DirectX),
        OpCode::new(0x2e, "ROL", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x3e, "ROL", 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x30, "BMI", 2, 2, AddressingMode::Implied), // +1 if branch same page, +2 otherwise, check addr mode

        OpCode::new(0x40, "RTI", 1, 6, AddressingMode::StackRTI),

        OpCode::new(0x49, "EOR", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x45, "EOR", 2, 3, AddressingMode::Direct),
        OpCode::new(0x55, "EOR", 2, 4, AddressingMode::DirectX),
        OpCode::new(0x4d, "EOR", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x5d, "EOR", 3, 4, AddressingMode::AbsoluteX), // *
        OpCode::new(0x59, "EOR", 3, 4, AddressingMode::AbsoluteY), // *
        OpCode::new(0x41, "EOR", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x51, "EOR", 2, 5, AddressingMode::IndirectY), // *
        OpCode::new(0x4f, "EOR", 4, 5, AddressingMode::AbsoluteLong),
        OpCode::new(0x5f, "EOR", 4, 5, AddressingMode::AbsoluteLongX),
        OpCode::new(0x52, "EOR", 2, 5, AddressingMode::Indirect),
        OpCode::new(0x47, "EOR", 2, 6, AddressingMode::IndirectLong),
        OpCode::new(0x57, "EOR", 2, 6, AddressingMode::IndirectLongY),
        OpCode::new(0x43, "EOR", 2, 4, AddressingMode::StackRelative),
        OpCode::new(0x53, "EOR", 2, 7, AddressingMode::StackRelativeIndirectY),

        OpCode::new(0x44, "MVP", 3, 7, AddressingMode::BlockMovePositive), // better check later, cycles maybe wrong

        OpCode::new(0x4a, "LSR", 1, 2, AddressingMode::Implied),
        OpCode::new(0x46, "LSR", 2, 5, AddressingMode::Direct),
        OpCode::new(0x56, "LSR", 2, 6, AddressingMode::DirectX),
        OpCode::new(0x4e, "LSR", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x5e, "LSR", 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x50, "BVC", 2, 2, AddressingMode::Implied), // +1 if branch same page, +2 otherwise, check addr mode

        OpCode::new(0x54, "MVN", 3, 7, AddressingMode::BlockMoveNegtive), // better check later, cycles maybe wrong

        OpCode::new(0x60, "RTS", 1, 6, AddressingMode::Implied),

        OpCode::new(0x62, "PER", 3, 6, AddressingMode::Implied),

        OpCode::new(0x9c, "STZ", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x9e, "STZ", 3, 5, AddressingMode::AbsoluteX),
        OpCode::new(0x64, "STZ", 2, 3, AddressingMode::Direct),
        OpCode::new(0x74, "STZ", 2, 4, AddressingMode::DirectX),

        OpCode::new(0x6a, "ROR", 1, 2, AddressingMode::Implied),
        OpCode::new(0x66, "ROR", 2, 5, AddressingMode::Direct),
        OpCode::new(0x76, "ROR", 2, 6, AddressingMode::DirectX),
        OpCode::new(0x6e, "ROR", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x7e, "ROR", 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x70, "BVS", 2, 2, AddressingMode::Implied), // +1 if branch same page, +2 otherwise, check addr mode

        OpCode::new(0x80, "BRA", 2, 2, AddressingMode::Implied),

        OpCode::new(0x85, "STA", 2, 3, AddressingMode::Direct),
        OpCode::new(0x95, "STA", 2, 4, AddressingMode::DirectX),
        OpCode::new(0x8d, "STA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x9d, "STA", 3, 5, AddressingMode::AbsoluteX),
        OpCode::new(0x99, "STA", 3, 5, AddressingMode::AbsoluteY),
        OpCode::new(0x81, "STA", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x91, "STA", 2, 6, AddressingMode::IndirectY),
        OpCode::new(0x8f, "STA", 4, 5, AddressingMode::AbsoluteLong),
        OpCode::new(0x9f, "STA", 4, 5, AddressingMode::AbsoluteLongX),
        OpCode::new(0x92, "STA", 2, 5, AddressingMode::Indirect),
        OpCode::new(0x87, "STA", 2, 6, AddressingMode::IndirectLong),
        OpCode::new(0x97, "STA", 2, 6, AddressingMode::IndirectLongY),
        OpCode::new(0x83, "STA", 2, 4, AddressingMode::StackRelative),
        OpCode::new(0x93, "STA", 2, 7, AddressingMode::StackRelativeIndirectY),

        OpCode::new(0x82, "BRL", 3, 3, AddressingMode::Implied),

        OpCode::new(0x84, "STY", 2, 3, AddressingMode::Direct),
        OpCode::new(0x94, "STY", 2, 4, AddressingMode::DirectX),
        OpCode::new(0x8c, "STY", 3, 4, AddressingMode::Absolute),

        OpCode::new(0x86, "STX", 2, 3, AddressingMode::Direct),
        OpCode::new(0x96, "STX", 2, 4, AddressingMode::DirectY),
        OpCode::new(0x8e, "STX", 3, 4, AddressingMode::Absolute),

        OpCode::new(0x90, "BCC", 2, 2, AddressingMode::Implied), // +1 if branch same page, +2 otherwise, check addr mode

        OpCode::new(0xa0, "LDY", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa4, "LDY", 2, 3, AddressingMode::Direct),
        OpCode::new(0xb4, "LDY", 2, 4, AddressingMode::DirectX),
        OpCode::new(0xac, "LDY", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbc, "LDY", 3, 4, AddressingMode::AbsoluteX), // *

        OpCode::new(0xa9, "LDA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa5, "LDA", 2, 3, AddressingMode::Direct),
        OpCode::new(0xb5, "LDA", 2, 4, AddressingMode::DirectX),
        OpCode::new(0xad, "LDA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbd, "LDA", 3, 4, AddressingMode::AbsoluteX), // *
        OpCode::new(0xb9, "LDA", 3, 4, AddressingMode::AbsoluteY), // *
        OpCode::new(0xa1, "LDA", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0xb1, "LDA", 2, 5, AddressingMode::IndirectY), // *
        OpCode::new(0xaf, "LDA", 4, 5, AddressingMode::AbsoluteLong),
        OpCode::new(0xbf, "LDA", 4, 5, AddressingMode::AbsoluteLongX),
        OpCode::new(0xb2, "LDA", 2, 5, AddressingMode::Indirect),
        OpCode::new(0xa7, "LDA", 2, 6, AddressingMode::IndirectLong),
        OpCode::new(0xb7, "LDA", 2, 6, AddressingMode::IndirectLongY),
        OpCode::new(0xa3, "LDA", 2, 4, AddressingMode::StackRelative),
        OpCode::new(0xb3, "LDA", 2, 7, AddressingMode::StackRelativeIndirectY),

        OpCode::new(0xa2, "LDX", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa6, "LDX", 2, 3, AddressingMode::Direct),
        OpCode::new(0xb6, "LDX", 2, 4, AddressingMode::DirectY),
        OpCode::new(0xae, "LDX", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbe, "LDX", 3, 4, AddressingMode::AbsoluteY), // *

        OpCode::new(0xb0, "BCS", 2, 2, AddressingMode::Implied), // +1 if branch same page, +2 otherwise, check addr mode

        OpCode::new(0xc0, "CPY", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xc4, "CPY", 2, 3, AddressingMode::Direct),
        OpCode::new(0xcc, "CPY", 3, 4, AddressingMode::Absolute),

        OpCode::new(0xc9, "CMP", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xc5, "CMP", 2, 3, AddressingMode::Direct),
        OpCode::new(0xd5, "CMP", 2, 4, AddressingMode::DirectX),
        OpCode::new(0xcd, "CMP", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xdd, "CMP", 3, 4, AddressingMode::AbsoluteX), // *
        OpCode::new(0xd9, "CMP", 3, 4, AddressingMode::AbsoluteY), // *
        OpCode::new(0xc1, "CMP", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0xd1, "CMP", 2, 5, AddressingMode::IndirectY), // *
        OpCode::new(0xcf, "CMP", 4, 5, AddressingMode::AbsoluteLong),
        OpCode::new(0xdf, "CMP", 4, 5, AddressingMode::AbsoluteLongX),
        OpCode::new(0xd2, "CMP", 2, 5, AddressingMode::Indirect),
        OpCode::new(0xc7, "CMP", 2, 6, AddressingMode::IndirectLong),
        OpCode::new(0xd7, "CMP", 2, 6, AddressingMode::IndirectLongY),
        OpCode::new(0xc3, "CMP", 2, 4, AddressingMode::StackRelative),
        OpCode::new(0xd3, "CMP", 2, 7, AddressingMode::StackRelativeIndirectY),

        OpCode::new(0xc2, "REP", 2, 3, AddressingMode::Immediate),

        OpCode::new(0xc6, "DEC", 2, 5, AddressingMode::Direct),
        OpCode::new(0xd6, "DEC", 2, 6, AddressingMode::DirectX),
        OpCode::new(0xce, "DEC", 3, 6, AddressingMode::Absolute),
        OpCode::new(0xde, "DEC", 3, 7, AddressingMode::AbsoluteX),
        OpCode::new(0x3a, "DEC", 1, 2, AddressingMode::Implied),

        OpCode::new(0xd0, "BNE", 2, 2, AddressingMode::Implied), // +1 if branch same page, +2 otherwise, check addr mode

        OpCode::new(0xd4, "PEI", 2, 6, AddressingMode::Indirect), // Add one cycle if low byte of direct page is <>0

        OpCode::new(0xe0, "CPX", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xe4, "CPX", 2, 3, AddressingMode::Direct),
        OpCode::new(0xec, "CPX", 3, 4, AddressingMode::Absolute),

        OpCode::new(0xe9, "SBC", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xe5, "SBC", 2, 3, AddressingMode::Direct),
        OpCode::new(0xf5, "SBC", 2, 4, AddressingMode::DirectX),
        OpCode::new(0xed, "SBC", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xfd, "SBC", 3, 4, AddressingMode::AbsoluteX), // *
        OpCode::new(0xf9, "SBC", 3, 4, AddressingMode::AbsoluteY), // *
        OpCode::new(0xe1, "SBC", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0xf1, "SBC", 2, 5, AddressingMode::IndirectY), // *
        OpCode::new(0xef, "SBC", 4, 5, AddressingMode::AbsoluteLong),
        OpCode::new(0xff, "SBC", 4, 5, AddressingMode::AbsoluteLongX),
        OpCode::new(0xf2, "SBC", 2, 5, AddressingMode::Indirect),
        OpCode::new(0xe7, "SBC", 2, 6, AddressingMode::IndirectLong),
        OpCode::new(0xf7, "SBC", 2, 6, AddressingMode::IndirectLongY),
        OpCode::new(0xe3, "SBC", 2, 4, AddressingMode::StackRelative),
        OpCode::new(0xf3, "SBC", 2, 7, AddressingMode::StackRelativeIndIndexY),

        OpCode::new(0xe2, "SEP", 2, 3, AddressingMode::Immediate),

        OpCode::new(0xe6, "INC", 2, 5, AddressingMode::Direct),
        OpCode::new(0xf6, "INC", 2, 6, AddressingMode::DirectX),
        OpCode::new(0xee, "INC", 3, 6, AddressingMode::Absolute),
        OpCode::new(0xfe, "INC", 3, 7, AddressingMode::AbsoluteX),
        OpCode::new(0x1a, "INC", 1, 2, AddressingMode::Implied),

        OpCode::new(0xf0, "BEQ", 2, 2, AddressingMode::Implied), // +1 if branch same page, +2 otherwise, check addr mode
        OpCode::new(0xf4, "PEA", 3, 5, AddressingMode::Immediate),
        OpCode::new(0x08, "PHP", 1, 3, AddressingMode::Implied),
        OpCode::new(0x0b, "PHD", 1, 4, AddressingMode::Implied),
        OpCode::new(0x18, "CLC", 1, 2, AddressingMode::Implied),
        OpCode::new(0x1b, "TCS", 1, 2, AddressingMode::Implied),
        OpCode::new(0x28, "PLP", 1, 4, AddressingMode::Implied),
        OpCode::new(0x2b, "PLD", 1, 5, AddressingMode::Implied),
        OpCode::new(0x38, "SEC", 1, 2, AddressingMode::Implied),
        OpCode::new(0x3b, "TSC", 1, 2, AddressingMode::Implied),
        OpCode::new(0x48, "PHA", 1, 3, AddressingMode::Implied),
        OpCode::new(0x4b, "PHK", 1, 3, AddressingMode::Implied),

        OpCode::new(0x4c, "JMP", 3, 3, AddressingMode::Absolute),
        OpCode::new(0x6c, "JMP", 3, 5, AddressingMode::Absolute),
        OpCode::new(0x7c, "JMP", 3, 6, AddressingMode::AbsoluteIndirectX),
        OpCode::new(0x5c, "JMP", 4, 4, AddressingMode::AbsoluteLong),

        OpCode::new(0x58, "CLI", 1, 2, AddressingMode::Implied),
        OpCode::new(0x5a, "PHY", 1, 3, AddressingMode::Implied),
        OpCode::new(0x5b, "TCD", 1, 2, AddressingMode::Implied),
        OpCode::new(0x68, "PLA", 1, 4, AddressingMode::Implied),
        OpCode::new(0x6b, "RTL", 1, 6, AddressingMode::Implied),
        OpCode::new(0x78, "SEI", 1, 2, AddressingMode::Implied),
        OpCode::new(0x7a, "PLY", 1, 4, AddressingMode::Implied), // maybe 5
        OpCode::new(0x7b, "TDC", 1, 2, AddressingMode::Implied),
        OpCode::new(0x88, "DEY", 1, 2, AddressingMode::Implied),
        OpCode::new(0x8a, "TXA", 1, 2, AddressingMode::Implied),
        OpCode::new(0x8b, "PHB", 1, 3, AddressingMode::Implied),
        OpCode::new(0x98, "TYA", 1, 2, AddressingMode::Implied),
        OpCode::new(0x9a, "TXS", 1, 2, AddressingMode::Implied),
        OpCode::new(0x9b, "TXY", 1, 2, AddressingMode::Implied),
        OpCode::new(0xa8, "TAY", 1, 2, AddressingMode::Implied),
        OpCode::new(0xaa, "TAX", 1, 2, AddressingMode::Implied),
        OpCode::new(0xab, "PLB", 1, 4, AddressingMode::Implied),
        OpCode::new(0xb8, "CLV", 1, 2, AddressingMode::Implied),
        OpCode::new(0xba, "TSX", 1, 2, AddressingMode::Implied),
        OpCode::new(0xbb, "TYX", 1, 2, AddressingMode::Implied),
        OpCode::new(0xc8, "INY", 1, 2, AddressingMode::Implied),
        OpCode::new(0xca, "DEX", 1, 2, AddressingMode::Implied),
        OpCode::new(0xcb, "WAI", 1, 3, AddressingMode::Implied), // maybe more than 3
        OpCode::new(0xd8, "CLD", 1, 2, AddressingMode::Implied),
        OpCode::new(0xda, "PHX", 1, 3, AddressingMode::Implied), // maybe 4
        OpCode::new(0xdb, "STP", 1, 3, AddressingMode::Implied), // maybe 4
        OpCode::new(0xdc, "JML", 3, 6, AddressingMode::Implied), // check AddressingMode
        OpCode::new(0xe8, "INX", 1, 2, AddressingMode::Implied),
        OpCode::new(0xea, "NOP", 1, 2, AddressingMode::Implied),
        OpCode::new(0xeb, "XBA", 1, 3, AddressingMode::Implied),
        OpCode::new(0xf8, "SED", 1, 2, AddressingMode::Implied),
        OpCode::new(0xfa, "PLX", 1, 4, AddressingMode::Implied),
        OpCode::new(0xfb, "XCE", 1, 2, AddressingMode::Implied),
    ];

    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        let mut map = HashMap::new();
        for op in &*CPU_OP_CODES {
            map.insert(op.code, op);
        }
        map
    };
}