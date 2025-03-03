use crate::apu::spc700::addressing::AddressingMode;
use crate::apu::spc700::cpu::Cpu;
use crate::bus::Bus;

use super::Spc700;

impl<B: Bus> Spc700<B> {
    pub fn nop(_cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
    }

    pub fn tcall<const index: u8>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {}
}
