use crate::{
    apu::spc700::{
        addressing::AddressingMode,
        cpu::{Cpu, Status},
        Spc700,
    },
    bus::Bus,
    utils::int_traits::ManipulateU16,
    w65c816::addressing::Address,
};

impl<B: Bus> Spc700<B> {
    pub fn adc<const DEST: AddressingMode>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let b = cpu.operand(bus, mode);
        cpu.do_rmw::<_, false>(bus, DEST, |cpu, a| cpu.do_adc(a, b));
    }

    pub fn addw(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        let offset = cpu.get_imm(bus);
        let b = cpu.word_from_direct_page(bus, offset);

        cpu.status.set_carry(false);

        cpu.accumulator = cpu.do_adc(cpu.accumulator, b.low_byte());
        cpu.index_y = cpu.do_adc(cpu.index_y, b.high_byte());
        cpu.status
            .set_zero(cpu.index_y == 0 && cpu.accumulator == 0);

        bus.add_io_cycles(1);
    }

    pub fn and1<const INVERSE: bool>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = (cpu.operand(bus, mode) != 0) ^ INVERSE;
        cpu.status.set_carry(cpu.status.carry() & operand);
    }

    pub fn and<const DEST: AddressingMode>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let b = cpu.operand(bus, mode);
        cpu.do_rmw::<_, false>(bus, DEST, |cpu, a| {
            let res = a & b;
            cpu.set_nz(res);
            res
        })
    }

    pub fn asl(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        cpu.do_rmw::<_, true>(bus, mode, |cpu, operand| {
            let res = operand << 1;
            cpu.status.set_carry(operand >> 7 != 0);
            cpu.set_nz(res);

            res
        });
    }

    pub fn bbs<const BIT: u8>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = cpu.operand(bus, mode);
        bus.add_io_cycles(1);
        cpu.do_branch(bus, (operand & (1 << BIT)) != 0);
    }

    pub fn bbc<const BIT: u8>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = cpu.operand(bus, mode);
        bus.add_io_cycles(1);
        cpu.do_branch(bus, (operand & (1 << BIT)) == 0);
    }

    pub fn bcc(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.do_branch(bus, !cpu.status.carry());
    }

    pub fn bcs(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.do_branch(bus, cpu.status.carry());
    }

    pub fn beq(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.do_branch(bus, cpu.status.zero());
    }

    pub fn bmi(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.do_branch(bus, cpu.status.negative());
    }

    pub fn bne(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.do_branch(bus, !cpu.status.zero());
    }

    pub fn bpl(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.do_branch(bus, !cpu.status.negative());
    }

    pub fn bra(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.do_branch(bus, true);
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
        cpu.do_branch(bus, !cpu.status.overflow());
    }

    pub fn bvs(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.do_branch(bus, cpu.status.overflow());
    }

    pub fn call(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(3);
        let new_pc = cpu.abs(bus);

        cpu.do_push(bus, cpu.program_counter.high_byte());
        cpu.do_push(bus, cpu.program_counter.low_byte());
        cpu.program_counter = new_pc;
    }

    pub fn cbne(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = cpu.operand(bus, mode);
        bus.add_io_cycles(1);
        cpu.do_branch(bus, cpu.accumulator != operand);
    }

    pub fn clr1<const BIT: u8>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        cpu.do_rmw::<_, false>(bus, mode, |_, operand| {
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

    pub fn clrv(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        cpu.status.set_overflow(false);
        cpu.status.set_half_carry(false);
    }

    pub fn cmp<const OPERAND_A: AddressingMode>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        if !OPERAND_A.is_register_access() {
            bus.add_io_cycles(1);
        }
        let b = cpu.operand(bus, mode);
        let a = cpu.operand(bus, OPERAND_A);

        cpu.status.set_carry(a >= b);
        cpu.set_nz(a.wrapping_sub(b));
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

    pub fn daa(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        if cpu.status.carry() || cpu.accumulator > 0x99 {
            cpu.accumulator = cpu.accumulator.wrapping_add(0x60);
            cpu.status.set_carry(true);
        }
        if cpu.status.half_carry() || cpu.accumulator & 0xF > 0x09 {
            cpu.accumulator = cpu.accumulator.wrapping_add(0x06);
        }
        cpu.set_nz(cpu.accumulator);

        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        bus.add_io_cycles(1);
    }

    pub fn das(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        if !cpu.status.carry() || cpu.accumulator > 0x99 {
            cpu.accumulator = cpu.accumulator.wrapping_sub(0x60);
            cpu.status.set_carry(false);
        }
        if !cpu.status.half_carry() || cpu.accumulator & 0xF > 0x09 {
            cpu.accumulator = cpu.accumulator.wrapping_sub(0x06);
        }
        cpu.set_nz(cpu.accumulator);

        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        bus.add_io_cycles(1);
    }

    pub fn dbnz(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let mut is_non_zero = false;
        cpu.do_rmw::<_, true>(bus, mode, |_, operand| {
            let res = operand.wrapping_sub(1);
            is_non_zero = res != 0;
            res
        });
        cpu.do_branch(bus, is_non_zero);
    }

    pub fn decw(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.do_rmw_word(bus, |cpu, operand| {
            let res = operand.wrapping_sub(1);
            cpu.status.set_negative(res >> 15 != 0);
            cpu.status.set_zero(res == 0);
            res
        })
    }

    pub fn dec(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        cpu.do_rmw::<_, true>(bus, mode, |cpu, operand| {
            let res = operand.wrapping_sub(1);
            cpu.set_nz(res);
            res
        });
    }

    pub fn di(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.status.set_irq_enabled(false);

        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        bus.add_io_cycles(1);
    }

    pub fn div(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        // Based on ares's implementation:
        cpu.status
            .set_half_carry(cpu.index_y & 0xF >= cpu.index_x & 0xF);
        cpu.status.set_overflow(cpu.index_y >= cpu.index_x);
        let ya = cpu.ya();
        let x = cpu.index_x as u16;
        if (cpu.index_y as u16) < x << 1 {
            cpu.accumulator = (ya / x) as u8;
            cpu.index_y = (ya % x) as u8;
        } else {
            cpu.accumulator = (255 - (ya - (x << 9)) / (256 - x)) as u8;
            cpu.index_y = (x + (ya - (x << 9)) % (256 - x)) as u8;
        };
        cpu.set_nz(cpu.accumulator);
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        bus.add_io_cycles(10);
    }

    pub fn ei(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.status.set_irq_enabled(true);

        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        bus.add_io_cycles(1);
    }

    pub fn eor1(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = cpu.operand(bus, mode) != 0;
        cpu.status.set_carry(cpu.status.carry() ^ operand);
        bus.add_io_cycles(1);
    }

    pub fn eor<const DEST: AddressingMode>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let b = cpu.operand(bus, mode);
        cpu.do_rmw::<_, false>(bus, DEST, |cpu, a| {
            let res = a ^ b;
            cpu.set_nz(res);
            res
        })
    }

    pub fn inc(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        cpu.do_rmw::<_, true>(bus, mode, |cpu, operand| {
            let res = operand.wrapping_add(1);
            cpu.set_nz(res);
            res
        });
    }

    pub fn incw(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.do_rmw_word(bus, |cpu, operand| {
            let res = operand.wrapping_add(1);
            cpu.status.set_negative(res >> 15 != 0);
            cpu.status.set_zero(res == 0);
            res
        })
    }

    pub fn jmp(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let value = cpu.decode_addressing_mode(bus, mode);
        if let AddressingMode::Absolute = mode {
            cpu.program_counter = value;
        } else {
            cpu.program_counter = cpu.read_16(bus, value);
        }
    }

    pub fn lsr(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        cpu.do_rmw::<_, true>(bus, mode, |cpu, operand| {
            let res = operand >> 1;
            cpu.status.set_carry(operand & 1 != 0);
            cpu.set_nz(res);

            res
        });
    }

    pub fn mov<const DEST: AddressingMode>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = cpu.operand(bus, mode);
        match DEST {
            AddressingMode::Accumulator => cpu.accumulator = operand,
            AddressingMode::X => cpu.index_x = operand,
            AddressingMode::Y => cpu.index_y = operand,
            AddressingMode::Sp => {
                let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
                cpu.stack_pointer = operand;
            }
            AddressingMode::Psw => cpu.status = Status(operand),
            _ => {
                let page = cpu.decode_addressing_mode(bus, DEST);
                if !matches!(DEST, AddressingMode::DirectXPostIncrement)
                    && !matches!(mode, AddressingMode::DirectPage)
                {
                    let _ = bus.read_and_tick(Address::new(page, 0));
                }
                bus.write_and_tick(Address::new(page, 0), operand);
            }
        }

        if matches!(mode, AddressingMode::Sp) {
            // Dummy read
            let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        }

        if DEST.is_register_access() {
            cpu.set_nz(operand);

            if mode.is_register_access() {
                // Dummy read
                let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
            }
        }
    }

    pub fn movw<const REG: bool>(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        let offset = cpu.get_imm(bus);
        if REG {
            let value = cpu.word_from_direct_page(bus, offset);
            cpu.accumulator = value.low_byte();
            cpu.index_y = value.high_byte();
            cpu.status.set_negative(value >> 15 != 0);
            cpu.status.set_zero(value == 0);
            bus.add_io_cycles(1);
        } else {
            let _ = bus.read_and_tick(u16::from_le_bytes([offset, cpu.direct_page()]).into());
            bus.write_and_tick(
                u16::from_le_bytes([offset, cpu.direct_page()]).into(),
                cpu.accumulator,
            );
            bus.write_and_tick(
                u16::from_le_bytes([offset.wrapping_add(1), cpu.direct_page()]).into(),
                cpu.index_y,
            );
        }
    }

    pub fn mov1<const CARRY: bool>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        if CARRY {
            let operand = cpu.operand(bus, mode);
            cpu.status.set_carry(operand != 0);
        } else {
            cpu.do_rmw::<_, false>(bus, mode, |cpu, _| u8::from(cpu.status.carry()));
            bus.add_io_cycles(1);
        }
    }

    pub fn mul(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        let result = cpu.index_y as u16 * cpu.accumulator as u16;
        cpu.accumulator = result.low_byte();
        cpu.index_y = result.high_byte();
        cpu.set_nz(cpu.index_y);

        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        bus.add_io_cycles(7);
    }

    pub fn nop(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
    }

    pub fn not1(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        cpu.do_rmw::<_, false>(bus, mode, |_, operand| !operand);
    }

    pub fn notc(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.status.set_carry(!cpu.status.carry());

        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        bus.add_io_cycles(1);
    }

    pub fn or<const DEST: AddressingMode>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let b = cpu.operand(bus, mode);
        cpu.do_rmw::<_, false>(bus, DEST, |cpu, a| {
            let res = a | b;
            cpu.set_nz(res);
            res
        })
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

    pub fn pop(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = cpu.do_pop(bus);
        match mode {
            AddressingMode::Accumulator => cpu.accumulator = operand,
            AddressingMode::X => cpu.index_x = operand,
            AddressingMode::Y => cpu.index_y = operand,
            AddressingMode::Psw => cpu.status = Status(operand),
            _ => unreachable!(),
        }

        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        bus.add_io_cycles(1);
    }

    pub fn push(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let operand = cpu.operand(bus, mode);
        cpu.do_push(bus, operand);

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
        cpu.do_rmw::<_, true>(bus, mode, |cpu, operand| {
            let res = (operand << 1) | u8::from(cpu.status.carry());
            cpu.status.set_carry(operand >> 7 != 0);
            cpu.set_nz(res);

            res
        });
    }

    pub fn ror(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        cpu.do_rmw::<_, true>(bus, mode, |cpu, operand| {
            let res = (operand >> 1) | (u8::from(cpu.status.carry()) << 7);
            cpu.status.set_carry(operand & 1 != 0);
            cpu.set_nz(res);

            res
        });
    }

    pub fn sbc<const DEST: AddressingMode>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let b = cpu.operand(bus, mode);
        cpu.do_rmw::<_, false>(bus, DEST, |cpu, a| cpu.do_adc(a, !b));
    }

    pub fn set1<const BIT: u8>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        cpu.do_rmw::<_, false>(bus, mode, |_, operand| {
            let mask = 1 << BIT;
            operand | mask
        });
    }

    pub fn setc(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        cpu.status.set_carry(true);
    }

    pub fn setp(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        cpu.status.set_direct_page(true);
    }

    pub fn sleep(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.paused = true;

        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        bus.add_io_cycles(3);
    }

    pub fn stop(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.paused = true;

        // Dummy read
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
        bus.add_io_cycles(3);
    }

    pub fn subw(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        let offset = cpu.get_imm(bus);
        let b = cpu.word_from_direct_page(bus, offset);

        cpu.status.set_carry(true);

        cpu.accumulator = cpu.do_adc(cpu.accumulator, !b.low_byte());
        cpu.index_y = cpu.do_adc(cpu.index_y, !b.high_byte());
        cpu.status
            .set_zero(cpu.index_y == 0 && cpu.accumulator == 0);

        bus.add_io_cycles(1);
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
        cpu.do_test_bit(bus, mode, true);
    }

    pub fn tset1(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        cpu.do_test_bit(bus, mode, false);
    }

    pub fn xcn(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.accumulator = cpu.accumulator.rotate_right(4);
        cpu.set_nz(cpu.accumulator);
        bus.add_io_cycles(3);
        let _ = bus.read_and_tick(Address::new(cpu.program_counter, 0));
    }
}
