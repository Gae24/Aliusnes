use crate::bus::Bus;
use cpu::Vector;

pub mod addressing;
mod cpu;
mod functions;
mod instructions;
mod opcode;
mod regsize;

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

pub struct W65C816<B: Bus> {
    pub cpu: cpu::Cpu,
    pub instruction_set: [OpCode<B>; 256],
}

impl<B: Bus> Default for W65C816<B> {
    fn default() -> Self {
        Self::new()
    }
}

impl<B: Bus> W65C816<B> {
    pub fn new() -> Self {
        Self {
            cpu: cpu::Cpu::new(),
            instruction_set: opcode_table(),
        }
    }

    pub fn step(&mut self, bus: &mut B) {
        if self.cpu.stopped {
            return;
        }

        if bus.fired_nmi() {
            self.cpu.waiting_interrupt = false;
            self.cpu.handle_interrupt(bus, Vector::Nmi);
        } else if !self.cpu.status.irq_disable() && bus.fired_irq() {
            self.cpu.waiting_interrupt = false;
            self.cpu.handle_interrupt(bus, Vector::Irq);
        } else if self.cpu.waiting_interrupt {
            if self.cpu.status.irq_disable() {
                self.cpu.waiting_interrupt = false;
            } else {
                return;
            }
        }

        let op = self.cpu.get_imm::<u8, B>(bus);

        let opcode = &self.instruction_set[op as usize];
        let instr = opcode.function;
        let address_mode = opcode.meta.mode;

        #[cfg(feature = "trace")]
        {
            use crate::w65c816::addressing::Address;
            let disasm = address_mode
                .disasm_operand(bus, Address::new(self.cpu.program_counter, self.cpu.pbr));

            log::trace!(
                "{} {:<10} {:02x}:{:04x} A:{:04x} X:{:04x} Y:{:04x}, S:{:04x}, D:{:04x}, DB:{:02x}, P:{:08b}",
                opcode.meta.mnemonic,
                disasm,
                self.cpu.dbr,
                (self.cpu.program_counter - 1),
                self.cpu.accumulator,
                self.cpu.index_x,
                self.cpu.index_y,
                self.cpu.stack_pointer,
                self.cpu.dpr,
                self.cpu.dbr,
                self.cpu.status.0,
            );
        }
        instr(&mut self.cpu, bus, address_mode);
    }

    pub fn peek_opcode(&self, bus: &B) -> Meta {
        let addr = addressing::Address::new(self.cpu.program_counter, self.cpu.pbr);
        let op = bus.peek_at(addr).unwrap_or_default();
        self.instruction_set[op as usize].meta
    }
}

pub const fn opcode_table<B: Bus>() -> [OpCode<B>; 256] {
    use addressing::AddressingMode::*;
    [
        OpCode::new(Meta::new(0x00, "BRK", Immediate), W65C816::brk),
        OpCode::new(Meta::new(0x01, "ORA", IndirectX), W65C816::ora),
        OpCode::new(Meta::new(0x02, "COP", Immediate), W65C816::cop),
        OpCode::new(Meta::new(0x03, "ORA", StackRelative), W65C816::ora),
        OpCode::new(Meta::new(0x04, "TSB", Direct), W65C816::tsb),
        OpCode::new(Meta::new(0x05, "ORA", Direct), W65C816::ora),
        OpCode::new(Meta::new(0x06, "ASL", Direct), W65C816::asl),
        OpCode::new(Meta::new(0x07, "ORA", IndirectLong), W65C816::ora),
        OpCode::new(Meta::new(0x08, "PHP", Implied), W65C816::php),
        OpCode::new(Meta::new(0x09, "ORA", Immediate), W65C816::ora),
        OpCode::new(Meta::new(0x0a, "ASL", Accumulator), W65C816::asl),
        OpCode::new(Meta::new(0x0b, "PHD", Implied), W65C816::phd),
        OpCode::new(Meta::new(0x0c, "TSB", Absolute), W65C816::tsb),
        OpCode::new(Meta::new(0x0d, "ORA", Absolute), W65C816::ora),
        OpCode::new(Meta::new(0x0e, "ASL", Absolute), W65C816::asl),
        OpCode::new(Meta::new(0x0f, "ORA", AbsoluteLong), W65C816::ora),
        OpCode::new(Meta::new(0x10, "BPL", Relative), W65C816::bpl),
        OpCode::new(Meta::new(0x11, "ORA", IndirectY), W65C816::ora),
        OpCode::new(Meta::new(0x12, "ORA", Indirect), W65C816::ora),
        OpCode::new(Meta::new(0x13, "ORA", StackRelIndirectY), W65C816::ora),
        OpCode::new(Meta::new(0x14, "TRB", Direct), W65C816::trb),
        OpCode::new(Meta::new(0x15, "ORA", DirectX), W65C816::ora),
        OpCode::new(Meta::new(0x16, "ASL", DirectX), W65C816::asl),
        OpCode::new(Meta::new(0x17, "ORA", IndirectLongY), W65C816::ora),
        OpCode::new(Meta::new(0x18, "CLC", Implied), W65C816::clc),
        OpCode::new(Meta::new(0x19, "ORA", AbsoluteY), W65C816::ora),
        OpCode::new(Meta::new(0x1a, "INC", Accumulator), W65C816::inc),
        OpCode::new(Meta::new(0x1b, "TCS", Implied), W65C816::tcs),
        OpCode::new(Meta::new(0x1c, "TRB", Absolute), W65C816::trb),
        OpCode::new(Meta::new(0x1d, "ORA", AbsoluteX), W65C816::ora),
        OpCode::new(Meta::new(0x1e, "ASL", AbsoluteX), W65C816::asl),
        OpCode::new(Meta::new(0x1f, "ORA", AbsoluteLongX), W65C816::ora),
        OpCode::new(Meta::new(0x20, "JSR", Absolute), W65C816::jsr),
        OpCode::new(Meta::new(0x21, "AND", IndirectX), W65C816::and),
        OpCode::new(Meta::new(0x22, "JSL", AbsoluteLong), W65C816::jsl),
        OpCode::new(Meta::new(0x23, "AND", StackRelative), W65C816::and),
        OpCode::new(Meta::new(0x24, "BIT", Direct), W65C816::bit),
        OpCode::new(Meta::new(0x25, "AND", Direct), W65C816::and),
        OpCode::new(Meta::new(0x26, "ROL", Direct), W65C816::rol),
        OpCode::new(Meta::new(0x27, "AND", IndirectLong), W65C816::and),
        OpCode::new(Meta::new(0x28, "PLP", Implied), W65C816::plp),
        OpCode::new(Meta::new(0x29, "AND", Immediate), W65C816::and),
        OpCode::new(Meta::new(0x2a, "ROL", Accumulator), W65C816::rol),
        OpCode::new(Meta::new(0x2b, "PLD", Implied), W65C816::pld),
        OpCode::new(Meta::new(0x2c, "BIT", Absolute), W65C816::bit),
        OpCode::new(Meta::new(0x2d, "AND", Absolute), W65C816::and),
        OpCode::new(Meta::new(0x2e, "ROL", Absolute), W65C816::rol),
        OpCode::new(Meta::new(0x2f, "AND", AbsoluteLong), W65C816::and),
        OpCode::new(Meta::new(0x30, "BMI", Relative), W65C816::bmi),
        OpCode::new(Meta::new(0x31, "AND", IndirectY), W65C816::and),
        OpCode::new(Meta::new(0x32, "AND", Indirect), W65C816::and),
        OpCode::new(Meta::new(0x33, "AND", StackRelIndirectY), W65C816::and),
        OpCode::new(Meta::new(0x34, "BIT", DirectX), W65C816::bit),
        OpCode::new(Meta::new(0x35, "AND", DirectX), W65C816::and),
        OpCode::new(Meta::new(0x36, "ROL", DirectX), W65C816::rol),
        OpCode::new(Meta::new(0x37, "AND", IndirectLongY), W65C816::and),
        OpCode::new(Meta::new(0x38, "SEC", Implied), W65C816::sec),
        OpCode::new(Meta::new(0x39, "AND", AbsoluteY), W65C816::and),
        OpCode::new(Meta::new(0x3a, "DEC", Accumulator), W65C816::dec),
        OpCode::new(Meta::new(0x3b, "TSC", Implied), W65C816::tsc),
        OpCode::new(Meta::new(0x3c, "BIT", AbsoluteX), W65C816::bit),
        OpCode::new(Meta::new(0x3d, "AND", AbsoluteX), W65C816::and),
        OpCode::new(Meta::new(0x3e, "ROL", AbsoluteX), W65C816::rol),
        OpCode::new(Meta::new(0x3f, "AND", AbsoluteLongX), W65C816::and),
        OpCode::new(Meta::new(0x40, "RTI", Implied), W65C816::rti),
        OpCode::new(Meta::new(0x41, "EOR", IndirectX), W65C816::eor),
        OpCode::new(Meta::new(0x42, "WDM", Immediate), W65C816::wdm),
        OpCode::new(Meta::new(0x43, "EOR", StackRelative), W65C816::eor),
        OpCode::new(Meta::new(0x44, "MVP", BlockMove), W65C816::mvp),
        OpCode::new(Meta::new(0x45, "EOR", Direct), W65C816::eor),
        OpCode::new(Meta::new(0x46, "LSR", Direct), W65C816::lsr),
        OpCode::new(Meta::new(0x47, "EOR", IndirectLong), W65C816::eor),
        OpCode::new(Meta::new(0x48, "PHA", Implied), W65C816::pha),
        OpCode::new(Meta::new(0x49, "EOR", Immediate), W65C816::eor),
        OpCode::new(Meta::new(0x4a, "LSR", Accumulator), W65C816::lsr),
        OpCode::new(Meta::new(0x4b, "PHK", Implied), W65C816::phk),
        OpCode::new(Meta::new(0x4c, "JMP", Absolute), W65C816::jmp),
        OpCode::new(Meta::new(0x4d, "EOR", Absolute), W65C816::eor),
        OpCode::new(Meta::new(0x4e, "LSR", Absolute), W65C816::lsr),
        OpCode::new(Meta::new(0x4f, "EOR", AbsoluteLong), W65C816::eor),
        OpCode::new(Meta::new(0x50, "BVC", Relative), W65C816::bvc),
        OpCode::new(Meta::new(0x51, "EOR", IndirectY), W65C816::eor),
        OpCode::new(Meta::new(0x52, "EOR", Indirect), W65C816::eor),
        OpCode::new(Meta::new(0x53, "EOR", StackRelIndirectY), W65C816::eor),
        OpCode::new(Meta::new(0x54, "MVN", BlockMove), W65C816::mvn),
        OpCode::new(Meta::new(0x55, "EOR", DirectX), W65C816::eor),
        OpCode::new(Meta::new(0x56, "LSR", DirectX), W65C816::lsr),
        OpCode::new(Meta::new(0x57, "EOR", IndirectLongY), W65C816::eor),
        OpCode::new(Meta::new(0x58, "CLI", Implied), W65C816::cli),
        OpCode::new(Meta::new(0x59, "EOR", AbsoluteY), W65C816::eor),
        OpCode::new(Meta::new(0x5a, "PHY", Implied), W65C816::phy),
        OpCode::new(Meta::new(0x5b, "TCD", Implied), W65C816::tcd),
        OpCode::new(Meta::new(0x5c, "JMP", AbsoluteLong), W65C816::jmp),
        OpCode::new(Meta::new(0x5d, "EOR", AbsoluteX), W65C816::eor),
        OpCode::new(Meta::new(0x5e, "LSR", AbsoluteX), W65C816::lsr),
        OpCode::new(Meta::new(0x5f, "EOR", AbsoluteLongX), W65C816::eor),
        OpCode::new(Meta::new(0x60, "RTS", Implied), W65C816::rts),
        OpCode::new(Meta::new(0x61, "ADC", IndirectX), W65C816::adc),
        OpCode::new(Meta::new(0x62, "PER", RelativeLong), W65C816::per),
        OpCode::new(Meta::new(0x63, "ADC", StackRelative), W65C816::adc),
        OpCode::new(Meta::new(0x64, "STZ", Direct), W65C816::stz),
        OpCode::new(Meta::new(0x65, "ADC", Direct), W65C816::adc),
        OpCode::new(Meta::new(0x66, "ROR", Direct), W65C816::ror),
        OpCode::new(Meta::new(0x67, "ADC", IndirectLong), W65C816::adc),
        OpCode::new(Meta::new(0x68, "PLA", Implied), W65C816::pla),
        OpCode::new(Meta::new(0x69, "ADC", Immediate), W65C816::adc),
        OpCode::new(Meta::new(0x6a, "ROR", Accumulator), W65C816::ror),
        OpCode::new(Meta::new(0x6b, "RTL", Implied), W65C816::rtl),
        OpCode::new(Meta::new(0x6c, "JMP", AbsoluteIndirect), W65C816::jmp),
        OpCode::new(Meta::new(0x6d, "ADC", Absolute), W65C816::adc),
        OpCode::new(Meta::new(0x6e, "ROR", Absolute), W65C816::ror),
        OpCode::new(Meta::new(0x6f, "ADC", AbsoluteLong), W65C816::adc),
        OpCode::new(Meta::new(0x70, "BVS", Relative), W65C816::bvs),
        OpCode::new(Meta::new(0x71, "ADC", IndirectY), W65C816::adc),
        OpCode::new(Meta::new(0x72, "ADC", Indirect), W65C816::adc),
        OpCode::new(Meta::new(0x73, "ADC", StackRelIndirectY), W65C816::adc),
        OpCode::new(Meta::new(0x74, "STZ", DirectX), W65C816::stz),
        OpCode::new(Meta::new(0x75, "ADC", DirectX), W65C816::adc),
        OpCode::new(Meta::new(0x76, "ROR", DirectX), W65C816::ror),
        OpCode::new(Meta::new(0x77, "ADC", IndirectLongY), W65C816::adc),
        OpCode::new(Meta::new(0x78, "SEI", Implied), W65C816::sei),
        OpCode::new(Meta::new(0x79, "ADC", AbsoluteY), W65C816::adc),
        OpCode::new(Meta::new(0x7a, "PLY", Implied), W65C816::ply),
        OpCode::new(Meta::new(0x7b, "TDC", Implied), W65C816::tdc),
        OpCode::new(Meta::new(0x7c, "JMP", AbsoluteIndirectX), W65C816::jmp),
        OpCode::new(Meta::new(0x7d, "ADC", AbsoluteX), W65C816::adc),
        OpCode::new(Meta::new(0x7e, "ROR", AbsoluteX), W65C816::ror),
        OpCode::new(Meta::new(0x7f, "ADC", AbsoluteLongX), W65C816::adc),
        OpCode::new(Meta::new(0x80, "BRA", Relative), W65C816::bra),
        OpCode::new(Meta::new(0x81, "STA", IndirectX), W65C816::sta),
        OpCode::new(Meta::new(0x82, "BRL", RelativeLong), W65C816::brl),
        OpCode::new(Meta::new(0x83, "STA", StackRelative), W65C816::sta),
        OpCode::new(Meta::new(0x84, "STY", Direct), W65C816::sty),
        OpCode::new(Meta::new(0x85, "STA", Direct), W65C816::sta),
        OpCode::new(Meta::new(0x86, "STX", Direct), W65C816::stx),
        OpCode::new(Meta::new(0x87, "STA", IndirectLong), W65C816::sta),
        OpCode::new(Meta::new(0x88, "DEY", Implied), W65C816::dey),
        OpCode::new(Meta::new(0x89, "BIT", Immediate), W65C816::bit),
        OpCode::new(Meta::new(0x8a, "TXA", Implied), W65C816::txa),
        OpCode::new(Meta::new(0x8b, "PHB", Implied), W65C816::phb),
        OpCode::new(Meta::new(0x8c, "STY", Absolute), W65C816::sty),
        OpCode::new(Meta::new(0x8d, "STA", Absolute), W65C816::sta),
        OpCode::new(Meta::new(0x8e, "STX", Absolute), W65C816::stx),
        OpCode::new(Meta::new(0x8f, "STA", AbsoluteLong), W65C816::sta),
        OpCode::new(Meta::new(0x90, "BCC", Relative), W65C816::bcc),
        OpCode::new(Meta::new(0x91, "STA", IndirectY), W65C816::sta),
        OpCode::new(Meta::new(0x92, "STA", Indirect), W65C816::sta),
        OpCode::new(Meta::new(0x93, "STA", StackRelIndirectY), W65C816::sta),
        OpCode::new(Meta::new(0x94, "STY", DirectX), W65C816::sty),
        OpCode::new(Meta::new(0x95, "STA", DirectX), W65C816::sta),
        OpCode::new(Meta::new(0x96, "STX", DirectY), W65C816::stx),
        OpCode::new(Meta::new(0x97, "STA", IndirectLongY), W65C816::sta),
        OpCode::new(Meta::new(0x98, "TYA", Implied), W65C816::tya),
        OpCode::new(Meta::new(0x99, "STA", AbsoluteY), W65C816::sta),
        OpCode::new(Meta::new(0x9a, "TXS", Implied), W65C816::txs),
        OpCode::new(Meta::new(0x9b, "TXY", Implied), W65C816::txy),
        OpCode::new(Meta::new(0x9c, "STZ", Absolute), W65C816::stz),
        OpCode::new(Meta::new(0x9d, "STA", AbsoluteX), W65C816::sta),
        OpCode::new(Meta::new(0x9e, "STZ", AbsoluteX), W65C816::stz),
        OpCode::new(Meta::new(0x9f, "STA", AbsoluteLongX), W65C816::sta),
        OpCode::new(Meta::new(0xa0, "LDY", Immediate), W65C816::ldy),
        OpCode::new(Meta::new(0xa1, "LDA", IndirectX), W65C816::lda),
        OpCode::new(Meta::new(0xa2, "LDX", Immediate), W65C816::ldx),
        OpCode::new(Meta::new(0xa3, "LDA", StackRelative), W65C816::lda),
        OpCode::new(Meta::new(0xa4, "LDY", Direct), W65C816::ldy),
        OpCode::new(Meta::new(0xa5, "LDA", Direct), W65C816::lda),
        OpCode::new(Meta::new(0xa6, "LDX", Direct), W65C816::ldx),
        OpCode::new(Meta::new(0xa7, "LDA", IndirectLong), W65C816::lda),
        OpCode::new(Meta::new(0xa8, "TAY", Implied), W65C816::tay),
        OpCode::new(Meta::new(0xa9, "LDA", Immediate), W65C816::lda),
        OpCode::new(Meta::new(0xaa, "TAX", Implied), W65C816::tax),
        OpCode::new(Meta::new(0xab, "PLB", Implied), W65C816::plb),
        OpCode::new(Meta::new(0xac, "LDY", Absolute), W65C816::ldy),
        OpCode::new(Meta::new(0xad, "LDA", Absolute), W65C816::lda),
        OpCode::new(Meta::new(0xae, "LDX", Absolute), W65C816::ldx),
        OpCode::new(Meta::new(0xaf, "LDA", AbsoluteLong), W65C816::lda),
        OpCode::new(Meta::new(0xb0, "BCS", Relative), W65C816::bcs),
        OpCode::new(Meta::new(0xb1, "LDA", IndirectY), W65C816::lda),
        OpCode::new(Meta::new(0xb2, "LDA", Indirect), W65C816::lda),
        OpCode::new(Meta::new(0xb3, "LDA", StackRelIndirectY), W65C816::lda),
        OpCode::new(Meta::new(0xb4, "LDY", DirectX), W65C816::ldy),
        OpCode::new(Meta::new(0xb5, "LDA", DirectX), W65C816::lda),
        OpCode::new(Meta::new(0xb6, "LDX", DirectY), W65C816::ldx),
        OpCode::new(Meta::new(0xb7, "LDA", IndirectLongY), W65C816::lda),
        OpCode::new(Meta::new(0xb8, "CLV", Implied), W65C816::clv),
        OpCode::new(Meta::new(0xb9, "LDA", AbsoluteY), W65C816::lda),
        OpCode::new(Meta::new(0xba, "TSX", Implied), W65C816::tsx),
        OpCode::new(Meta::new(0xbb, "TYX", Implied), W65C816::tyx),
        OpCode::new(Meta::new(0xbc, "LDY", AbsoluteX), W65C816::ldy),
        OpCode::new(Meta::new(0xbd, "LDA", AbsoluteX), W65C816::lda),
        OpCode::new(Meta::new(0xbe, "LDX", AbsoluteY), W65C816::ldx),
        OpCode::new(Meta::new(0xbf, "LDA", AbsoluteLongX), W65C816::lda),
        OpCode::new(Meta::new(0xc0, "CPY", Immediate), W65C816::cpy),
        OpCode::new(Meta::new(0xc1, "CMP", IndirectX), W65C816::cmp),
        OpCode::new(Meta::new(0xc2, "REP", Immediate), W65C816::rep),
        OpCode::new(Meta::new(0xc3, "CMP", StackRelative), W65C816::cmp),
        OpCode::new(Meta::new(0xc4, "CPY", Direct), W65C816::cpy),
        OpCode::new(Meta::new(0xc5, "CMP", Direct), W65C816::cmp),
        OpCode::new(Meta::new(0xc6, "DEC", Direct), W65C816::dec),
        OpCode::new(Meta::new(0xc7, "CMP", IndirectLong), W65C816::cmp),
        OpCode::new(Meta::new(0xc8, "INY", Implied), W65C816::iny),
        OpCode::new(Meta::new(0xc9, "CMP", Immediate), W65C816::cmp),
        OpCode::new(Meta::new(0xca, "DEX", Implied), W65C816::dex),
        OpCode::new(Meta::new(0xcb, "WAI", Implied), W65C816::wai),
        OpCode::new(Meta::new(0xcc, "CPY", Absolute), W65C816::cpy),
        OpCode::new(Meta::new(0xcd, "CMP", Absolute), W65C816::cmp),
        OpCode::new(Meta::new(0xce, "DEC", Absolute), W65C816::dec),
        OpCode::new(Meta::new(0xcf, "CMP", AbsoluteLong), W65C816::cmp),
        OpCode::new(Meta::new(0xd0, "BNE", Relative), W65C816::bne),
        OpCode::new(Meta::new(0xd1, "CMP", IndirectY), W65C816::cmp),
        OpCode::new(Meta::new(0xd2, "CMP", Indirect), W65C816::cmp),
        OpCode::new(Meta::new(0xd3, "CMP", StackRelIndirectY), W65C816::cmp),
        OpCode::new(Meta::new(0xd4, "PEI", Direct), W65C816::pei),
        OpCode::new(Meta::new(0xd5, "CMP", DirectX), W65C816::cmp),
        OpCode::new(Meta::new(0xd6, "DEC", DirectX), W65C816::dec),
        OpCode::new(Meta::new(0xd7, "CMP", IndirectLongY), W65C816::cmp),
        OpCode::new(Meta::new(0xd8, "CLD", Implied), W65C816::cld),
        OpCode::new(Meta::new(0xd9, "CMP", AbsoluteY), W65C816::cmp),
        OpCode::new(Meta::new(0xda, "PHX", Implied), W65C816::phx),
        OpCode::new(Meta::new(0xdb, "STP", Implied), W65C816::stp),
        OpCode::new(Meta::new(0xdc, "JML", AbsoluteIndirectLong), W65C816::jml),
        OpCode::new(Meta::new(0xdd, "CMP", AbsoluteX), W65C816::cmp),
        OpCode::new(Meta::new(0xde, "DEC", AbsoluteX), W65C816::dec),
        OpCode::new(Meta::new(0xdf, "CMP", AbsoluteLongX), W65C816::cmp),
        OpCode::new(Meta::new(0xe0, "CPX", Immediate), W65C816::cpx),
        OpCode::new(Meta::new(0xe1, "SBC", IndirectX), W65C816::sbc),
        OpCode::new(Meta::new(0xe2, "SEP", Immediate), W65C816::sep),
        OpCode::new(Meta::new(0xe3, "SBC", StackRelative), W65C816::sbc),
        OpCode::new(Meta::new(0xe4, "CPX", Direct), W65C816::cpx),
        OpCode::new(Meta::new(0xe5, "SBC", Direct), W65C816::sbc),
        OpCode::new(Meta::new(0xe6, "INC", Direct), W65C816::inc),
        OpCode::new(Meta::new(0xe7, "SBC", IndirectLong), W65C816::sbc),
        OpCode::new(Meta::new(0xe8, "INX", Implied), W65C816::inx),
        OpCode::new(Meta::new(0xe9, "SBC", Immediate), W65C816::sbc),
        OpCode::new(Meta::new(0xea, "NOP", Implied), W65C816::nop),
        OpCode::new(Meta::new(0xeb, "XBA", Implied), W65C816::xba),
        OpCode::new(Meta::new(0xec, "CPX", Absolute), W65C816::cpx),
        OpCode::new(Meta::new(0xed, "SBC", Absolute), W65C816::sbc),
        OpCode::new(Meta::new(0xee, "INC", Absolute), W65C816::inc),
        OpCode::new(Meta::new(0xef, "SBC", AbsoluteLong), W65C816::sbc),
        OpCode::new(Meta::new(0xf0, "BEQ", Relative), W65C816::beq),
        OpCode::new(Meta::new(0xf1, "SBC", IndirectY), W65C816::sbc),
        OpCode::new(Meta::new(0xf2, "SBC", Indirect), W65C816::sbc),
        OpCode::new(Meta::new(0xf3, "SBC", StackRelIndirectY), W65C816::sbc),
        OpCode::new(Meta::new(0xf4, "PEA", Immediate), W65C816::pea),
        OpCode::new(Meta::new(0xf5, "SBC", DirectX), W65C816::sbc),
        OpCode::new(Meta::new(0xf6, "INC", DirectX), W65C816::inc),
        OpCode::new(Meta::new(0xf7, "SBC", IndirectLongY), W65C816::sbc),
        OpCode::new(Meta::new(0xf8, "SED", Implied), W65C816::sed),
        OpCode::new(Meta::new(0xf9, "SBC", AbsoluteY), W65C816::sbc),
        OpCode::new(Meta::new(0xfa, "PLX", Implied), W65C816::plx),
        OpCode::new(Meta::new(0xfb, "XCE", Implied), W65C816::xce),
        OpCode::new(Meta::new(0xfc, "JSR", AbsoluteIndirectX), W65C816::jsr),
        OpCode::new(Meta::new(0xfd, "SBC", AbsoluteX), W65C816::sbc),
        OpCode::new(Meta::new(0xfe, "INC", AbsoluteX), W65C816::inc),
        OpCode::new(Meta::new(0xff, "SBC", AbsoluteLongX), W65C816::sbc),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::testbus::{deserialize_as_map, Cycle, TomHarteBus};
    use crate::utils::testrun::{run_test, OpcodeTest};
    use crate::w65c816::cpu::{Cpu, Status};
    use serde::{Deserialize, Deserializer};
    use std::{collections::HashMap, path::PathBuf};

    #[derive(Debug, PartialEq, Deserialize)]
    pub struct CpuState {
        pc: u16,
        s: u16,
        p: u8,
        a: u16,
        x: u16,
        y: u16,
        dbr: u8,
        d: u16,
        pbr: u8,
        e: u8,
        #[serde(deserialize_with = "deserialize_as_map")]
        ram: HashMap<u32, u8>,
    }

    impl CpuState {
        pub fn convert_state(&self) -> (W65C816<TomHarteBus>, TomHarteBus) {
            let mut w65c816 = W65C816::new();
            w65c816.cpu.accumulator = self.a;
            w65c816.cpu.index_x = self.x;
            w65c816.cpu.index_y = self.y;
            w65c816.cpu.stack_pointer = self.s;
            w65c816.cpu.program_counter = self.pc;
            w65c816.cpu.status = Status(self.p);
            w65c816.cpu.dpr = self.d;
            w65c816.cpu.pbr = self.pbr;
            w65c816.cpu.dbr = self.dbr;
            w65c816.cpu.emulation_mode = self.e == 1;
            w65c816.cpu.stopped = false;
            w65c816.cpu.waiting_interrupt = false;

            let bus = TomHarteBus {
                memory: self.ram.clone(),
                ..Default::default()
            };

            (w65c816, bus)
        }
    }

    impl From<(Cpu, TomHarteBus)> for CpuState {
        fn from(value: (Cpu, TomHarteBus)) -> Self {
            Self {
                pc: value.0.program_counter,
                s: value.0.stack_pointer,
                p: value.0.status.0,
                a: value.0.accumulator,
                x: value.0.index_x,
                y: value.0.index_y,
                dbr: value.0.dbr,
                d: value.0.dpr,
                pbr: value.0.pbr,
                e: value.0.emulation_mode as u8,
                ram: value.1.memory,
            }
        }
    }

    impl std::fmt::Display for CpuState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
            f,
            "pc:{:04X} s:{:04X} p:{:02X} a:{:04X} x:{:04X} y:{:04X} dbr:{:02X} d:{:04X} pbr:{:02X} e:{:01X} \n\t  ram:{:02X?}",
            self.pc,
            self.s,
            self.p,
            self.a,
            self.x,
            self.y,
            self.dbr,
            self.d,
            self.pbr,
            self.e,
            self.ram
        )
        }
    }

    impl OpcodeTest for CpuState {
        type Proc = Cpu;

        fn test_path(name: &str) -> PathBuf {
            let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            root_dir.join(format!("tests/65816/{name}.json.xz"))
        }

        fn do_step(&mut self, other: &Self, cycles_len: usize) -> (Self::Proc, TomHarteBus, bool) {
            let (mut w65c816, mut bus) = self.convert_state();

            let opcode = w65c816.peek_opcode(&bus);
            let skip_cycles = if opcode.code == 0x44 || opcode.code == 0x54 {
                loop {
                    if bus.cycles.len() >= (cycles_len - 2) {
                        break;
                    }
                    w65c816.step(&mut bus);
                }
                w65c816.cpu.program_counter = other.pc;
                true
            } else {
                w65c816.step(&mut bus);
                false
            };

            (w65c816.cpu, bus, skip_cycles)
        }

        fn deserialize_cycles<'de, D: Deserializer<'de>>(
            deserializer: D,
        ) -> Result<Vec<Cycle>, D::Error> {
            let v: Vec<(Option<u32>, Option<u8>, String)> = Deserialize::deserialize(deserializer)?;
            let mut cycles: Vec<Cycle> = v
                .iter()
                .map(|(addr, value, state)| {
                    if !(state.contains('p') || state.contains('d')) {
                        Cycle::Internal
                    } else if state.contains('r') {
                        Cycle::Read(addr.unwrap_or_default(), *value)
                    } else if state.contains('w') {
                        Cycle::Write(addr.unwrap_or_default(), value.unwrap_or_default())
                    } else {
                        Cycle::Internal
                    }
                })
                .collect();
            cycles.sort();
            Ok(cycles)
        }
    }

    include!(concat!(env!("OUT_DIR"), "/tomharte_65816.rs"));
}
