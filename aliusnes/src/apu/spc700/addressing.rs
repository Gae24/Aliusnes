use crate::{apu::spc700::Cpu, bus::Bus, w65c816::addressing::Address};

#[derive(Clone, Copy)]
pub enum AddressingMode {
    Implied,
    Immediate,
    DirectPage,
    Absolute,
    DirectX,
    DirectY,
    DirectXPostIncrement,
    DirectYPostIncrement,
    IndirectX,
    IndirectY,
    XIndirect,
    YIndirect,
}

impl Cpu {
    pub fn get_imm<B: Bus>(&mut self, bus: &mut B) -> u8 {
        let addr = Address::new(self.program_counter, 0);
        self.program_counter = self.program_counter.wrapping_add(1);
        bus.read_and_tick(addr)
    }

    pub fn decode_addressing_mode<B: Bus>(&mut self, bus: &mut B, mode: AddressingMode) -> Address {
        match mode {
            AddressingMode::DirectPage => {
                let base_addr: u16 = if self.status.direct_page() {
                    0x0100
                } else {
                    0x0000
                };
                let page = base_addr.wrapping_add(u16::from(self.get_imm(bus)));
                Address::new(page, 0)
            }
            _ => todo!(),
        }
    }

    pub fn operand<B: Bus>(&mut self, bus: &mut B, mode: AddressingMode) -> u8 {
        let addr = self.decode_addressing_mode(bus, mode);
        bus.read_and_tick(addr)
    }
}
