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
    instruction_set: [OpCode<B>; 128],
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
const fn opcode_table<B: Bus>() -> [OpCode<B>; 128] {
    use addressing::AddressingMode::*;
    [
        OpCode::new(Meta::new(0x00, "NOP", Implied), Spc700::nop),
        OpCode::new(Meta::new(0x01, "TCALL", Implied), Spc700::tcall::<0>),
        OpCode::new(Meta::new(0x02, "SET1", DirectPage), Spc700::set1::<0>),
        OpCode::new(Meta::new(0x03, "BBS", DirectPage), Spc700::bbs::<0>),
        OpCode::new(Meta::new(0x04, "OR", DirectPage), Spc700::or_a),
        OpCode::new(Meta::new(0x05, "OR", Absolute), Spc700::or_a),
        OpCode::new(Meta::new(0x06, "OR", IndirectX), Spc700::or_a),
        OpCode::new(Meta::new(0x07, "OR", XIndirect), Spc700::or_a),
        OpCode::new(Meta::new(0x08, "OR", Immediate), Spc700::or_a),
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
        OpCode::new(Meta::new(0x14, "OR", DirectX), Spc700::or_a),
        OpCode::new(Meta::new(0x15, "OR", AbsoluteX), Spc700::or_a),
        OpCode::new(Meta::new(0x16, "OR", AbsoluteY), Spc700::or_a),
        OpCode::new(Meta::new(0x17, "OR", DirectPageIndirectY), Spc700::or_a),
        OpCode::new(Meta::new(0x18, "OR", Immediate), Spc700::or::<{ DirectPage }>),
        OpCode::new(Meta::new(0x19, "OR", IndirectY), Spc700::or::<{ IndirectX }>),
        OpCode::new(Meta::new(0x1A, "DECW", DirectPage), Spc700::decw),
        OpCode::new(Meta::new(0x1B, "ASL", DirectX), Spc700::asl),
        OpCode::new(Meta::new(0x1C, "ASL", Implied), Spc700::asl_a),
        OpCode::new(Meta::new(0x1D, "DEC", Implied), Spc700::dec_x),
        OpCode::new(Meta::new(0x1E, "CMP", Absolute), Spc700::cmp::<{ X }>),
        OpCode::new(Meta::new(0x1F, "JMP", AbsoluteX), Spc700::jmp),
        OpCode::new(Meta::new(0x20, "CLRP", Implied), Spc700::clrp),
        OpCode::new(Meta::new(0x21, "TCALL", Implied), Spc700::tcall::<2>),
        OpCode::new(Meta::new(0x22, "SET1", DirectPage), Spc700::set1::<1>),
        OpCode::new(Meta::new(0x23, "BBS", DirectPage), Spc700::bbs::<1>),
        OpCode::new(Meta::new(0x24, "AND", DirectPage), Spc700::and_a),
        OpCode::new(Meta::new(0x25, "AND", Absolute), Spc700::and_a),
        OpCode::new(Meta::new(0x26, "AND", IndirectX), Spc700::and_a),
        OpCode::new(Meta::new(0x27, "AND", XIndirect), Spc700::and_a),
        OpCode::new(Meta::new(0x28, "AND", Immediate), Spc700::and_a),
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
        OpCode::new(Meta::new(0x34, "AND", DirectX), Spc700::and_a),
        OpCode::new(Meta::new(0x35, "AND", AbsoluteX), Spc700::and_a),
        OpCode::new(Meta::new(0x36, "AND", AbsoluteY), Spc700::and_a),
        OpCode::new(Meta::new(0x37, "AND", DirectPageIndirectY), Spc700::and_a),
        OpCode::new(Meta::new(0x38, "AND", Immediate), Spc700::and::<{ DirectPage }>),
        OpCode::new(Meta::new(0x39, "AND", IndirectY), Spc700::and::<{ IndirectX }>),
        OpCode::new(Meta::new(0x3A, "INCW", DirectPage), Spc700::incw),
        OpCode::new(Meta::new(0x3B, "ROL", DirectX), Spc700::rol),
        OpCode::new(Meta::new(0x3C, "ROL", Implied), Spc700::rol_a),
        OpCode::new(Meta::new(0x3D, "INC", Implied), Spc700::inc_x),
        OpCode::new(Meta::new(0x3E, "CMP", DirectPage), Spc700::cmp::<{ X }>),
        OpCode::new(Meta::new(0x3F, "CALL", Absolute), Spc700::call),
        OpCode::new(Meta::new(0x40, "SETP", Implied), Spc700::setp),
        OpCode::new(Meta::new(0x41, "TCALL", Implied), Spc700::tcall::<4>),
        OpCode::new(Meta::new(0x42, "SET1", DirectPage), Spc700::set1::<2>),
        OpCode::new(Meta::new(0x43, "BBS", DirectPage), Spc700::bbs::<2>),
        OpCode::new(Meta::new(0x44, "EOR", DirectPage), Spc700::eor_a),
        OpCode::new(Meta::new(0x45, "EOR", Absolute), Spc700::eor_a),
        OpCode::new(Meta::new(0x46, "EOR", IndirectX), Spc700::eor_a),
        OpCode::new(Meta::new(0x47, "EOR", XIndirect), Spc700::eor_a),
        OpCode::new(Meta::new(0x48, "EOR", Immediate), Spc700::eor_a),
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
        OpCode::new(Meta::new(0x54, "EOR", DirectX), Spc700::eor_a),
        OpCode::new(Meta::new(0x55, "EOR", AbsoluteX), Spc700::eor_a),
        OpCode::new(Meta::new(0x56, "EOR", AbsoluteY), Spc700::eor_a),
        OpCode::new(Meta::new(0x57, "EOR", DirectPageIndirectY), Spc700::eor_a),
        OpCode::new(Meta::new(0x58, "EOR", Immediate), Spc700::eor::<{ DirectPage }>),
        OpCode::new(Meta::new(0x59, "EOR", IndirectY), Spc700::eor::<{ IndirectX }>),
        OpCode::new(Meta::new(0x5A, "CMPW", DirectPage), Spc700::cmpw),
        OpCode::new(Meta::new(0x5B, "LSR", DirectX), Spc700::lsr),
        OpCode::new(Meta::new(0x5C, "LSR", Implied), Spc700::lsr_a),
        OpCode::new(Meta::new(0x5D, "MOV", Implied), Spc700::mov),
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
        OpCode::new(Meta::new(0x7C, "ROR", Implied), Spc700::ror_a),
        OpCode::new(Meta::new(0x7D, "MOV", Implied), Spc700::mov),
        OpCode::new(Meta::new(0x7E, "CMP", DirectPage), Spc700::cmp::<{ Y }>),
        OpCode::new(Meta::new(0x7F, "RETI", Implied), Spc700::reti),
    ]
}
