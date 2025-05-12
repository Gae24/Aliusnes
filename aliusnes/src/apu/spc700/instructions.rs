use crate::{
    apu::spc700::{addressing::AddressingMode, cpu::Cpu, functions::do_branch, Spc700},
    bus::Bus,
};

impl<B: Bus> Spc700<B> {
    pub fn bbs<const BIT: u8>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = cpu.operand(bus, mode);
        do_branch(cpu, bus, (operand & (1 << BIT)) != 0);
    }

    pub fn nop(_cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
    }

    pub fn or(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = cpu.operand(bus, mode);
        let dest_addr = cpu.decode_addressing_mode(bus, mode);

        let res = bus.read_and_tick(dest_addr) | operand;
        cpu.status.set_negative(res >> 7 != 0);
        cpu.status.set_zero(res == 0);

        bus.write_and_tick(dest_addr, res);
    }

    pub fn or_a(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = cpu.operand(bus, mode);
        cpu.accumulator |= operand;
        cpu.status.set_negative(cpu.accumulator >> 7 != 0);
        cpu.status.set_zero(cpu.accumulator == 0);
    }

    pub fn set1<const BIT: u8>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        cpu.do_rmw(bus, &mode, |_cpu, operand| {
            let mask = 1 << BIT;
            operand | mask
        });
    }

    pub fn tcall<const INDEX: u8>(_cpu: &mut Cpu, _bus: &mut B, _mode: AddressingMode) {}
}
