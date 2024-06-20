use crate::bus::Bus;
use addressing::{Address, AddressingMode};
use cpu::Vectors;

pub mod addressing;
pub mod cpu;
mod functions;
mod instructions;
mod regsize;

type Instr<B> = fn(&mut cpu::Cpu, &mut B, AddressingMode);

#[derive(Clone, Copy)]
pub struct OpCode<F> {
    pub code: u8,
    pub mnemonic: &'static str,
    pub mode: AddressingMode,
    pub function: F,
}

impl<F> OpCode<F> {
    fn new<B: Bus>(code: u8, mnemonic: &'static str, mode: AddressingMode, function: F) -> Self
    where
        F: FnMut(&mut cpu::Cpu, &mut B, AddressingMode),
    {
        OpCode {
            code,
            mnemonic,
            mode,
            function,
        }
    }
}

pub struct W65C816<B: Bus> {
    pub cpu: cpu::Cpu,
    pub instruction_set: [OpCode<Instr<B>>; 256],
}

impl<B: Bus> W65C816<B> {
    pub fn new() -> Self {
        Self {
            cpu: cpu::Cpu::new(),
            instruction_set: Self::opcode_table(),
        }
    }

    pub fn step(&mut self, bus: &mut B) {
        if self.cpu.stopped {
            return;
        }
        if self.cpu.waiting_interrupt {
            if bus.fired_nmi() {
                self.cpu.waiting_interrupt = false;
                self.cpu.handle_interrupt(bus, Vectors::Nmi);
            } else if !self.cpu.status.irq_disable() && bus.fired_irq() {
                self.cpu.waiting_interrupt = false;
                self.cpu.handle_interrupt(bus, Vectors::Irq);
            } else {
                return;
            }
        }

        let op = self.cpu.get_imm::<u8, B>(bus);
        let opcode = self.instruction_set[op as usize];

        // log::trace!(
        //     "Instr {} A:{:#06x} X:{:#06x} Y:{:#06x}, PC:{:#06x}, SP:{:#06x}, P:{:#04x} {}",
        //     opcode.mnemonic,
        //     self.accumulator,
        //     self.index_x,
        //     self.index_y,
        //     (self.program_counter - 1),
        //     self.stack_pointer,
        //     self.status.0,
        //     format_status(&self.status)
        // );
        let instr = opcode.function;
        instr(&mut self.cpu, bus, opcode.mode);
    }

    pub fn peek_opcode(&self, bus: &B) -> OpCode<Instr<B>> {
        let op = bus
            .peek_at(Address::new(self.cpu.program_counter, self.cpu.pbr))
            .unwrap_or_default();
        self.instruction_set[op as usize]
    }

    pub fn opcode_table() -> [OpCode<Instr<B>>; 256] {
        [
            OpCode::new(0x00, "BRK", AddressingMode::Immediate, W65C816::brk),
            OpCode::new(0x01, "ORA", AddressingMode::IndirectX, W65C816::ora),
            OpCode::new(0x02, "COP", AddressingMode::Immediate, W65C816::cop),
            OpCode::new(0x03, "ORA", AddressingMode::StackRelative, W65C816::ora),
            OpCode::new(0x04, "TSB", AddressingMode::Direct, W65C816::tsb),
            OpCode::new(0x05, "ORA", AddressingMode::Direct, W65C816::ora),
            OpCode::new(0x06, "ASL", AddressingMode::Direct, W65C816::asl),
            OpCode::new(0x07, "ORA", AddressingMode::IndirectLong, W65C816::ora),
            OpCode::new(0x08, "PHP", AddressingMode::Implied, W65C816::php),
            OpCode::new(0x09, "ORA", AddressingMode::Immediate, W65C816::ora),
            OpCode::new(0x0a, "ASL", AddressingMode::Implied, W65C816::asl_a),
            OpCode::new(0x0b, "PHD", AddressingMode::Implied, W65C816::phd),
            OpCode::new(0x0c, "TSB", AddressingMode::Absolute, W65C816::tsb),
            OpCode::new(0x0d, "ORA", AddressingMode::Absolute, W65C816::ora),
            OpCode::new(0x0e, "ASL", AddressingMode::Absolute, W65C816::asl),
            OpCode::new(0x0f, "ORA", AddressingMode::AbsoluteLong, W65C816::ora),
            OpCode::new(0x10, "BPL", AddressingMode::Relative, W65C816::bpl),
            OpCode::new(0x11, "ORA", AddressingMode::IndirectY, W65C816::ora),
            OpCode::new(0x12, "ORA", AddressingMode::Indirect, W65C816::ora),
            OpCode::new(0x13, "ORA", AddressingMode::StackRelIndirectY, W65C816::ora),
            OpCode::new(0x14, "TRB", AddressingMode::Direct, W65C816::trb),
            OpCode::new(0x15, "ORA", AddressingMode::DirectX, W65C816::ora),
            OpCode::new(0x16, "ASL", AddressingMode::DirectX, W65C816::asl),
            OpCode::new(0x17, "ORA", AddressingMode::IndirectLongY, W65C816::ora),
            OpCode::new(0x18, "CLC", AddressingMode::Implied, W65C816::clc),
            OpCode::new(0x19, "ORA", AddressingMode::AbsoluteY, W65C816::ora),
            OpCode::new(0x1a, "INC", AddressingMode::Implied, W65C816::inc_a),
            OpCode::new(0x1b, "TCS", AddressingMode::Implied, W65C816::tcs),
            OpCode::new(0x1c, "TRB", AddressingMode::Absolute, W65C816::trb),
            OpCode::new(0x1d, "ORA", AddressingMode::AbsoluteX, W65C816::ora),
            OpCode::new(0x1e, "ASL", AddressingMode::AbsoluteX, W65C816::asl),
            OpCode::new(0x1f, "ORA", AddressingMode::AbsoluteLongX, W65C816::ora),
            OpCode::new(0x20, "JSR", AddressingMode::AbsoluteJMP, W65C816::jsr),
            OpCode::new(0x21, "AND", AddressingMode::IndirectX, W65C816::and),
            OpCode::new(0x22, "JSL", AddressingMode::AbsoluteLongJSL, W65C816::jsl),
            OpCode::new(0x23, "AND", AddressingMode::StackRelative, W65C816::and),
            OpCode::new(0x24, "BIT", AddressingMode::Direct, W65C816::bit),
            OpCode::new(0x25, "AND", AddressingMode::Direct, W65C816::and),
            OpCode::new(0x26, "ROL", AddressingMode::Direct, W65C816::rol),
            OpCode::new(0x27, "AND", AddressingMode::IndirectLong, W65C816::and),
            OpCode::new(0x28, "PLP", AddressingMode::Implied, W65C816::plp),
            OpCode::new(0x29, "AND", AddressingMode::Immediate, W65C816::and),
            OpCode::new(0x2a, "ROL", AddressingMode::Implied, W65C816::rol_a),
            OpCode::new(0x2b, "PLD", AddressingMode::Implied, W65C816::pld),
            OpCode::new(0x2c, "BIT", AddressingMode::Absolute, W65C816::bit),
            OpCode::new(0x2d, "AND", AddressingMode::Absolute, W65C816::and),
            OpCode::new(0x2e, "ROL", AddressingMode::Absolute, W65C816::rol),
            OpCode::new(0x2f, "AND", AddressingMode::AbsoluteLong, W65C816::and),
            OpCode::new(0x30, "BMI", AddressingMode::Relative, W65C816::bmi),
            OpCode::new(0x31, "AND", AddressingMode::IndirectY, W65C816::and),
            OpCode::new(0x32, "AND", AddressingMode::Indirect, W65C816::and),
            OpCode::new(0x33, "AND", AddressingMode::StackRelIndirectY, W65C816::and),
            OpCode::new(0x34, "BIT", AddressingMode::DirectX, W65C816::bit),
            OpCode::new(0x35, "AND", AddressingMode::DirectX, W65C816::and),
            OpCode::new(0x36, "ROL", AddressingMode::DirectX, W65C816::rol),
            OpCode::new(0x37, "AND", AddressingMode::IndirectLongY, W65C816::and),
            OpCode::new(0x38, "SEC", AddressingMode::Implied, W65C816::sec),
            OpCode::new(0x39, "AND", AddressingMode::AbsoluteY, W65C816::and),
            OpCode::new(0x3a, "DEC", AddressingMode::Implied, W65C816::dec_a),
            OpCode::new(0x3b, "TSC", AddressingMode::Implied, W65C816::tsc),
            OpCode::new(0x3c, "BIT", AddressingMode::AbsoluteX, W65C816::bit),
            OpCode::new(0x3d, "AND", AddressingMode::AbsoluteX, W65C816::and),
            OpCode::new(0x3e, "ROL", AddressingMode::AbsoluteX, W65C816::rol),
            OpCode::new(0x3f, "AND", AddressingMode::AbsoluteLongX, W65C816::and),
            OpCode::new(0x40, "RTI", AddressingMode::Implied, W65C816::rti),
            OpCode::new(0x41, "EOR", AddressingMode::IndirectX, W65C816::eor),
            OpCode::new(0x42, "WDM", AddressingMode::Immediate, W65C816::wdm),
            OpCode::new(0x43, "EOR", AddressingMode::StackRelative, W65C816::eor),
            OpCode::new(0x44, "MVP", AddressingMode::BlockMove, W65C816::mvp),
            OpCode::new(0x45, "EOR", AddressingMode::Direct, W65C816::eor),
            OpCode::new(0x46, "LSR", AddressingMode::Direct, W65C816::lsr),
            OpCode::new(0x47, "EOR", AddressingMode::IndirectLong, W65C816::eor),
            OpCode::new(0x48, "PHA", AddressingMode::Implied, W65C816::pha),
            OpCode::new(0x49, "EOR", AddressingMode::Immediate, W65C816::eor),
            OpCode::new(0x4a, "LSR", AddressingMode::Implied, W65C816::lsr_a),
            OpCode::new(0x4b, "PHK", AddressingMode::Implied, W65C816::phk),
            OpCode::new(0x4c, "JMP", AddressingMode::AbsoluteJMP, W65C816::jmp),
            OpCode::new(0x4d, "EOR", AddressingMode::Absolute, W65C816::eor),
            OpCode::new(0x4e, "LSR", AddressingMode::Absolute, W65C816::lsr),
            OpCode::new(0x4f, "EOR", AddressingMode::AbsoluteLong, W65C816::eor),
            OpCode::new(0x50, "BVC", AddressingMode::Relative, W65C816::bvc),
            OpCode::new(0x51, "EOR", AddressingMode::IndirectY, W65C816::eor),
            OpCode::new(0x52, "EOR", AddressingMode::Indirect, W65C816::eor),
            OpCode::new(0x53, "EOR", AddressingMode::StackRelIndirectY, W65C816::eor),
            OpCode::new(0x54, "MVN", AddressingMode::BlockMove, W65C816::mvn),
            OpCode::new(0x55, "EOR", AddressingMode::DirectX, W65C816::eor),
            OpCode::new(0x56, "LSR", AddressingMode::DirectX, W65C816::lsr),
            OpCode::new(0x57, "EOR", AddressingMode::IndirectLongY, W65C816::eor),
            OpCode::new(0x58, "CLI", AddressingMode::Implied, W65C816::cli),
            OpCode::new(0x59, "EOR", AddressingMode::AbsoluteY, W65C816::eor),
            OpCode::new(0x5a, "PHY", AddressingMode::Implied, W65C816::phy),
            OpCode::new(0x5b, "TCD", AddressingMode::Implied, W65C816::tcd),
            OpCode::new(0x5c, "JMP", AddressingMode::AbsoluteLong, W65C816::jmp),
            OpCode::new(0x5d, "EOR", AddressingMode::AbsoluteX, W65C816::eor),
            OpCode::new(0x5e, "LSR", AddressingMode::AbsoluteX, W65C816::lsr),
            OpCode::new(0x5f, "EOR", AddressingMode::AbsoluteLongX, W65C816::eor),
            OpCode::new(0x60, "RTS", AddressingMode::Implied, W65C816::rts),
            OpCode::new(0x61, "ADC", AddressingMode::IndirectX, W65C816::adc),
            OpCode::new(0x62, "PER", AddressingMode::Implied, W65C816::per),
            OpCode::new(0x63, "ADC", AddressingMode::StackRelative, W65C816::adc),
            OpCode::new(0x64, "STZ", AddressingMode::Direct, W65C816::stz),
            OpCode::new(0x65, "ADC", AddressingMode::Direct, W65C816::adc),
            OpCode::new(0x66, "ROR", AddressingMode::Direct, W65C816::ror),
            OpCode::new(0x67, "ADC", AddressingMode::IndirectLong, W65C816::adc),
            OpCode::new(0x68, "PLA", AddressingMode::Implied, W65C816::pla),
            OpCode::new(0x69, "ADC", AddressingMode::Immediate, W65C816::adc),
            OpCode::new(0x6a, "ROR", AddressingMode::Implied, W65C816::ror_a),
            OpCode::new(0x6b, "RTL", AddressingMode::Implied, W65C816::rtl),
            OpCode::new(0x6c, "JMP", AddressingMode::AbsoluteIndirect, W65C816::jmp),
            OpCode::new(0x6d, "ADC", AddressingMode::Absolute, W65C816::adc),
            OpCode::new(0x6e, "ROR", AddressingMode::Absolute, W65C816::ror),
            OpCode::new(0x6f, "ADC", AddressingMode::AbsoluteLong, W65C816::adc),
            OpCode::new(0x70, "BVS", AddressingMode::Relative, W65C816::bvs),
            OpCode::new(0x71, "ADC", AddressingMode::IndirectY, W65C816::adc),
            OpCode::new(0x72, "ADC", AddressingMode::Indirect, W65C816::adc),
            OpCode::new(0x73, "ADC", AddressingMode::StackRelIndirectY, W65C816::adc),
            OpCode::new(0x74, "STZ", AddressingMode::DirectX, W65C816::stz),
            OpCode::new(0x75, "ADC", AddressingMode::DirectX, W65C816::adc),
            OpCode::new(0x76, "ROR", AddressingMode::DirectX, W65C816::ror),
            OpCode::new(0x77, "ADC", AddressingMode::IndirectLongY, W65C816::adc),
            OpCode::new(0x78, "SEI", AddressingMode::Implied, W65C816::sei),
            OpCode::new(0x79, "ADC", AddressingMode::AbsoluteY, W65C816::adc),
            OpCode::new(0x7a, "PLY", AddressingMode::Implied, W65C816::ply),
            OpCode::new(0x7b, "TDC", AddressingMode::Implied, W65C816::tdc),
            OpCode::new(0x7c, "JMP", AddressingMode::AbsoluteIndirectX, W65C816::jmp),
            OpCode::new(0x7d, "ADC", AddressingMode::AbsoluteX, W65C816::adc),
            OpCode::new(0x7e, "ROR", AddressingMode::AbsoluteX, W65C816::ror),
            OpCode::new(0x7f, "ADC", AddressingMode::AbsoluteLongX, W65C816::adc),
            OpCode::new(0x80, "BRA", AddressingMode::Relative, W65C816::bra),
            OpCode::new(0x81, "STA", AddressingMode::IndirectX, W65C816::sta),
            OpCode::new(0x82, "BRL", AddressingMode::RelativeLong, W65C816::brl),
            OpCode::new(0x83, "STA", AddressingMode::StackRelative, W65C816::sta),
            OpCode::new(0x84, "STY", AddressingMode::Direct, W65C816::sty),
            OpCode::new(0x85, "STA", AddressingMode::Direct, W65C816::sta),
            OpCode::new(0x86, "STX", AddressingMode::Direct, W65C816::stx),
            OpCode::new(0x87, "STA", AddressingMode::IndirectLong, W65C816::sta),
            OpCode::new(0x88, "DEY", AddressingMode::Implied, W65C816::dey),
            OpCode::new(0x89, "BIT", AddressingMode::Immediate, W65C816::bit),
            OpCode::new(0x8a, "TXA", AddressingMode::Implied, W65C816::txa),
            OpCode::new(0x8b, "PHB", AddressingMode::Implied, W65C816::phb),
            OpCode::new(0x8c, "STY", AddressingMode::Absolute, W65C816::sty),
            OpCode::new(0x8d, "STA", AddressingMode::Absolute, W65C816::sta),
            OpCode::new(0x8e, "STX", AddressingMode::Absolute, W65C816::stx),
            OpCode::new(0x8f, "STA", AddressingMode::AbsoluteLong, W65C816::sta),
            OpCode::new(0x90, "BCC", AddressingMode::Relative, W65C816::bcc),
            OpCode::new(0x91, "STA", AddressingMode::IndirectY, W65C816::sta),
            OpCode::new(0x92, "STA", AddressingMode::Indirect, W65C816::sta),
            OpCode::new(0x93, "STA", AddressingMode::StackRelIndirectY, W65C816::sta),
            OpCode::new(0x94, "STY", AddressingMode::DirectX, W65C816::sty),
            OpCode::new(0x95, "STA", AddressingMode::DirectX, W65C816::sta),
            OpCode::new(0x96, "STX", AddressingMode::DirectY, W65C816::stx),
            OpCode::new(0x97, "STA", AddressingMode::IndirectLongY, W65C816::sta),
            OpCode::new(0x98, "TYA", AddressingMode::Implied, W65C816::tya),
            OpCode::new(0x99, "STA", AddressingMode::AbsoluteY, W65C816::sta),
            OpCode::new(0x9a, "TXS", AddressingMode::Implied, W65C816::txs),
            OpCode::new(0x9b, "TXY", AddressingMode::Implied, W65C816::txy),
            OpCode::new(0x9c, "STZ", AddressingMode::Absolute, W65C816::stz),
            OpCode::new(0x9d, "STA", AddressingMode::AbsoluteX, W65C816::sta),
            OpCode::new(0x9e, "STZ", AddressingMode::AbsoluteX, W65C816::stz),
            OpCode::new(0x9f, "STA", AddressingMode::AbsoluteLongX, W65C816::sta),
            OpCode::new(0xa0, "LDY", AddressingMode::Immediate, W65C816::ldy),
            OpCode::new(0xa1, "LDA", AddressingMode::IndirectX, W65C816::lda),
            OpCode::new(0xa2, "LDX", AddressingMode::Immediate, W65C816::ldx),
            OpCode::new(0xa3, "LDA", AddressingMode::StackRelative, W65C816::lda),
            OpCode::new(0xa4, "LDY", AddressingMode::Direct, W65C816::ldy),
            OpCode::new(0xa5, "LDA", AddressingMode::Direct, W65C816::lda),
            OpCode::new(0xa6, "LDX", AddressingMode::Direct, W65C816::ldx),
            OpCode::new(0xa7, "LDA", AddressingMode::IndirectLong, W65C816::lda),
            OpCode::new(0xa8, "TAY", AddressingMode::Implied, W65C816::tay),
            OpCode::new(0xa9, "LDA", AddressingMode::Immediate, W65C816::lda),
            OpCode::new(0xaa, "TAX", AddressingMode::Implied, W65C816::tax),
            OpCode::new(0xab, "PLB", AddressingMode::Implied, W65C816::plb),
            OpCode::new(0xac, "LDY", AddressingMode::Absolute, W65C816::ldy),
            OpCode::new(0xad, "LDA", AddressingMode::Absolute, W65C816::lda),
            OpCode::new(0xae, "LDX", AddressingMode::Absolute, W65C816::ldx),
            OpCode::new(0xaf, "LDA", AddressingMode::AbsoluteLong, W65C816::lda),
            OpCode::new(0xb0, "BCS", AddressingMode::Relative, W65C816::bcs),
            OpCode::new(0xb1, "LDA", AddressingMode::IndirectY, W65C816::lda),
            OpCode::new(0xb2, "LDA", AddressingMode::Indirect, W65C816::lda),
            OpCode::new(0xb3, "LDA", AddressingMode::StackRelIndirectY, W65C816::lda),
            OpCode::new(0xb4, "LDY", AddressingMode::DirectX, W65C816::ldy),
            OpCode::new(0xb5, "LDA", AddressingMode::DirectX, W65C816::lda),
            OpCode::new(0xb6, "LDX", AddressingMode::DirectY, W65C816::ldx),
            OpCode::new(0xb7, "LDA", AddressingMode::IndirectLongY, W65C816::lda),
            OpCode::new(0xb8, "CLV", AddressingMode::Implied, W65C816::clv),
            OpCode::new(0xb9, "LDA", AddressingMode::AbsoluteY, W65C816::lda),
            OpCode::new(0xba, "TSX", AddressingMode::Implied, W65C816::tsx),
            OpCode::new(0xbb, "TYX", AddressingMode::Implied, W65C816::tyx),
            OpCode::new(0xbc, "LDY", AddressingMode::AbsoluteX, W65C816::ldy),
            OpCode::new(0xbd, "LDA", AddressingMode::AbsoluteX, W65C816::lda),
            OpCode::new(0xbe, "LDX", AddressingMode::AbsoluteY, W65C816::ldx),
            OpCode::new(0xbf, "LDA", AddressingMode::AbsoluteLongX, W65C816::lda),
            OpCode::new(0xc0, "CPY", AddressingMode::Immediate, W65C816::cpy),
            OpCode::new(0xc1, "CMP", AddressingMode::IndirectX, W65C816::cmp),
            OpCode::new(0xc2, "REP", AddressingMode::Immediate, W65C816::rep),
            OpCode::new(0xc3, "CMP", AddressingMode::StackRelative, W65C816::cmp),
            OpCode::new(0xc4, "CPY", AddressingMode::Direct, W65C816::cpy),
            OpCode::new(0xc5, "CMP", AddressingMode::Direct, W65C816::cmp),
            OpCode::new(0xc6, "DEC", AddressingMode::Direct, W65C816::dec),
            OpCode::new(0xc7, "CMP", AddressingMode::IndirectLong, W65C816::cmp),
            OpCode::new(0xc8, "INY", AddressingMode::Implied, W65C816::iny),
            OpCode::new(0xc9, "CMP", AddressingMode::Immediate, W65C816::cmp),
            OpCode::new(0xca, "DEX", AddressingMode::Implied, W65C816::dex),
            OpCode::new(0xcb, "WAI", AddressingMode::Implied, W65C816::wai),
            OpCode::new(0xcc, "CPY", AddressingMode::Absolute, W65C816::cpy),
            OpCode::new(0xcd, "CMP", AddressingMode::Absolute, W65C816::cmp),
            OpCode::new(0xce, "DEC", AddressingMode::Absolute, W65C816::dec),
            OpCode::new(0xcf, "CMP", AddressingMode::AbsoluteLong, W65C816::cmp),
            OpCode::new(0xd0, "BNE", AddressingMode::Relative, W65C816::bne),
            OpCode::new(0xd1, "CMP", AddressingMode::IndirectY, W65C816::cmp),
            OpCode::new(0xd2, "CMP", AddressingMode::Indirect, W65C816::cmp),
            OpCode::new(0xd3, "CMP", AddressingMode::StackRelIndirectY, W65C816::cmp),
            OpCode::new(0xd4, "PEI", AddressingMode::StackPEI, W65C816::pei),
            OpCode::new(0xd5, "CMP", AddressingMode::DirectX, W65C816::cmp),
            OpCode::new(0xd6, "DEC", AddressingMode::DirectX, W65C816::dec),
            OpCode::new(0xd7, "CMP", AddressingMode::IndirectLongY, W65C816::cmp),
            OpCode::new(0xd8, "CLD", AddressingMode::Implied, W65C816::cld),
            OpCode::new(0xd9, "CMP", AddressingMode::AbsoluteY, W65C816::cmp),
            OpCode::new(0xda, "PHX", AddressingMode::Implied, W65C816::phx),
            OpCode::new(0xdb, "STP", AddressingMode::Implied, W65C816::stp),
            OpCode::new(0xdc, "JML", AddressingMode::AbsoluteJMP, W65C816::jml),
            OpCode::new(0xdd, "CMP", AddressingMode::AbsoluteX, W65C816::cmp),
            OpCode::new(0xde, "DEC", AddressingMode::AbsoluteX, W65C816::dec),
            OpCode::new(0xdf, "CMP", AddressingMode::AbsoluteLongX, W65C816::cmp),
            OpCode::new(0xe0, "CPX", AddressingMode::Immediate, W65C816::cpx),
            OpCode::new(0xe1, "SBC", AddressingMode::IndirectX, W65C816::sbc),
            OpCode::new(0xe2, "SEP", AddressingMode::Immediate, W65C816::sep),
            OpCode::new(0xe3, "SBC", AddressingMode::StackRelative, W65C816::sbc),
            OpCode::new(0xe4, "CPX", AddressingMode::Direct, W65C816::cpx),
            OpCode::new(0xe5, "SBC", AddressingMode::Direct, W65C816::sbc),
            OpCode::new(0xe6, "INC", AddressingMode::Direct, W65C816::inc),
            OpCode::new(0xe7, "SBC", AddressingMode::IndirectLong, W65C816::sbc),
            OpCode::new(0xe8, "INX", AddressingMode::Implied, W65C816::inx),
            OpCode::new(0xe9, "SBC", AddressingMode::Immediate, W65C816::sbc),
            OpCode::new(0xea, "NOP", AddressingMode::Implied, W65C816::nop),
            OpCode::new(0xeb, "XBA", AddressingMode::Implied, W65C816::xba),
            OpCode::new(0xec, "CPX", AddressingMode::Absolute, W65C816::cpx),
            OpCode::new(0xed, "SBC", AddressingMode::Absolute, W65C816::sbc),
            OpCode::new(0xee, "INC", AddressingMode::Absolute, W65C816::inc),
            OpCode::new(0xef, "SBC", AddressingMode::AbsoluteLong, W65C816::sbc),
            OpCode::new(0xf0, "BEQ", AddressingMode::Relative, W65C816::beq),
            OpCode::new(0xf1, "SBC", AddressingMode::IndirectY, W65C816::sbc),
            OpCode::new(0xf2, "SBC", AddressingMode::Indirect, W65C816::sbc),
            OpCode::new(0xf3, "SBC", AddressingMode::StackRelIndirectY, W65C816::sbc),
            OpCode::new(0xf4, "PEA", AddressingMode::Immediate, W65C816::pea),
            OpCode::new(0xf5, "SBC", AddressingMode::DirectX, W65C816::sbc),
            OpCode::new(0xf6, "INC", AddressingMode::DirectX, W65C816::inc),
            OpCode::new(0xf7, "SBC", AddressingMode::IndirectLongY, W65C816::sbc),
            OpCode::new(0xf8, "SED", AddressingMode::Implied, W65C816::sed),
            OpCode::new(0xf9, "SBC", AddressingMode::AbsoluteY, W65C816::sbc),
            OpCode::new(0xfa, "PLX", AddressingMode::Implied, W65C816::plx),
            OpCode::new(0xfb, "XCE", AddressingMode::Implied, W65C816::xce),
            OpCode::new(0xfc, "JSR", AddressingMode::AbsoluteIndirectX, W65C816::jsr),
            OpCode::new(0xfd, "SBC", AddressingMode::AbsoluteX, W65C816::sbc),
            OpCode::new(0xfe, "INC", AddressingMode::AbsoluteX, W65C816::inc),
            OpCode::new(0xff, "SBC", AddressingMode::AbsoluteLongX, W65C816::sbc),
        ]
    }
}
