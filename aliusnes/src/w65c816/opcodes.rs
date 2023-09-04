use crate::{
    bus::bus::Bus,
    w65c816::{cpu::AddressingMode, cpu::Cpu, instructions::*},
};
use std::collections::HashMap;

pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub cycles: u8,
    pub mode: AddressingMode,
    pub function: for<'a, 'b, 'c> fn(&mut Cpu, &mut Bus, &AddressingMode),
}

impl OpCode {
    fn new(
        code: u8,
        mnemonic: &'static str,
        cycles: u8,
        mode: AddressingMode,
        function: for<'a, 'b, 'c> fn(&mut Cpu, &mut Bus, &AddressingMode),
    ) -> Self {
        OpCode {
            code,
            mnemonic,
            cycles,
            mode,
            function,
        }
    }
}

lazy_static! {
    pub static ref CPU_OP_CODES: Vec<OpCode> = vec![
        OpCode::new(0x69, "ADC", 2, AddressingMode::Immediate, adc),
        OpCode::new(0x65, "ADC", 3, AddressingMode::Direct, adc),
        OpCode::new(0x75, "ADC", 4, AddressingMode::DirectX, adc),
        OpCode::new(0x6d, "ADC", 4, AddressingMode::Absolute, adc),
        OpCode::new(0x7d, "ADC", 4, AddressingMode::AbsoluteX, adc),
        OpCode::new(0x79, "ADC", 4, AddressingMode::AbsoluteY, adc),
        OpCode::new(0x61, "ADC", 6, AddressingMode::IndirectX, adc),
        OpCode::new(0x71, "ADC", 5, AddressingMode::IndirectY, adc),
        OpCode::new(0x6f, "ADC", 5, AddressingMode::AbsoluteLong, adc),
        OpCode::new(0x7f, "ADC", 5, AddressingMode::AbsoluteLongX, adc),
        OpCode::new(0x72, "ADC", 5, AddressingMode::Indirect, adc),
        OpCode::new(0x67, "ADC", 6, AddressingMode::IndirectLong, adc),
        OpCode::new(0x77, "ADC", 6, AddressingMode::IndirectLongY, adc),
        OpCode::new(0x63, "ADC", 4, AddressingMode::StackRelative, adc),
        OpCode::new(0x73, "ADC", 7, AddressingMode::StackRelativeIndirectY, adc),

        OpCode::new(0x29, "AND", 2, AddressingMode::Immediate, and),
        OpCode::new(0x25, "AND", 3, AddressingMode::Direct, and),
        OpCode::new(0x35, "AND", 4, AddressingMode::DirectX, and),
        OpCode::new(0x2d, "AND", 4, AddressingMode::Absolute, and),
        OpCode::new(0x3d, "AND", 4, AddressingMode::AbsoluteX, and),
        OpCode::new(0x39, "AND", 4, AddressingMode::AbsoluteY, and),
        OpCode::new(0x21, "AND", 6, AddressingMode::IndirectX, and),
        OpCode::new(0x31, "AND", 5, AddressingMode::IndirectY, and),
        OpCode::new(0x2f, "AND", 5, AddressingMode::AbsoluteLong, and),
        OpCode::new(0x3f, "AND", 5, AddressingMode::AbsoluteLongX, and),
        OpCode::new(0x32, "AND", 5, AddressingMode::Indirect, and),
        OpCode::new(0x27, "AND", 6, AddressingMode::IndirectLong, and),
        OpCode::new(0x37, "AND", 6, AddressingMode::IndirectLongY, and),
        OpCode::new(0x23, "AND", 4, AddressingMode::StackRelative, and),
        OpCode::new(0x33, "AND", 7, AddressingMode::StackRelativeIndirectY, and),

        OpCode::new(0x0a, "ASL", 2, AddressingMode::Implied, asl_a),
        OpCode::new(0x06, "ASL", 5, AddressingMode::Direct, asl),
        OpCode::new(0x16, "ASL", 6, AddressingMode::DirectX, asl),
        OpCode::new(0x0e, "ASL", 6, AddressingMode::Absolute, asl),
        OpCode::new(0x1e, "ASL", 7, AddressingMode::AbsoluteX, asl),

        OpCode::new(0x90, "BCC", 2, AddressingMode::Relative, bcc), // +1 if branch same page, +2 otherwise, check addr mode
        OpCode::new(0xb0, "BCS", 2, AddressingMode::Relative, bcs), // +1 if branch same page, +2 otherwise, check addr mode
        OpCode::new(0xf0, "BEQ", 2, AddressingMode::Relative, beq), // +1 if branch same page, +2 otherwise, check addr mode
        OpCode::new(0x30, "BMI", 2, AddressingMode::Relative, bmi), // +1 if branch same page, +2 otherwise, check addr mode
        OpCode::new(0xd0, "BNE", 2, AddressingMode::Relative, bne), // +1 if branch same page, +2 otherwise, check addr mode
        OpCode::new(0x10, "BPL", 2, AddressingMode::Relative, bpl), // +1 if branch same page, +2 otherwise, check addr mode
        OpCode::new(0x80, "BRA", 2, AddressingMode::Relative, bra),
        OpCode::new(0x82, "BRL", 4, AddressingMode::RelativeLong, brl),
        OpCode::new(0x50, "BVC", 2, AddressingMode::Relative, bvc), // +1 if branch same page, +2 otherwise, check addr mode
        OpCode::new(0x70, "BVS", 2, AddressingMode::Relative, bvs), // +1 if branch same page, +2 otherwise, check addr mode

        OpCode::new(0x24, "BIT", 3, AddressingMode::Direct, bit),
        OpCode::new(0x2c, "BIT", 4, AddressingMode::Absolute, bit),
        OpCode::new(0x89, "BIT", 2, AddressingMode::Immediate, bit),
        OpCode::new(0x34, "BIT", 3, AddressingMode::DirectX, bit),
        OpCode::new(0x3c, "BIT", 4, AddressingMode::AbsoluteX, bit),

        OpCode::new(0x00, "BRK", 8, AddressingMode::Immediate, brk), //check addressingmode, PC+2

        OpCode::new(0x18, "CLC", 2, AddressingMode::Implied, clc),
        OpCode::new(0xd8, "CLD", 2, AddressingMode::Implied, cld),
        OpCode::new(0x58, "CLI", 2, AddressingMode::Implied, cli),
        OpCode::new(0xb8, "CLV", 2, AddressingMode::Implied, clv),

        OpCode::new(0x02, "COP", 8, AddressingMode::Immediate, cop), // maybe 8 cycles

        OpCode::new(0xc9, "CMP", 2, AddressingMode::Immediate, cmp),
        OpCode::new(0xc5, "CMP", 3, AddressingMode::Direct, cmp),
        OpCode::new(0xd5, "CMP", 4, AddressingMode::DirectX, cmp),
        OpCode::new(0xcd, "CMP", 4, AddressingMode::Absolute, cmp),
        OpCode::new(0xdd, "CMP", 4, AddressingMode::AbsoluteX, cmp),
        OpCode::new(0xd9, "CMP", 4, AddressingMode::AbsoluteY, cmp),
        OpCode::new(0xc1, "CMP", 6, AddressingMode::IndirectX, cmp),
        OpCode::new(0xd1, "CMP", 5, AddressingMode::IndirectY, cmp),
        OpCode::new(0xcf, "CMP", 5, AddressingMode::AbsoluteLong, cmp),
        OpCode::new(0xdf, "CMP", 5, AddressingMode::AbsoluteLongX, cmp),
        OpCode::new(0xd2, "CMP", 5, AddressingMode::Indirect, cmp),
        OpCode::new(0xc7, "CMP", 6, AddressingMode::IndirectLong, cmp),
        OpCode::new(0xd7, "CMP", 6, AddressingMode::IndirectLongY, cmp),
        OpCode::new(0xc3, "CMP", 4, AddressingMode::StackRelative, cmp),
        OpCode::new(0xd3, "CMP", 7, AddressingMode::StackRelativeIndirectY, cmp),

        OpCode::new(0xe0, "CPX", 2, AddressingMode::Immediate, cpx),
        OpCode::new(0xe4, "CPX", 3, AddressingMode::Direct, cpx),
        OpCode::new(0xec, "CPX", 4, AddressingMode::Absolute, cpx),

        OpCode::new(0xc0, "CPY", 2, AddressingMode::Immediate, cpy),
        OpCode::new(0xc4, "CPY", 3, AddressingMode::Direct, cpy),
        OpCode::new(0xcc, "CPY", 4, AddressingMode::Absolute, cpy),

        OpCode::new(0xc6, "DEC", 5, AddressingMode::Direct, dec),
        OpCode::new(0xd6, "DEC", 6, AddressingMode::DirectX, dec),
        OpCode::new(0xce, "DEC", 6, AddressingMode::Absolute, dec),
        OpCode::new(0xde, "DEC", 7, AddressingMode::AbsoluteX, dec),
        OpCode::new(0x3a, "DEC", 2, AddressingMode::Implied, dec_a),
        OpCode::new(0xca, "DEX", 2, AddressingMode::Implied, dex),
        OpCode::new(0x88, "DEY", 2, AddressingMode::Implied, dey),

        OpCode::new(0x49, "EOR", 2, AddressingMode::Immediate, eor),
        OpCode::new(0x45, "EOR", 3, AddressingMode::Direct, eor),
        OpCode::new(0x55, "EOR", 4, AddressingMode::DirectX, eor),
        OpCode::new(0x4d, "EOR", 4, AddressingMode::Absolute, eor),
        OpCode::new(0x5d, "EOR", 4, AddressingMode::AbsoluteX, eor),
        OpCode::new(0x59, "EOR", 4, AddressingMode::AbsoluteY, eor),
        OpCode::new(0x41, "EOR", 6, AddressingMode::IndirectX, eor),
        OpCode::new(0x51, "EOR", 5, AddressingMode::IndirectY, eor),
        OpCode::new(0x4f, "EOR", 5, AddressingMode::AbsoluteLong, eor),
        OpCode::new(0x5f, "EOR", 5, AddressingMode::AbsoluteLongX, eor),
        OpCode::new(0x52, "EOR", 5, AddressingMode::Indirect, eor),
        OpCode::new(0x47, "EOR", 6, AddressingMode::IndirectLong, eor),
        OpCode::new(0x57, "EOR", 6, AddressingMode::IndirectLongY, eor),
        OpCode::new(0x43, "EOR", 4, AddressingMode::StackRelative, eor),
        OpCode::new(0x53, "EOR", 7, AddressingMode::StackRelativeIndirectY, eor),

        OpCode::new(0xe6, "INC", 5, AddressingMode::Direct, inc),
        OpCode::new(0xf6, "INC", 6, AddressingMode::DirectX, inc),
        OpCode::new(0xee, "INC", 6, AddressingMode::Absolute, inc),
        OpCode::new(0xfe, "INC", 7, AddressingMode::AbsoluteX, inc),
        OpCode::new(0x1a, "INC", 2, AddressingMode::Implied, inc_a),
        OpCode::new(0xe8, "INX", 2, AddressingMode::Implied, inx),
        OpCode::new(0xc8, "INY", 2, AddressingMode::Implied, iny),

        OpCode::new(0xdc, "JML", 6, AddressingMode::AbsoluteIndirectLong, jml),
        OpCode::new(0x4c, "JMP", 3, AddressingMode::AbsoluteJMP, jmp),
        OpCode::new(0x6c, "JMP", 5, AddressingMode::AbsoluteIndirect, jmp),
        OpCode::new(0x7c, "JMP", 6, AddressingMode::AbsoluteIndirectX, jmp),
        OpCode::new(0x5c, "JMP", 4, AddressingMode::AbsoluteLong, jmp),
        OpCode::new(0x22, "JSL", 8, AddressingMode::AbsoluteLongJSL, jsl),
        OpCode::new(0x20, "JSR", 6, AddressingMode::AbsoluteJMP, jsr),
        OpCode::new(0xfc, "JSR", 6, AddressingMode::AbsoluteIndirectX, jsr),

        OpCode::new(0xa9, "LDA", 2, AddressingMode::Immediate, lda),
        OpCode::new(0xa5, "LDA", 3, AddressingMode::Direct, lda),
        OpCode::new(0xb5, "LDA", 4, AddressingMode::DirectX, lda),
        OpCode::new(0xad, "LDA", 4, AddressingMode::Absolute, lda),
        OpCode::new(0xbd, "LDA", 4, AddressingMode::AbsoluteX, lda),
        OpCode::new(0xb9, "LDA", 4, AddressingMode::AbsoluteY, lda),
        OpCode::new(0xa1, "LDA", 6, AddressingMode::IndirectX, lda),
        OpCode::new(0xb1, "LDA", 5, AddressingMode::IndirectY, lda),
        OpCode::new(0xaf, "LDA", 5, AddressingMode::AbsoluteLong, lda),
        OpCode::new(0xbf, "LDA", 5, AddressingMode::AbsoluteLongX, lda),
        OpCode::new(0xb2, "LDA", 5, AddressingMode::Indirect, lda),
        OpCode::new(0xa7, "LDA", 6, AddressingMode::IndirectLong, lda),
        OpCode::new(0xb7, "LDA", 6, AddressingMode::IndirectLongY, lda),
        OpCode::new(0xa3, "LDA", 4, AddressingMode::StackRelative, lda),
        OpCode::new(0xb3, "LDA", 7, AddressingMode::StackRelativeIndirectY, lda),

        OpCode::new(0xa2, "LDX", 2, AddressingMode::Immediate, ldx),
        OpCode::new(0xa6, "LDX", 3, AddressingMode::Direct, ldx),
        OpCode::new(0xb6, "LDX", 4, AddressingMode::DirectY, ldx),
        OpCode::new(0xae, "LDX", 4, AddressingMode::Absolute, ldx),
        OpCode::new(0xbe, "LDX", 4, AddressingMode::AbsoluteY, ldx),

        OpCode::new(0xa0, "LDY", 2, AddressingMode::Immediate, ldy),
        OpCode::new(0xa4, "LDY", 3, AddressingMode::Direct, ldy),
        OpCode::new(0xb4, "LDY", 4, AddressingMode::DirectX, ldy),
        OpCode::new(0xac, "LDY", 4, AddressingMode::Absolute, ldy),
        OpCode::new(0xbc, "LDY", 4, AddressingMode::AbsoluteX, ldy),

        OpCode::new(0x4a, "LSR", 2, AddressingMode::Implied, lsr_a),
        OpCode::new(0x46, "LSR", 5, AddressingMode::Direct, lsr),
        OpCode::new(0x56, "LSR", 6, AddressingMode::DirectX, lsr),
        OpCode::new(0x4e, "LSR", 6, AddressingMode::Absolute, lsr),
        OpCode::new(0x5e, "LSR", 7, AddressingMode::AbsoluteX, lsr),

        OpCode::new(0x54, "MVN", 7, AddressingMode::BlockMove, mvn),
        OpCode::new(0x44, "MVP", 7, AddressingMode::BlockMove, mvp),

        OpCode::new(0xea, "NOP", 2, AddressingMode::Implied, nop),

        OpCode::new(0x09, "ORA", 2, AddressingMode::Immediate, ora),
        OpCode::new(0x05, "ORA", 3, AddressingMode::Direct, ora),
        OpCode::new(0x15, "ORA", 4, AddressingMode::DirectX, ora),
        OpCode::new(0x0d, "ORA", 4, AddressingMode::Absolute, ora),
        OpCode::new(0x1d, "ORA", 4, AddressingMode::AbsoluteX, ora),
        OpCode::new(0x19, "ORA", 4, AddressingMode::AbsoluteY, ora),
        OpCode::new(0x01, "ORA", 6, AddressingMode::IndirectX, ora),
        OpCode::new(0x11, "ORA", 5, AddressingMode::IndirectY, ora),
        OpCode::new(0x0f, "ORA", 5, AddressingMode::AbsoluteLong, ora),
        OpCode::new(0x1f, "ORA", 5, AddressingMode::AbsoluteLongX, ora),
        OpCode::new(0x12, "ORA", 5, AddressingMode::Indirect, ora),
        OpCode::new(0x07, "ORA", 6, AddressingMode::IndirectLong, ora),
        OpCode::new(0x17, "ORA", 6, AddressingMode::IndirectLongY, ora),
        OpCode::new(0x03, "ORA", 4, AddressingMode::StackRelative, ora),
        OpCode::new(0x13, "ORA", 7, AddressingMode::StackRelativeIndirectY, ora),

        OpCode::new(0xf4, "PEA", 5, AddressingMode::Immediate, pea),
        OpCode::new(0xd4, "PEI", 6, AddressingMode::StackPEI, pei),
        OpCode::new(0x62, "PER", 6, AddressingMode::Implied, per),
        OpCode::new(0x48, "PHA", 3, AddressingMode::Implied, pha),
        OpCode::new(0x8b, "PHB", 3, AddressingMode::Implied, phb),
        OpCode::new(0x0b, "PHD", 4, AddressingMode::Implied, phd),
        OpCode::new(0x4b, "PHK", 3, AddressingMode::Implied, phk),
        OpCode::new(0x08, "PHP", 3, AddressingMode::Implied, php),
        OpCode::new(0xda, "PHX", 3, AddressingMode::Implied, phx),
        OpCode::new(0x5a, "PHY", 3, AddressingMode::Implied, phy),

        OpCode::new(0x68, "PLA", 4, AddressingMode::Implied, pla),
        OpCode::new(0xab, "PLB", 4, AddressingMode::Implied, plb),
        OpCode::new(0x2b, "PLD", 5, AddressingMode::Implied, pld),
        OpCode::new(0x28, "PLP", 4, AddressingMode::Implied, plp),
        OpCode::new(0xfa, "PLX", 4, AddressingMode::Implied, plx),
        OpCode::new(0x7a, "PLY", 4, AddressingMode::Implied, ply),

        OpCode::new(0xc2, "REP", 3, AddressingMode::Immediate, rep),

        OpCode::new(0x2a, "ROL", 2, AddressingMode::Implied, rol_a),
        OpCode::new(0x26, "ROL", 5, AddressingMode::Direct, rol),
        OpCode::new(0x36, "ROL", 6, AddressingMode::DirectX, rol),
        OpCode::new(0x2e, "ROL", 6, AddressingMode::Absolute, rol),
        OpCode::new(0x3e, "ROL", 7, AddressingMode::AbsoluteX, rol),

        OpCode::new(0x6a, "ROR", 2, AddressingMode::Implied, ror_a),
        OpCode::new(0x66, "ROR", 5, AddressingMode::Direct, ror),
        OpCode::new(0x76, "ROR", 6, AddressingMode::DirectX, ror),
        OpCode::new(0x6e, "ROR", 6, AddressingMode::Absolute, ror),
        OpCode::new(0x7e, "ROR", 7, AddressingMode::AbsoluteX, ror),

        OpCode::new(0x40, "RTI", 6, AddressingMode::Implied, rti),
        OpCode::new(0x6b, "RTL", 6, AddressingMode::Implied, rtl),
        OpCode::new(0x60, "RTS", 6, AddressingMode::Implied, rts),

        OpCode::new(0xe9, "SBC", 2, AddressingMode::Immediate, sbc),
        OpCode::new(0xe5, "SBC", 3, AddressingMode::Direct, sbc),
        OpCode::new(0xf5, "SBC", 4, AddressingMode::DirectX, sbc),
        OpCode::new(0xed, "SBC", 4, AddressingMode::Absolute, sbc),
        OpCode::new(0xfd, "SBC", 4, AddressingMode::AbsoluteX, sbc),
        OpCode::new(0xf9, "SBC", 4, AddressingMode::AbsoluteY, sbc),
        OpCode::new(0xe1, "SBC", 6, AddressingMode::IndirectX, sbc),
        OpCode::new(0xf1, "SBC", 5, AddressingMode::IndirectY, sbc),
        OpCode::new(0xef, "SBC", 5, AddressingMode::AbsoluteLong, sbc),
        OpCode::new(0xff, "SBC", 5, AddressingMode::AbsoluteLongX, sbc),
        OpCode::new(0xf2, "SBC", 5, AddressingMode::Indirect, sbc),
        OpCode::new(0xe7, "SBC", 6, AddressingMode::IndirectLong, sbc),
        OpCode::new(0xf7, "SBC", 6, AddressingMode::IndirectLongY, sbc),
        OpCode::new(0xe3, "SBC", 4, AddressingMode::StackRelative, sbc),
        OpCode::new(0xf3, "SBC", 7, AddressingMode::StackRelativeIndirectY, sbc),

        OpCode::new(0x38, "SEC", 2, AddressingMode::Implied, sec),
        OpCode::new(0xf8, "SED", 2, AddressingMode::Implied, sed),
        OpCode::new(0x78, "SEI", 2, AddressingMode::Implied, sei),
        OpCode::new(0xe2, "SEP", 3, AddressingMode::Immediate, sep),

        OpCode::new(0x85, "STA", 3, AddressingMode::Direct, sta),
        OpCode::new(0x95, "STA", 4, AddressingMode::DirectX, sta),
        OpCode::new(0x8d, "STA", 4, AddressingMode::Absolute, sta),
        OpCode::new(0x9d, "STA", 5, AddressingMode::AbsoluteX, sta),
        OpCode::new(0x99, "STA", 5, AddressingMode::AbsoluteY, sta),
        OpCode::new(0x81, "STA", 6, AddressingMode::IndirectX, sta),
        OpCode::new(0x91, "STA", 6, AddressingMode::IndirectY, sta),
        OpCode::new(0x8f, "STA", 5, AddressingMode::AbsoluteLong, sta),
        OpCode::new(0x9f, "STA", 5, AddressingMode::AbsoluteLongX, sta),
        OpCode::new(0x92, "STA", 5, AddressingMode::Indirect, sta),
        OpCode::new(0x87, "STA", 6, AddressingMode::IndirectLong, sta),
        OpCode::new(0x97, "STA", 6, AddressingMode::IndirectLongY, sta),
        OpCode::new(0x83, "STA", 4, AddressingMode::StackRelative, sta),
        OpCode::new(0x93, "STA", 7, AddressingMode::StackRelativeIndirectY, sta),

        OpCode::new(0x86, "STX", 3, AddressingMode::Direct, stx),
        OpCode::new(0x96, "STX", 4, AddressingMode::DirectY, stx),
        OpCode::new(0x8e, "STX", 4, AddressingMode::Absolute, stx),

        OpCode::new(0x84, "STY", 3, AddressingMode::Direct, sty),
        OpCode::new(0x94, "STY", 4, AddressingMode::DirectX, sty),
        OpCode::new(0x8c, "STY", 4, AddressingMode::Absolute, sty),

        OpCode::new(0x9c, "STZ", 4, AddressingMode::Absolute, stz),
        OpCode::new(0x9e, "STZ", 5, AddressingMode::AbsoluteX, stz),
        OpCode::new(0x64, "STZ", 3, AddressingMode::Direct, stz),
        OpCode::new(0x74, "STZ", 4, AddressingMode::DirectX, stz),

        OpCode::new(0xdb, "STP", 3, AddressingMode::Implied, stp),

        OpCode::new(0xaa, "TAX", 2, AddressingMode::Implied, tax),
        OpCode::new(0xa8, "TAY", 2, AddressingMode::Implied, tay),
        OpCode::new(0x5b, "TCD", 2, AddressingMode::Implied, tcd),
        OpCode::new(0x1b, "TCS", 2, AddressingMode::Implied, tcs),
        OpCode::new(0x7b, "TDC", 2, AddressingMode::Implied, tdc),
        OpCode::new(0x3b, "TSC", 2, AddressingMode::Implied, tsc),
        OpCode::new(0xba, "TSX", 2, AddressingMode::Implied, tsx),
        OpCode::new(0x8a, "TXA", 2, AddressingMode::Implied, txa),
        OpCode::new(0x9a, "TXS", 2, AddressingMode::Implied, txs),
        OpCode::new(0x98, "TYA", 2, AddressingMode::Implied, tya),
        OpCode::new(0x9b, "TXY", 2, AddressingMode::Implied, txy),
        OpCode::new(0xbb, "TYX", 2, AddressingMode::Implied, tyx),

        OpCode::new(0x1c, "TRB", 4, AddressingMode::Absolute, trb),
        OpCode::new(0x14, "TRB", 3, AddressingMode::Direct, trb),
        OpCode::new(0x0c, "TSB", 4, AddressingMode::Absolute, tsb),
        OpCode::new(0x04, "TSB", 3, AddressingMode::Direct, tsb),

        OpCode::new(0xcb, "WAI", 3, AddressingMode::Implied, wai), // maybe more than 3
        OpCode::new(0xff, "WDM", 2, AddressingMode::Immediate, wdm),

        OpCode::new(0xeb, "XBA", 3, AddressingMode::Implied, xba),
        OpCode::new(0xfb, "XCE", 2, AddressingMode::Implied, xce),
    ];

    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        let mut map = HashMap::new();
        for op in &*CPU_OP_CODES {
            map.insert(op.code, op);
        }
        map
    };
}
