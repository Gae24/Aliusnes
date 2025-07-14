use crate::{
    bus::Bus,
    w65c816::addressing::{Address, AddressingMode},
};

impl AddressingMode {
    const fn operand_size(&self) -> u8 {
        match self {
            AddressingMode::Accumulator => 0,
            AddressingMode::Implied => 0,
            AddressingMode::Immediate => 1,
            AddressingMode::Relative => 1,
            AddressingMode::RelativeLong => 2,
            AddressingMode::Direct => 1,
            AddressingMode::DirectX => 1,
            AddressingMode::DirectY => 1,
            AddressingMode::Indirect => 1,
            AddressingMode::IndirectX => 1,
            AddressingMode::IndirectY => 1,
            AddressingMode::IndirectLong => 1,
            AddressingMode::IndirectLongY => 1,
            AddressingMode::Absolute => 2,
            AddressingMode::AbsoluteX => 2,
            AddressingMode::AbsoluteY => 2,
            AddressingMode::AbsoluteLong => 3,
            AddressingMode::AbsoluteLongX => 3,
            AddressingMode::AbsoluteIndirect => 2,
            AddressingMode::AbsoluteIndirectLong => 2,
            AddressingMode::AbsoluteIndirectX => 2,
            AddressingMode::AbsoluteJMP => 2,
            AddressingMode::AbsoluteLongJSL => 2,
            AddressingMode::StackRelative => 1,
            AddressingMode::StackRelIndirectY => 1,
            AddressingMode::StackPEI => 1,
            AddressingMode::BlockMove => 1,
        }
    }

    pub fn disasm_operand<B: Bus>(&self, bus: &B, addr: Address) -> String {
        let operand = match self.operand_size() {
            0 => 0,
            1 => bus.peek_at(addr).unwrap_or_default() as u32,
            2 => u16::from_le_bytes([
                bus.peek_at(addr).unwrap_or_default(),
                bus.peek_at(addr.wrapping_offset_add(1)).unwrap_or_default(),
            ]) as u32,
            3 => u32::from_le_bytes([
                bus.peek_at(addr).unwrap_or_default(),
                bus.peek_at(addr.wrapping_offset_add(1)).unwrap_or_default(),
                bus.peek_at(addr.wrapping_offset_add(2)).unwrap_or_default(),
                0,
            ]),
            _ => unreachable!(),
        };

        match self {
            AddressingMode::Accumulator => "".to_string(),
            AddressingMode::Implied => "".to_string(),
            AddressingMode::Immediate => format!("#${:02X}", operand),
            AddressingMode::Relative => format!("${:04X}", operand),
            AddressingMode::RelativeLong => format!("${:06X}", operand),
            AddressingMode::Direct => format!("${:02X}", operand),
            AddressingMode::DirectX => format!("${:02X},X", operand),
            AddressingMode::DirectY => format!("${:02X},Y", operand),
            AddressingMode::Indirect => format!("(${:02X})", operand),
            AddressingMode::IndirectX => format!("(${:02X},X)", operand),
            AddressingMode::IndirectY => format!("(${:02X}),Y", operand),
            AddressingMode::IndirectLong => format!("[${:02X}]", operand),
            AddressingMode::IndirectLongY => format!("[${:02X}],Y", operand),
            AddressingMode::Absolute => format!("${:04X}", operand),
            AddressingMode::AbsoluteX => format!("${:04X},X", operand),
            AddressingMode::AbsoluteY => format!("${:04X},Y", operand),
            AddressingMode::AbsoluteLong => format!("${:06X}", operand),
            AddressingMode::AbsoluteLongX => format!("${:06X},X", operand),
            AddressingMode::AbsoluteIndirect => format!("(${:04X})", operand),
            AddressingMode::AbsoluteIndirectLong => todo!(),
            AddressingMode::AbsoluteIndirectX => format!("(${:04X},X)", operand),
            AddressingMode::AbsoluteJMP => format!("${:04X}", operand),
            AddressingMode::AbsoluteLongJSL => format!("${:06X}", operand),
            AddressingMode::StackRelative => format!("${:02X},S", operand),
            AddressingMode::StackRelIndirectY => format!("(${:02X},S),Y", operand),
            AddressingMode::StackPEI => format!("(${:02X})", operand),
            AddressingMode::BlockMove => {
                format!("${:02X} ${:02X}", operand as u8, (operand >> 8) & 0xFF)
            }
        }
    }
}
