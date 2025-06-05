use crate::{apu::spc700::cpu::Cpu, bus::Bus};

pub(super) fn do_branch<B: Bus>(cpu: &mut Cpu, bus: &mut B, cond: bool) {
    let offset = cpu.get_imm(bus) as i8;
    if cond {
        bus.add_io_cycles(2);
        cpu.program_counter = cpu.program_counter.wrapping_add(offset as u16);
    }
}

pub(super) fn do_compare(cpu: &mut Cpu, a: u8, b: u8) {
    let result = a.wrapping_sub(b);
    cpu.status.set_carry(a >= b);
    cpu.set_nz(result);
}
