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
}
