use crate::{apu::spc700::cpu::Cpu, bus::Bus};

pub(super) fn do_branch<B: Bus>(cpu: &mut Cpu, bus: &mut B, cond: bool) {
    let offset = cpu.get_imm(bus) as i8;
    bus.add_io_cycles(1);
    if cond {
        bus.add_io_cycles(2);
        cpu.program_counter = cpu.program_counter.wrapping_add(offset as u16);
    }
}
