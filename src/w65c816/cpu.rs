use opcodes;
use std::collections::HashMap;

bitflags! {
    pub struct CpuFlags: u8 {
        const negative = 0b10000000;
        const overflow = 0b01000000;
        const a_reg_size = 0b00100000;
        const index_regs_size = 0b00010000;
        const decimal = 0b00001000;
        const irq_disable = 0b00000100;
        const zero = 0b00000010;
        const carry = 0b00000001;
    }
}

pub struct cpu {
    pub register_a: u16,
    pub register_x: u16,
    pub register_y: u16,
    pub stack_pointer: u16,
    pub program_couter: u16,
    pub status: CpuFlags,
    pub dpr: u16,
    pub pbr: u8,
    pub dbr: u8,
    memory: [u8; 0xFFFF]
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
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

impl cpu {
    pub fn new() -> Self {
        cpu { 
            register_a: (), 
            register_x: (), 
            register_y: (), 
            stack_pointer: (), 
            status: (), 
            dpr: (), 
            pbr: (), 
            dbr: (),
            memory: [0; 0xFFFF]
        }
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u32 {
        match mode {
            AddressingMode::Implied => self.program_couter,

            AddressingMode::Immediate => self.program_couter,

            AddressingMode::Relative => self.program_couter,

            
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn set_register_a(&mut self, data: u16) {
        self.register_a = data;
    }

    fn set_register_x(&mut self, data: u16) {
        self.register_x = data;
    }
}