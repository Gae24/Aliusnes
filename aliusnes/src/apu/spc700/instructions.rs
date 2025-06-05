use crate::{
    apu::spc700::{
        addressing::{AddressingMode, Source},
        cpu::{Cpu, Status},
        functions::{do_branch, do_compare},
        Spc700,
    },
    bus::Bus,
    utils::int_traits::ManipulateU16,
    w65c816::addressing::Address,
};

impl<B: Bus> Spc700<B> {
    pub fn addw(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        let a = u32::from(cpu.ya());

        let offset = cpu.get_imm(bus);
        let b = u32::from(cpu.word_from_direct_page(bus, offset));
        let result = a + b;

        let is_overflow = !(a ^ b) & (a ^ result) & 0x8000 != 0;
        let is_half_carry = ((a & 0xFFF) + (b & 0xFFF)) >> 12 != 0;
        cpu.status.set_carry(result >> 16 != 0);
        cpu.status.set_overflow(is_overflow);
        cpu.status.set_half_carry(is_half_carry);

        let result = result as u16;
        cpu.status.set_negative(result >> 15 != 0);
        cpu.status.set_zero(result == 0);
        cpu.set_ya(result);

        bus.add_io_cycles(1);
    }

    pub fn and1<const INVERSE: bool>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = (cpu.operand(bus, mode) != 0) ^ INVERSE;
        cpu.status.set_carry(cpu.status.carry() & operand);
    }

    pub fn and_a(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = cpu.operand(bus, mode);
        cpu.accumulator &= operand;
        cpu.status.set_negative(cpu.accumulator >> 7 != 0);
        cpu.status.set_zero(cpu.accumulator == 0);
    }

    pub fn asl(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        cpu.do_rmw(bus, &mode, |cpu, operand| {
            let res = operand << 1;
            cpu.status.set_carry(operand >> 7 != 0);
            cpu.status.set_negative(res >> 7 != 0);
            cpu.status.set_zero(res == 0);

            res
        });
    }

    pub fn asl_a(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.status.set_carry(cpu.accumulator >> 7 != 0);
        cpu.accumulator = cpu.accumulator << 1;
        cpu.status.set_negative(cpu.accumulator >> 7 != 0);
        cpu.status.set_zero(cpu.accumulator == 0);

        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
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

    pub fn bmi(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        do_branch(cpu, bus, cpu.status.negative());
    }

    pub fn bpl(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        do_branch(cpu, bus, cpu.status.negative() == false);
    }

    pub fn bra(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        do_branch(cpu, bus, true);
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

    pub fn bvc(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        do_branch(cpu, bus, !cpu.status.overflow());
    }

    pub fn bvs(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        do_branch(cpu, bus, cpu.status.overflow());
    }

    pub fn call(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        bus.add_io_cycles(3);
        let new_pc = cpu.decode_addressing_mode(bus, mode).offset;

        cpu.do_push(bus, cpu.program_counter.high_byte());
        cpu.do_push(bus, cpu.program_counter.low_byte());
        cpu.program_counter = new_pc;
    }

    pub fn cbne(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = cpu.operand(bus, mode);
        bus.add_io_cycles(1);
        do_branch(cpu, bus, cpu.accumulator != operand);
    }

    pub fn clr1<const BIT: u8>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        cpu.do_rmw(bus, &mode, |_cpu, operand| {
            let mask = 1 << BIT;
            operand & !mask
        });
    }

    pub fn clrc(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        cpu.status.set_carry(false);
    }

    pub fn clrp(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        cpu.status.set_direct_page(false);
    }

    pub fn cmpw(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        let a = cpu.ya();
        let offset = cpu.get_imm(bus);
        let b = cpu.word_from_direct_page(bus, offset);

        let result = a.wrapping_sub(b);
        cpu.status.set_carry(a >= b);
        cpu.status.set_negative(result >> 15 != 0);
        cpu.status.set_zero(result == 0);
    }

    pub fn cmp_reg<const REG: Source>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let a = match REG {
            Source::A => cpu.accumulator,
            Source::X => cpu.index_x,
            Source::Y => cpu.index_y,
            Source::PSW => cpu.status.0,
        };
        let b = cpu.operand(bus, mode);

        do_compare(cpu, a, b);
    }

    pub fn dbnz<const REG: bool>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        if REG {
            cpu.index_y = cpu.index_y.wrapping_sub(1);
            do_branch(cpu, bus, cpu.index_y != 0);
        } else {
            let addr = cpu.decode_addressing_mode(bus, mode);
            let operand = bus.read_and_tick(addr);

            let res = operand.wrapping_sub(1);

            bus.write_and_tick(addr, res);
            do_branch(cpu, bus, res != 0);
        }
    }

    pub fn decw(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.do_rmw_word(bus, |cpu, operand| {
            let res = operand.wrapping_sub(1);
            cpu.status.set_negative(res >> 15 != 0);
            cpu.status.set_zero(res == 0);
            res
        })
    }

    pub fn dec_x(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.index_x = cpu.index_x.wrapping_sub(1);
        cpu.status.set_negative(cpu.index_x >> 7 != 0);
        cpu.status.set_zero(cpu.index_x == 0);

        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
    }

    pub fn eor_a(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = cpu.operand(bus, mode);
        cpu.accumulator ^= operand;
        cpu.status.set_negative(cpu.accumulator >> 7 != 0);
        cpu.status.set_zero(cpu.accumulator == 0);
    }

    pub fn incw(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.do_rmw_word(bus, |cpu, operand| {
            let res = operand.wrapping_add(1);
            cpu.status.set_negative(res >> 15 != 0);
            cpu.status.set_zero(res == 0);
            res
        })
    }

    pub fn inc_x(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.index_x = cpu.index_x.wrapping_add(1);
        cpu.status.set_negative(cpu.index_x >> 7 != 0);
        cpu.status.set_zero(cpu.index_x == 0);

        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
    }

    pub fn jmp(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let value = cpu.decode_addressing_mode(bus, mode).offset;
        if let AddressingMode::Absolute = mode {
            cpu.program_counter = value;
        } else {
            cpu.program_counter = cpu.read_16(bus, value);
        }
    }

    pub fn lsr(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        cpu.do_rmw(bus, &mode, |cpu, operand| {
            let res = operand >> 1;
            cpu.status.set_carry(operand & 1 != 0);
            cpu.status.set_negative(res >> 7 != 0);
            cpu.status.set_zero(res == 0);

            res
        });
    }

    pub fn lsr_a(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.status.set_carry(cpu.accumulator & 1 != 0);
        cpu.accumulator = cpu.accumulator >> 1;
        cpu.status.set_negative(cpu.accumulator >> 7 != 0);
        cpu.status.set_zero(cpu.accumulator == 0);

        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
    }

    pub fn mov(_cpu: &mut Cpu, _bus: &mut B, _mode: AddressingMode) {}

    pub fn nop(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
    }

    pub fn or<const DEST: AddressingMode>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let b = cpu.operand(bus, mode);
        cpu.do_rmw(bus, &DEST, |cpu, a| {
            let res = a | b;
            cpu.status.set_negative(res >> 7 != 0);
            cpu.status.set_zero(res == 0);
            res
        })
    }

    pub fn or_a(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = cpu.operand(bus, mode);
        cpu.accumulator |= operand;
        cpu.status.set_negative(cpu.accumulator >> 7 != 0);
        cpu.status.set_zero(cpu.accumulator == 0);
    }

    pub fn or1<const INVERSE: bool>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = (cpu.operand(bus, mode) != 0) ^ INVERSE;
        cpu.status.set_carry(cpu.status.carry() | operand);
        bus.add_io_cycles(1);
    }

    pub fn pcall(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(2);
        let new_pc = u16::from_le_bytes([cpu.get_imm(bus), 0xFF]);

        cpu.do_push(bus, cpu.program_counter.high_byte());
        cpu.do_push(bus, cpu.program_counter.low_byte());
        cpu.program_counter = new_pc;
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

    pub fn ret(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        bus.add_io_cycles(1);

        let new_pc = u16::from_le_bytes([cpu.do_pop(bus), cpu.do_pop(bus)]);
        cpu.program_counter = new_pc;
    }

    pub fn reti(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        bus.add_io_cycles(1);

        cpu.status = Status(cpu.do_pop(bus));
        let new_pc = u16::from_le_bytes([cpu.do_pop(bus), cpu.do_pop(bus)]);
        cpu.program_counter = new_pc;
    }

    pub fn rol(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        cpu.do_rmw(bus, &mode, |cpu, operand| {
            let res = (operand << 1) | u8::from(cpu.status.carry());
            cpu.status.set_carry(operand >> 7 != 0);
            cpu.status.set_negative(res >> 7 != 0);
            cpu.status.set_zero(res == 0);

            res
        });
    }

    pub fn rol_a(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        let carry_bit = u8::from(cpu.status.carry());
        cpu.status.set_carry(cpu.accumulator >> 7 != 0);
        cpu.accumulator = (cpu.accumulator << 1) | carry_bit;
        cpu.status.set_negative(cpu.accumulator >> 7 != 0);
        cpu.status.set_zero(cpu.accumulator == 0);

        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
    }

    pub fn ror(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        cpu.do_rmw(bus, &mode, |cpu, operand| {
            let res = (operand >> 1) | (u8::from(cpu.status.carry()) << 7);
            cpu.status.set_carry(operand & 1 != 0);
            cpu.status.set_negative(res >> 7 != 0);
            cpu.status.set_zero(res == 0);

            res
        });
    }

    pub fn ror_a(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        let carry_bit = u8::from(cpu.status.carry()) << 7;
        cpu.status.set_carry(cpu.accumulator & 1 != 0);
        cpu.accumulator = (cpu.accumulator >> 1) | carry_bit;
        cpu.status.set_negative(cpu.accumulator >> 7 != 0);
        cpu.status.set_zero(cpu.accumulator == 0);

        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
    }

    pub fn set1<const BIT: u8>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        cpu.do_rmw(bus, &mode, |_cpu, operand| {
            let mask = 1 << BIT;
            operand | mask
        });
    }

    pub fn setp(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        cpu.status.set_direct_page(true);
    }

    pub fn tcall<const INDEX: u8>(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        bus.add_io_cycles(2);

        cpu.do_push(bus, cpu.program_counter.high_byte());
        cpu.do_push(bus, cpu.program_counter.low_byte());

        let vector_addr = 0xFFDE - 2 * INDEX as u16;
        cpu.program_counter = cpu.read_16(bus, vector_addr);
    }

    pub fn tclr1(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let addr = cpu.decode_addressing_mode(bus, mode);
        let operand = bus.read_and_tick(addr);
        let nz_value = cpu.accumulator.wrapping_sub(operand);
        cpu.status.set_negative(nz_value >> 7 != 0);
        cpu.status.set_zero(nz_value == 0);

        // Dummy read
        let _ = bus.read_and_tick(addr);
        bus.write_and_tick(addr, !cpu.accumulator & operand);
    }

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
