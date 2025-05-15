use cpu::Cpu;

use crate::bus::Bus;

mod addressing;
pub mod cpu;
mod functions;
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
    instruction_set: [OpCode<B>; 36],
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
const fn opcode_table<B: Bus>() -> [OpCode<B>; 36] {
    use addressing::{AddressingMode::*, Source::*};
    [
        OpCode::new(Meta::new(0x00, "NOP", Implied), Spc700::nop),
        OpCode::new(Meta::new(0x01, "TCALL", Implied), Spc700::tcall::<1>),
        OpCode::new(Meta::new(0x02, "SET1", DirectPage), Spc700::set1::<0>),
        OpCode::new(Meta::new(0x03, "BBS", DirectPage), Spc700::bbs::<0>),
        OpCode::new(Meta::new(0x04, "OR", DirectPage), Spc700::or_a),
        OpCode::new(Meta::new(0x05, "OR", Absolute), Spc700::or_a),
        OpCode::new(Meta::new(0x06, "OR", IndirectX), Spc700::or_a),
        OpCode::new(Meta::new(0x07, "OR", XIndirect), Spc700::or_a),
        OpCode::new(Meta::new(0x08, "OR", Immediate), Spc700::or_a),
        OpCode::new(Meta::new(0x09, "OR", DirectPage), Spc700::or::<{ DirectPage }>),
        OpCode::new(Meta::new(0x0A, "OR1", AbsoluteBooleanBit), Spc700::or1),
        OpCode::new(Meta::new(0x0B, "ASL", DirectPage), Spc700::asl),
        OpCode::new(Meta::new(0x0C, "ASL", Absolute), Spc700::asl),
        OpCode::new(Meta::new(0x0D, "PUSH", Implied), Spc700::push::<{ PSW }>),
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
        OpCode::new(Meta::new(0x1E, "CMP", Absolute), Spc700::cmp_reg::<{ X }>),
        OpCode::new(Meta::new(0x1F, "JMP", AbsoluteX), Spc700::jmp),
        OpCode::new(Meta::new(0x20, "CLRP", Implied), Spc700::clrp),
        OpCode::new(Meta::new(0x21, "TCALL", Implied), Spc700::tcall::<2>),
        OpCode::new(Meta::new(0x22, "SET1", DirectPage), Spc700::set1::<1>),
        OpCode::new(Meta::new(0x23, "BBS", DirectPage), Spc700::bbs::<1>),
    ]
}
