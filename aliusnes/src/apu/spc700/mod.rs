use cpu::Cpu;

use crate::bus::Bus;

mod addressing;
pub mod cpu;
mod instructions;

#[derive(Clone, Copy)]
pub struct Meta {
    pub code: u8,
    pub mnemonic: &'static str,
    pub mode: addressing::AddressingMode,
}

impl Meta {
    const fn new(code: u8, mnemonic: &'static str, mode: addressing::AddressingMode) -> Self {
        Self {
            code,
            mnemonic,
            mode,
        }
    }
}

pub struct OpCode<B: Bus> {
    pub meta: Meta,
    pub function: fn(&mut cpu::Cpu, &mut B, addressing::AddressingMode),
}

impl<B: Bus> OpCode<B> {
    const fn new(
        meta: Meta,
        function: fn(&mut cpu::Cpu, &mut B, addressing::AddressingMode),
    ) -> Self {
        OpCode { meta, function }
    }
}

pub struct Spc700<B: Bus> {
    pub cpu: cpu::Cpu,
    instruction_set: [OpCode<B>; 192],
}

impl<B: Bus> Spc700<B> {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            instruction_set: opcode_table(),
        }
    }

    pub fn step(&mut self, bus: &mut B) {
        let op = self.cpu.get_imm::<B>(bus);
        let opcode = &self.instruction_set[op as usize];

        let instr = opcode.function;
        instr(&mut self.cpu, bus, opcode.meta.mode);
    }
}

#[rustfmt::skip]
const fn opcode_table<B: Bus>() -> [OpCode<B>; 192] {
    use addressing::AddressingMode::*;
    [
        OpCode::new(Meta::new(0x00, "NOP", Implied), Spc700::nop),
        OpCode::new(Meta::new(0x01, "TCALL", Implied), Spc700::tcall::<0>),
        OpCode::new(Meta::new(0x02, "SET1", DirectPage), Spc700::set1::<0>),
        OpCode::new(Meta::new(0x03, "BBS", DirectPage), Spc700::bbs::<0>),
        OpCode::new(Meta::new(0x04, "OR", DirectPage), Spc700::or::<{ Accumulator }>),
        OpCode::new(Meta::new(0x05, "OR", Absolute), Spc700::or::<{ Accumulator }>),
        OpCode::new(Meta::new(0x06, "OR", IndirectX), Spc700::or::<{ Accumulator }>),
        OpCode::new(Meta::new(0x07, "OR", XIndirect), Spc700::or::<{ Accumulator }>),
        OpCode::new(Meta::new(0x08, "OR", Immediate), Spc700::or::<{ Accumulator }>),
        OpCode::new(Meta::new(0x09, "OR", DirectPage), Spc700::or::<{ DirectPage }>),
        OpCode::new(Meta::new(0x0A, "OR1", AbsoluteBooleanBit), Spc700::or1::<false>),
        OpCode::new(Meta::new(0x0B, "ASL", DirectPage), Spc700::asl),
        OpCode::new(Meta::new(0x0C, "ASL", Absolute), Spc700::asl),
        OpCode::new(Meta::new(0x0D, "PUSH", Psw), Spc700::push),
        OpCode::new(Meta::new(0x0E, "TSET1", Absolute), Spc700::tset1),
        OpCode::new(Meta::new(0x0F, "BRK", Implied), Spc700::brk),
        OpCode::new(Meta::new(0x10, "BPL", Implied), Spc700::bpl),
        OpCode::new(Meta::new(0x11, "TCALL", Implied), Spc700::tcall::<1>),
        OpCode::new(Meta::new(0x12, "CLR1", DirectPage), Spc700::clr1::<0>),
        OpCode::new(Meta::new(0x13, "BBC", DirectPage), Spc700::bbc::<0>),
        OpCode::new(Meta::new(0x14, "OR", DirectX), Spc700::or::<{ Accumulator }>),
        OpCode::new(Meta::new(0x15, "OR", AbsoluteX), Spc700::or::<{ Accumulator }>),
        OpCode::new(Meta::new(0x16, "OR", AbsoluteY), Spc700::or::<{ Accumulator }>),
        OpCode::new(Meta::new(0x17, "OR", DirectPageIndirectY), Spc700::or::<{ Accumulator }>),
        OpCode::new(Meta::new(0x18, "OR", Immediate), Spc700::or::<{ DirectPage }>),
        OpCode::new(Meta::new(0x19, "OR", IndirectY), Spc700::or::<{ IndirectX }>),
        OpCode::new(Meta::new(0x1A, "DECW", DirectPage), Spc700::decw),
        OpCode::new(Meta::new(0x1B, "ASL", DirectX), Spc700::asl),
        OpCode::new(Meta::new(0x1C, "ASL", Accumulator), Spc700::asl),
        OpCode::new(Meta::new(0x1D, "DEC", X), Spc700::dec),
        OpCode::new(Meta::new(0x1E, "CMP", Absolute), Spc700::cmp::<{ X }>),
        OpCode::new(Meta::new(0x1F, "JMP", AbsoluteX), Spc700::jmp),
        OpCode::new(Meta::new(0x20, "CLRP", Implied), Spc700::clrp),
        OpCode::new(Meta::new(0x21, "TCALL", Implied), Spc700::tcall::<2>),
        OpCode::new(Meta::new(0x22, "SET1", DirectPage), Spc700::set1::<1>),
        OpCode::new(Meta::new(0x23, "BBS", DirectPage), Spc700::bbs::<1>),
        OpCode::new(Meta::new(0x24, "AND", DirectPage), Spc700::and::<{ Accumulator }>),
        OpCode::new(Meta::new(0x25, "AND", Absolute), Spc700::and::<{ Accumulator }>),
        OpCode::new(Meta::new(0x26, "AND", IndirectX), Spc700::and::<{ Accumulator }>),
        OpCode::new(Meta::new(0x27, "AND", XIndirect), Spc700::and::<{ Accumulator }>),
        OpCode::new(Meta::new(0x28, "AND", Immediate), Spc700::and::<{ Accumulator }>),
        OpCode::new(Meta::new(0x29, "AND", DirectPage), Spc700::and::<{ DirectPage }>),
        OpCode::new(Meta::new(0x2A, "OR1", AbsoluteBooleanBit), Spc700::or1::<true>),
        OpCode::new(Meta::new(0x2B, "ROL", DirectPage), Spc700::rol),
        OpCode::new(Meta::new(0x2C, "ROL", Absolute), Spc700::rol),
        OpCode::new(Meta::new(0x2D, "PUSH", Accumulator), Spc700::push),
        OpCode::new(Meta::new(0x2E, "CBNE", DirectPage), Spc700::cbne),
        OpCode::new(Meta::new(0x2F, "BRA", Implied), Spc700::bra),
        OpCode::new(Meta::new(0x30, "BMI", Implied), Spc700::bmi),
        OpCode::new(Meta::new(0x31, "TCALL", Implied), Spc700::tcall::<3>),
        OpCode::new(Meta::new(0x32, "CLR1", DirectPage), Spc700::clr1::<1>),
        OpCode::new(Meta::new(0x33, "BBC", DirectPage), Spc700::bbc::<1>),
        OpCode::new(Meta::new(0x34, "AND", DirectX), Spc700::and::<{ Accumulator}>),
        OpCode::new(Meta::new(0x35, "AND", AbsoluteX), Spc700::and::<{ Accumulator}>),
        OpCode::new(Meta::new(0x36, "AND", AbsoluteY), Spc700::and::<{ Accumulator}>),
        OpCode::new(Meta::new(0x37, "AND", DirectPageIndirectY), Spc700::and::<{ Accumulator}>),
        OpCode::new(Meta::new(0x38, "AND", Immediate), Spc700::and::<{ DirectPage }>),
        OpCode::new(Meta::new(0x39, "AND", IndirectY), Spc700::and::<{ IndirectX }>),
        OpCode::new(Meta::new(0x3A, "INCW", DirectPage), Spc700::incw),
        OpCode::new(Meta::new(0x3B, "ROL", DirectX), Spc700::rol),
        OpCode::new(Meta::new(0x3C, "ROL", Accumulator), Spc700::rol),
        OpCode::new(Meta::new(0x3D, "INC", X), Spc700::inc),
        OpCode::new(Meta::new(0x3E, "CMP", DirectPage), Spc700::cmp::<{ X }>),
        OpCode::new(Meta::new(0x3F, "CALL", Absolute), Spc700::call),
        OpCode::new(Meta::new(0x40, "SETP", Implied), Spc700::setp),
        OpCode::new(Meta::new(0x41, "TCALL", Implied), Spc700::tcall::<4>),
        OpCode::new(Meta::new(0x42, "SET1", DirectPage), Spc700::set1::<2>),
        OpCode::new(Meta::new(0x43, "BBS", DirectPage), Spc700::bbs::<2>),
        OpCode::new(Meta::new(0x44, "EOR", DirectPage), Spc700::eor::<{ Accumulator }>),
        OpCode::new(Meta::new(0x45, "EOR", Absolute), Spc700::eor::<{ Accumulator }>),
        OpCode::new(Meta::new(0x46, "EOR", IndirectX), Spc700::eor::<{ Accumulator }>),
        OpCode::new(Meta::new(0x47, "EOR", XIndirect), Spc700::eor::<{ Accumulator }>),
        OpCode::new(Meta::new(0x48, "EOR", Immediate), Spc700::eor::<{ Accumulator }>),
        OpCode::new(Meta::new(0x49, "EOR", DirectPage), Spc700::eor::<{ DirectPage }>),
        OpCode::new(Meta::new(0x4A, "AND1", AbsoluteBooleanBit), Spc700::and1::<false>),
        OpCode::new(Meta::new(0x4B, "LSR", DirectPage), Spc700::lsr),
        OpCode::new(Meta::new(0x4C, "LSR", Absolute), Spc700::lsr),
        OpCode::new(Meta::new(0x4D, "PUSH", X), Spc700::push),
        OpCode::new(Meta::new(0x4E, "TCLR1", Absolute), Spc700::tclr1),
        OpCode::new(Meta::new(0x4F, "PCALL", Implied), Spc700::pcall),
        OpCode::new(Meta::new(0x50, "BVC", Implied), Spc700::bvc),
        OpCode::new(Meta::new(0x51, "TCALL", Implied), Spc700::tcall::<5>),
        OpCode::new(Meta::new(0x52, "CLR1", DirectPage), Spc700::clr1::<2>),
        OpCode::new(Meta::new(0x53, "BBC", DirectPage), Spc700::bbc::<2>),
        OpCode::new(Meta::new(0x54, "EOR", DirectX), Spc700::eor::<{ Accumulator }>),
        OpCode::new(Meta::new(0x55, "EOR", AbsoluteX), Spc700::eor::<{ Accumulator }>),
        OpCode::new(Meta::new(0x56, "EOR", AbsoluteY), Spc700::eor::<{ Accumulator }>),
        OpCode::new(Meta::new(0x57, "EOR", DirectPageIndirectY), Spc700::eor::<{ Accumulator }>),
        OpCode::new(Meta::new(0x58, "EOR", Immediate), Spc700::eor::<{ DirectPage }>),
        OpCode::new(Meta::new(0x59, "EOR", IndirectY), Spc700::eor::<{ IndirectX }>),
        OpCode::new(Meta::new(0x5A, "CMPW", DirectPage), Spc700::cmpw),
        OpCode::new(Meta::new(0x5B, "LSR", DirectX), Spc700::lsr),
        OpCode::new(Meta::new(0x5C, "LSR", Accumulator), Spc700::lsr),
        OpCode::new(Meta::new(0x5D, "MOV", Accumulator), Spc700::mov::<{ X }>),
        OpCode::new(Meta::new(0x5E, "CMP", Absolute), Spc700::cmp::<{ Y }>),
        OpCode::new(Meta::new(0x5F, "JMP", Absolute), Spc700::jmp),
        OpCode::new(Meta::new(0x60, "CLRC", Implied), Spc700::clrc),
        OpCode::new(Meta::new(0x61, "TCALL", Implied), Spc700::tcall::<6>),
        OpCode::new(Meta::new(0x62, "SET1", DirectPage), Spc700::set1::<3>),
        OpCode::new(Meta::new(0x63, "BBS", DirectPage), Spc700::bbs::<3>),
        OpCode::new(Meta::new(0x64, "CMP", DirectPage), Spc700::cmp::<{ Accumulator }>),
        OpCode::new(Meta::new(0x65, "CMP", Absolute), Spc700::cmp::<{ Accumulator }>),
        OpCode::new(Meta::new(0x66, "CMP", IndirectX), Spc700::cmp::<{ Accumulator }>),
        OpCode::new(Meta::new(0x67, "CMP", XIndirect), Spc700::cmp::<{ Accumulator }>),
        OpCode::new(Meta::new(0x68, "CMP", Immediate), Spc700::cmp::<{ Accumulator }>),
        OpCode::new(Meta::new(0x69, "CMP", DirectPage), Spc700::cmp::<{ DirectPage }>),
        OpCode::new(Meta::new(0x6A, "AND1", AbsoluteBooleanBit), Spc700::and1::<true>),
        OpCode::new(Meta::new(0x6B, "ROR", DirectPage), Spc700::ror),
        OpCode::new(Meta::new(0x6C, "ROR", Absolute), Spc700::ror),
        OpCode::new(Meta::new(0x6D, "PUSH", Y), Spc700::push),
        OpCode::new(Meta::new(0x6E, "DBNZ", DirectPage), Spc700::dbnz::<false>),
        OpCode::new(Meta::new(0x6F, "RET", Implied), Spc700::ret),
        OpCode::new(Meta::new(0x70, "BVS", Implied), Spc700::bvs),
        OpCode::new(Meta::new(0x71, "TCALL", Implied), Spc700::tcall::<7>),
        OpCode::new(Meta::new(0x72, "CLR1", DirectPage), Spc700::clr1::<3>),
        OpCode::new(Meta::new(0x73, "BBC", DirectPage), Spc700::bbc::<3>),
        OpCode::new(Meta::new(0x74, "CMP", DirectX), Spc700::cmp::<{ Accumulator }>),
        OpCode::new(Meta::new(0x75, "CMP", AbsoluteX), Spc700::cmp::<{ Accumulator }>),
        OpCode::new(Meta::new(0x76, "CMP", AbsoluteY), Spc700::cmp::<{ Accumulator }>),
        OpCode::new(Meta::new(0x77, "CMP", DirectPageIndirectY), Spc700::cmp::<{ Accumulator }>),
        OpCode::new(Meta::new(0x78, "CMP", Immediate), Spc700::cmp::<{ DirectPage }>),
        OpCode::new(Meta::new(0x79, "CMP", IndirectY), Spc700::cmp::<{ IndirectX }>),
        OpCode::new(Meta::new(0x7A, "ADDW", DirectPage), Spc700::addw),
        OpCode::new(Meta::new(0x7B, "ROR", DirectX), Spc700::ror),
        OpCode::new(Meta::new(0x7C, "ROR", Accumulator), Spc700::ror),
        OpCode::new(Meta::new(0x7D, "MOV", X), Spc700::mov::<{ Accumulator }>),
        OpCode::new(Meta::new(0x7E, "CMP", DirectPage), Spc700::cmp::<{ Y }>),
        OpCode::new(Meta::new(0x7F, "RETI", Implied), Spc700::reti),
        OpCode::new(Meta::new(0x80, "SETC", Implied), Spc700::setc),
        OpCode::new(Meta::new(0x81, "TCALL", Implied), Spc700::tcall::<8>),
        OpCode::new(Meta::new(0x82, "SET1", DirectPage), Spc700::set1::<4>),
        OpCode::new(Meta::new(0x83, "BBS", DirectPage), Spc700::bbs::<4>),
        OpCode::new(Meta::new(0x84, "ADC", DirectPage), Spc700::adc::<{ Accumulator }>),
        OpCode::new(Meta::new(0x85, "ADC", Absolute), Spc700::adc::<{ Accumulator }>),
        OpCode::new(Meta::new(0x86, "ADC", IndirectX), Spc700::adc::<{ Accumulator }>),
        OpCode::new(Meta::new(0x87, "ADC", XIndirect), Spc700::adc::<{ Accumulator }>),
        OpCode::new(Meta::new(0x88, "ADC", Immediate), Spc700::adc::<{ Accumulator }>),
        OpCode::new(Meta::new(0x89, "ADC", DirectPage), Spc700::adc::<{ DirectPage }>),
        OpCode::new(Meta::new(0x8A, "EOR1", AbsoluteBooleanBit), Spc700::eor1),
        OpCode::new(Meta::new(0x8B, "DEC", DirectPage), Spc700::dec),
        OpCode::new(Meta::new(0x8C, "DEC", Absolute), Spc700::dec),
        OpCode::new(Meta::new(0x8D, "MOV", Immediate), Spc700::mov::<{ Y }>),
        OpCode::new(Meta::new(0x8E, "POP", Psw), Spc700::pop),
        OpCode::new(Meta::new(0x8F, "MOV", Immediate), Spc700::mov::<{ DirectPage }>),
        OpCode::new(Meta::new(0x90, "BCC", Implied), Spc700::bcc),
        OpCode::new(Meta::new(0x91, "TCALL", Implied), Spc700::tcall::<9>),
        OpCode::new(Meta::new(0x92, "CLR1", DirectPage), Spc700::clr1::<4>),
        OpCode::new(Meta::new(0x93, "BBC", DirectPage), Spc700::bbc::<4>),
        OpCode::new(Meta::new(0x94, "ADC", DirectX), Spc700::adc::<{ Accumulator }>),
        OpCode::new(Meta::new(0x95, "ADC", AbsoluteX), Spc700::adc::<{ Accumulator }>),
        OpCode::new(Meta::new(0x96, "ADC", AbsoluteY), Spc700::adc::<{ Accumulator }>),
        OpCode::new(Meta::new(0x97, "ADC", DirectPageIndirectY), Spc700::adc::<{ Accumulator }>),
        OpCode::new(Meta::new(0x98, "ADC", Immediate), Spc700::adc::<{ DirectPage }>),
        OpCode::new(Meta::new(0x99, "ADC", IndirectY), Spc700::adc::<{ IndirectX }>),
        OpCode::new(Meta::new(0x9A, "SUBW", DirectPage), Spc700::subw),
        OpCode::new(Meta::new(0x9B, "DEC", DirectX), Spc700::dec),
        OpCode::new(Meta::new(0x9C, "DEC", Accumulator), Spc700::dec),
        OpCode::new(Meta::new(0x9D, "MOV", Sp), Spc700::mov::<{ X }>),
        OpCode::new(Meta::new(0x9E, "DIV", Implied), Spc700::div),
        OpCode::new(Meta::new(0x9F, "XCN", Implied), Spc700::xcn),
        OpCode::new(Meta::new(0xA0, "EI", Implied), Spc700::ei),
        OpCode::new(Meta::new(0xA1, "TCALL", Implied), Spc700::tcall::<10>),
        OpCode::new(Meta::new(0xA2, "SET1", DirectPage), Spc700::set1::<5>),
        OpCode::new(Meta::new(0xA3, "BBS", DirectPage), Spc700::bbs::<5>),
        OpCode::new(Meta::new(0xA4, "SBC", DirectPage), Spc700::sbc::<{ Accumulator }>),
        OpCode::new(Meta::new(0xA5, "SBC", Absolute), Spc700::sbc::<{ Accumulator }>),
        OpCode::new(Meta::new(0xA6, "SBC", IndirectX), Spc700::sbc::<{ Accumulator }>),
        OpCode::new(Meta::new(0xA7, "SBC", XIndirect), Spc700::sbc::<{ Accumulator }>),
        OpCode::new(Meta::new(0xA8, "SBC", Immediate), Spc700::sbc::<{ Accumulator }>),
        OpCode::new(Meta::new(0xA9, "SBC", DirectPage), Spc700::sbc::<{ DirectPage }>),
        OpCode::new(Meta::new(0xAA, "MOV1", AbsoluteBooleanBit), Spc700::mov1::<true>),
        OpCode::new(Meta::new(0xAB, "INC", DirectPage), Spc700::inc),
        OpCode::new(Meta::new(0xAC, "INC", Absolute), Spc700::inc),
        OpCode::new(Meta::new(0xAD, "CMP", Immediate), Spc700::cmp::<{ Y }>),
        OpCode::new(Meta::new(0xAE, "POP", Accumulator), Spc700::pop),
        OpCode::new(Meta::new(0xAF, "MOV", Accumulator), Spc700::mov::<{ DirectXPostIncrement }>),
        OpCode::new(Meta::new(0xB0, "BCS", Implied), Spc700::bcs),
        OpCode::new(Meta::new(0xB1, "TCALL", Implied), Spc700::tcall::<11>),
        OpCode::new(Meta::new(0xB2, "CLR1", DirectPage), Spc700::clr1::<5>),
        OpCode::new(Meta::new(0xB3, "BBC", DirectPage), Spc700::bbc::<5>),
        OpCode::new(Meta::new(0xB4, "SBC", DirectX), Spc700::sbc::<{ Accumulator }>),
        OpCode::new(Meta::new(0xB5, "SBC", AbsoluteX), Spc700::sbc::<{ Accumulator }>),
        OpCode::new(Meta::new(0xB6, "SBC", AbsoluteY), Spc700::sbc::<{ Accumulator }>),
        OpCode::new(Meta::new(0xB7, "SBC", DirectPageIndirectY), Spc700::sbc::<{ Accumulator }>),
        OpCode::new(Meta::new(0xB8, "SBC", Immediate), Spc700::sbc::<{ DirectPage }>),
        OpCode::new(Meta::new(0xB9, "SBC", IndirectY), Spc700::sbc::<{ IndirectX }>),
        OpCode::new(Meta::new(0xBA, "MOVW", DirectPage), Spc700::movw::<true>),
        OpCode::new(Meta::new(0xBB, "INC", DirectX), Spc700::inc),
        OpCode::new(Meta::new(0xBC, "INC", Accumulator), Spc700::inc),
        OpCode::new(Meta::new(0xBD, "MOV", X), Spc700::mov::<{ Sp }>),
        OpCode::new(Meta::new(0xBE, "DAS", Implied), Spc700::das),
        OpCode::new(Meta::new(0xBF, "MOV", DirectXPostIncrement), Spc700::mov::<{ Accumulator }>),
    ]
}
