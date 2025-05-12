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
    instruction_set: [OpCode<B>; 10],
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

const fn opcode_table<B: Bus>() -> [OpCode<B>; 10] {
    use addressing::AddressingMode::*;
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
        OpCode::new(Meta::new(0x09, "OR", DirectPage), Spc700::or),
    ]
}
