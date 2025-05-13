use crate::{
    apu::spc700::{
        addressing::{AddressingMode, Source},
        cpu::Cpu,
        functions::do_branch,
        Spc700,
    },
    bus::Bus,
    utils::int_traits::ManipulateU16,
    w65c816::addressing::Address,
};

impl<B: Bus> Spc700<B> {
    pub fn asl(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        cpu.do_rmw(bus, &mode, |cpu, operand| {
            let res = operand << 1;
            cpu.status.set_carry(operand >> 7 != 0);
            cpu.status.set_negative(res >> 7 != 0);
            cpu.status.set_zero(res == 0);

            res
        });
    }

    pub fn bbs<const BIT: u8>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = cpu.operand(bus, mode);
        bus.add_io_cycles(1);
        do_branch(cpu, bus, (operand & (1 << BIT)) != 0);
    }

    pub fn bbc<const BIT: u8>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = cpu.operand(bus, mode);
        bus.add_io_cycles(1);
        do_branch(cpu, bus, (operand & (1 << BIT)) == 0);
    }

    pub fn bpl(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        do_branch(cpu, bus, cpu.status.negative() == false);
    }

    pub fn brk(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        cpu.do_push(bus, cpu.program_counter.high_byte());
        cpu.do_push(bus, cpu.program_counter.low_byte());
        cpu.do_push(bus, cpu.status.0);

        cpu.program_counter = cpu.read_16(bus, 0xFFDE);
        cpu.status.set_irq_enabled(false);
        cpu.status.set_break_(true);
        bus.add_io_cycles(1);
    }

    pub fn clr1<const BIT: u8>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        cpu.do_rmw(bus, &mode, |_cpu, operand| {
            let mask = 1 << BIT;
            operand & !mask
        });
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

    pub fn or1(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = cpu.operand(bus, mode) != 0;
        cpu.status.set_carry(cpu.status.carry() | operand);
        bus.add_io_cycles(1);
    }

    pub fn push<const REG: Source>(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        let value = match REG {
            Source::A => cpu.accumulator,
            Source::X => cpu.index_x,
            Source::Y => cpu.index_y,
            Source::PSW => cpu.status.0,
        };
        cpu.do_push(bus, value);

        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        bus.add_io_cycles(1);
    }

    pub fn set1<const BIT: u8>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        cpu.do_rmw(bus, &mode, |_cpu, operand| {
            let mask = 1 << BIT;
            operand | mask
        });
    }

    pub fn tcall<const INDEX: u8>(_cpu: &mut Cpu, _bus: &mut B, _mode: AddressingMode) {}

    pub fn tset1(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let addr = cpu.decode_addressing_mode(bus, mode);
        let operand = bus.read_and_tick(addr);
        let nz_value = cpu.accumulator.wrapping_sub(operand);
        cpu.status.set_negative(nz_value >> 7 != 0);
        cpu.status.set_zero(nz_value == 0);

        // Dummy read
        let _ = bus.read_and_tick(addr);
        bus.write_and_tick(addr, cpu.accumulator | operand);
    }
}
