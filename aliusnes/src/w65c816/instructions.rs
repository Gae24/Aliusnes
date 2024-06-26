use super::{
    addressing::{Address, AddressingMode},
    cpu::{Cpu, Vectors},
    functions::*,
};
use crate::bus::Bus;

impl<B: Bus> super::W65C816<B> {
    pub fn adc(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        if cpu.status.a_reg_size() {
            let operand = cpu.get_operand::<u8, B>(bus, &mode);
            if cpu.status.decimal() {
                do_dec_adc::<u8>(cpu, operand);
            } else {
                do_bin_adc::<u8>(cpu, operand);
            }
        } else {
            let operand = cpu.get_operand::<u16, B>(bus, &mode);
            if cpu.status.decimal() {
                do_dec_adc::<u16>(cpu, operand);
            } else {
                do_bin_adc::<u16>(cpu, operand);
            }
        }
    }

    pub fn and(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        if cpu.status.a_reg_size() {
            let operand = cpu.get_operand::<u8, B>(bus, &mode);
            let result = cpu.accumulator as u8 & operand;
            cpu.set_nz(result);
            cpu.set_accumulator(result);
        } else {
            let operand = cpu.get_operand::<u16, B>(bus, &mode);
            let result = cpu.accumulator & operand;
            cpu.set_nz(result);
            cpu.set_accumulator(result);
        }
    }

    pub fn asl(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.a_reg_size() {
            cpu.do_rmw(bus, &mode, do_asl::<u8>);
        } else {
            cpu.do_rmw(bus, &mode, do_asl::<u16>);
        }
    }

    pub fn asl_a(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.a_reg_size() {
            let result = do_asl(cpu, cpu.accumulator as u8);
            cpu.set_accumulator(result);
        } else {
            let result = do_asl(cpu, cpu.accumulator);
            cpu.set_accumulator(result);
        }
    }

    pub fn bcc(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        do_branch(cpu, bus, &mode, !cpu.status.carry());
    }

    pub fn bcs(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        do_branch(cpu, bus, &mode, cpu.status.carry());
    }

    pub fn beq(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        do_branch(cpu, bus, &mode, cpu.status.zero());
    }

    pub fn bit(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        if cpu.status.a_reg_size() {
            let operand = cpu.get_operand::<u8, B>(bus, &mode);
            do_bit::<u8>(cpu, operand, &mode);
        } else {
            let operand = cpu.get_operand::<u16, B>(bus, &mode);
            do_bit::<u16>(cpu, operand, &mode);
        }
    }

    pub fn bmi(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        do_branch(cpu, bus, &mode, cpu.status.negative());
    }

    pub fn bne(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        do_branch(cpu, bus, &mode, !cpu.status.zero());
    }

    pub fn bpl(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        do_branch(cpu, bus, &mode, !cpu.status.negative());
    }

    pub fn bra(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        do_branch(cpu, bus, &mode, true);
    }

    pub fn brl(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        bus.add_io_cycles(1);
        let offset = cpu.get_operand::<u16, B>(bus, &mode) as i16;
        cpu.program_counter = cpu.program_counter.wrapping_add(offset as u16);
    }

    pub fn bvc(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        do_branch(cpu, bus, &mode, !cpu.status.overflow());
    }

    pub fn bvs(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        do_branch(cpu, bus, &mode, cpu.status.overflow());
    }

    pub fn brk(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let _thrown = cpu.get_operand::<u8, B>(bus, &mode);
        if !cpu.emulation_mode() {
            cpu.handle_interrupt(bus, Vectors::Brk);
        } else {
            cpu.handle_interrupt(bus, Vectors::EmuBrk);
        }
    }

    pub fn cop(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let _thrown = cpu.get_operand::<u8, B>(bus, &mode);
        if !cpu.emulation_mode() {
            cpu.handle_interrupt(bus, Vectors::Cop);
        } else {
            cpu.handle_interrupt(bus, Vectors::EmuCop);
        }
    }

    pub fn clc(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        cpu.status.set_carry(false);
    }

    pub fn cld(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        cpu.status.set_decimal(false);
    }

    pub fn cli(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        cpu.status.set_irq_disable(false);
    }

    pub fn clv(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        cpu.status.set_overflow(false);
    }

    pub fn cmp(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        if cpu.status.a_reg_size() {
            let operand = cpu.get_operand::<u8, B>(bus, &mode);
            do_cmp(cpu, cpu.accumulator as u8, operand);
        } else {
            let operand = cpu.get_operand::<u16, B>(bus, &mode);
            do_cmp(cpu, cpu.accumulator, operand);
        }
    }

    pub fn cpx(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        if cpu.status.index_regs_size() {
            let operand = cpu.get_operand::<u8, B>(bus, &mode);
            do_cmp(cpu, cpu.index_x as u8, operand);
        } else {
            let operand = cpu.get_operand::<u16, B>(bus, &mode);
            do_cmp(cpu, cpu.index_x, operand);
        }
    }

    pub fn cpy(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        if cpu.status.index_regs_size() {
            let operand = cpu.get_operand::<u8, B>(bus, &mode);
            do_cmp(cpu, cpu.index_y as u8, operand);
        } else {
            let operand = cpu.get_operand::<u16, B>(bus, &mode);
            do_cmp(cpu, cpu.index_y, operand);
        }
    }

    pub fn dec(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.a_reg_size() {
            cpu.do_rmw::<u8, B>(bus, &mode, do_dec);
        } else {
            cpu.do_rmw::<u16, B>(bus, &mode, do_dec);
        }
    }

    pub fn dec_a(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.a_reg_size() {
            let result = do_dec::<u8>(cpu, cpu.accumulator as u8);
            cpu.set_accumulator(result);
        } else {
            let result = do_dec::<u16>(cpu, cpu.accumulator);
            cpu.set_accumulator(result);
        }
    }

    pub fn dex(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.index_regs_size() {
            let result = do_dec::<u8>(cpu, cpu.index_x as u8);
            cpu.set_index_x(result);
        } else {
            let result = do_dec::<u16>(cpu, cpu.index_x);
            cpu.set_index_x(result);
        }
    }

    pub fn dey(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.index_regs_size() {
            let result = do_dec::<u8>(cpu, cpu.index_y as u8);
            cpu.set_index_y(result);
        } else {
            let result = do_dec::<u16>(cpu, cpu.index_y);
            cpu.set_index_y(result);
        }
    }

    pub fn eor(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        if cpu.status.a_reg_size() {
            let operand = cpu.get_operand::<u8, B>(bus, &mode);
            let result = cpu.accumulator as u8 ^ operand;
            cpu.set_nz(result);
            cpu.set_accumulator(result);
        } else {
            let operand = cpu.get_operand::<u16, B>(bus, &mode);
            let result = cpu.accumulator ^ operand;
            cpu.set_nz(result);
            cpu.set_accumulator(result);
        }
    }

    pub fn inc(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.a_reg_size() {
            cpu.do_rmw::<u8, B>(bus, &mode, do_inc);
        } else {
            cpu.do_rmw::<u16, B>(bus, &mode, do_inc);
        }
    }

    pub fn inc_a(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.a_reg_size() {
            let result = do_inc::<u8>(cpu, cpu.accumulator as u8);
            cpu.set_accumulator(result);
        } else {
            let result = do_inc::<u16>(cpu, cpu.accumulator);
            cpu.set_accumulator(result);
        }
    }

    pub fn inx(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.index_regs_size() {
            let result = do_inc::<u8>(cpu, cpu.index_x as u8);
            cpu.set_index_x(result);
        } else {
            let result = do_inc::<u16>(cpu, cpu.index_x);
            cpu.set_index_x(result);
        }
    }

    pub fn iny(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.index_regs_size() {
            let result = do_inc::<u8>(cpu, cpu.index_y as u8);
            cpu.set_index_y(result);
        } else {
            let result = do_inc::<u16>(cpu, cpu.index_y);
            cpu.set_index_y(result);
        }
    }

    pub fn jml(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let addr = cpu.get_operand::<u16, B>(bus, &mode);
        let pc = cpu.read_bank0(bus, addr);
        let pbr = bus.read_and_tick(addr.wrapping_add(2).into());
        cpu.program_counter = pc;
        cpu.pbr = pbr;
    }

    pub fn jmp(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        match mode {
            AddressingMode::AbsoluteLong => {
                let new_pc = cpu.get_operand::<u16, B>(bus, &AddressingMode::AbsoluteJMP);
                let new_pbr = cpu.get_operand::<u8, B>(bus, &AddressingMode::AbsoluteJMP);
                cpu.program_counter = new_pc;
                cpu.pbr = new_pbr;
            }
            _ => {
                cpu.program_counter = cpu.get_operand::<u16, B>(bus, &mode);
            }
        }
    }

    pub fn jsl(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        bus.add_io_cycles(1);
        let new_pc = cpu.get_operand::<u16, B>(bus, &mode);
        do_push(cpu, bus, cpu.pbr);
        let new_pbr = cpu.get_operand::<u8, B>(bus, &mode);
        do_push(cpu, bus, cpu.program_counter.wrapping_sub(1));
        cpu.program_counter = new_pc;
        cpu.pbr = new_pbr;
    }

    pub fn jsr(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let val = match mode {
            AddressingMode::AbsoluteIndirectX => {
                let addr = cpu.decode_addressing_mode::<false, B>(bus, mode);
                bus.read_and_tick(addr) as u16
                    | (bus.read_and_tick(addr.wrapping_offset_add(1)) as u16) << 8
            }
            _ => {
                bus.add_io_cycles(1);
                cpu.get_operand(bus, &mode)
            }
        };
        do_push(cpu, bus, cpu.program_counter.wrapping_sub(1));
        cpu.program_counter = val;
    }

    pub fn lda(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        if cpu.status.a_reg_size() {
            let operand = cpu.get_operand::<u8, B>(bus, &mode);
            cpu.set_accumulator(operand);
            cpu.set_nz(operand);
        } else {
            let operand = cpu.get_operand::<u16, B>(bus, &mode);
            cpu.set_accumulator(operand);
            cpu.set_nz(operand);
        }
    }

    pub fn ldx(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        if cpu.status.index_regs_size() {
            let operand = cpu.get_operand::<u8, B>(bus, &mode);
            cpu.set_index_x(operand);
            cpu.set_nz(operand);
        } else {
            let operand = cpu.get_operand::<u16, B>(bus, &mode);
            cpu.set_index_x(operand);
            cpu.set_nz(operand);
        }
    }

    pub fn ldy(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        if cpu.status.index_regs_size() {
            let operand = cpu.get_operand::<u8, B>(bus, &mode);
            cpu.set_index_y(operand);
            cpu.set_nz(operand);
        } else {
            let operand = cpu.get_operand::<u16, B>(bus, &mode);
            cpu.set_index_y(operand);
            cpu.set_nz(operand);
        }
    }

    pub fn lsr(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.a_reg_size() {
            cpu.do_rmw::<u8, B>(bus, &mode, do_lsr);
        } else {
            cpu.do_rmw::<u16, B>(bus, &mode, do_lsr);
        }
    }

    pub fn lsr_a(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.a_reg_size() {
            let result = do_lsr::<u8>(cpu, cpu.accumulator as u8);
            cpu.set_accumulator(result);
        } else {
            let result = do_lsr::<u16>(cpu, cpu.accumulator);
            cpu.set_accumulator(result);
        }
    }

    pub fn mvn(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let banks = cpu.get_operand::<u16, B>(bus, &mode);
        let dst_bank = (banks >> 8) as u8;
        let src_bank = (banks & 0xFF) as u8;
        bus.add_io_cycles(2);
        loop {
            let src = bus.read_and_tick(Address::new(cpu.index_x, src_bank));
            let dst = cpu.index_y as u32 | (dst_bank as u32) << 16;
            bus.write_and_tick(dst.into(), src);
            cpu.index_x = cpu.index_x.wrapping_add(1);
            cpu.index_y = cpu.index_y.wrapping_add(1);
            cpu.accumulator = cpu.accumulator.wrapping_sub(1);
            if cpu.accumulator == 0xFFFF {
                break;
            }
        }
    }

    pub fn mvp(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let banks = cpu.get_operand::<u16, B>(bus, &mode);
        let dst_bank = (banks >> 8) as u8;
        let src_bank = (banks & 0xFF) as u8;
        bus.add_io_cycles(2);
        loop {
            let src = bus.read_and_tick(Address::new(cpu.index_x, src_bank));
            let dst = cpu.index_y as u32 | (dst_bank as u32) << 16;
            bus.write_and_tick(dst.into(), src);
            cpu.index_x = cpu.index_x.wrapping_sub(1);
            cpu.index_y = cpu.index_y.wrapping_sub(1);
            cpu.accumulator = cpu.accumulator.wrapping_sub(1);
            if cpu.accumulator == 0xFFFF {
                break;
            }
        }
    }

    pub fn nop(_cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
    }

    pub fn ora(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        if cpu.status.a_reg_size() {
            let operand = cpu.get_operand::<u8, B>(bus, &mode);
            let result = cpu.accumulator as u8 | operand;
            cpu.set_nz(result);
            cpu.set_accumulator(result);
        } else {
            let operand = cpu.get_operand::<u16, B>(bus, &mode);
            let result = cpu.accumulator | operand;
            cpu.set_nz(result);
            cpu.set_accumulator(result);
        }
    }

    pub fn pea(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let value = cpu.get_operand::<u16, B>(bus, &mode);
        do_push(cpu, bus, value);
    }

    pub fn pei(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        let value = cpu.get_operand::<u16, B>(bus, &mode);
        do_push(cpu, bus, value);
    }

    pub fn per(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        bus.add_io_cycles(1);
        let value = cpu.get_operand::<u16, B>(bus, &mode);
        do_push(cpu, bus, cpu.program_counter.wrapping_add(value));
    }

    pub fn pha(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.a_reg_size() {
            do_push(cpu, bus, cpu.accumulator as u8);
        } else {
            do_push(cpu, bus, cpu.accumulator);
        }
    }

    pub fn phb(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        do_push(cpu, bus, cpu.dbr);
    }

    pub fn phd(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        do_push(cpu, bus, cpu.dpr);
    }

    pub fn phk(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        do_push(cpu, bus, cpu.pbr);
    }

    pub fn php(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        do_push::<u8, B>(cpu, bus, cpu.status.0);
    }

    pub fn phx(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.index_regs_size() {
            do_push::<u8, B>(cpu, bus, cpu.index_x as u8);
        } else {
            do_push::<u16, B>(cpu, bus, cpu.index_x);
        }
    }

    pub fn phy(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.index_regs_size() {
            do_push::<u8, B>(cpu, bus, cpu.index_y as u8);
        } else {
            do_push::<u16, B>(cpu, bus, cpu.index_y);
        }
    }

    pub fn pla(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(2);
        if cpu.status.a_reg_size() {
            let result = do_pull::<u8, B>(cpu, bus);
            cpu.set_nz(result);
            cpu.set_accumulator(result);
        } else {
            let result = do_pull::<u16, B>(cpu, bus);
            cpu.set_nz(result);
            cpu.set_accumulator(result);
        }
    }

    pub fn plb(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(2);
        let result = do_pull::<u8, B>(cpu, bus);
        cpu.set_nz(result);
        cpu.dbr = result;
    }

    pub fn pld(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(2);
        let result = do_pull::<u16, B>(cpu, bus);
        cpu.set_nz(result);
        cpu.dpr = result;
    }

    pub fn plp(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(2);
        let result = do_pull::<u8, B>(cpu, bus);
        cpu.set_status_register(result);
    }

    pub fn plx(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(2);
        if cpu.status.index_regs_size() {
            let result = do_pull::<u8, B>(cpu, bus);
            cpu.set_index_x(result);
            cpu.set_nz(result);
        } else {
            let result = do_pull::<u16, B>(cpu, bus);
            cpu.set_index_x(result);
            cpu.set_nz(result);
        }
    }

    pub fn ply(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(2);
        if cpu.status.index_regs_size() {
            let result = do_pull::<u8, B>(cpu, bus);
            cpu.set_index_y(result);
            cpu.set_nz(result);
        } else {
            let result = do_pull::<u16, B>(cpu, bus);
            cpu.set_index_y(result);
            cpu.set_nz(result);
        }
    }

    pub fn rep(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        bus.add_io_cycles(1);
        let mask = cpu.get_operand::<u8, B>(bus, &mode);
        let src = cpu.status.0;
        cpu.status = super::cpu::Status(src & !mask);
    }

    pub fn rol(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.a_reg_size() {
            cpu.do_rmw::<u8, B>(bus, &mode, do_rol);
        } else {
            cpu.do_rmw::<u16, B>(bus, &mode, do_rol);
        }
    }

    pub fn rol_a(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.a_reg_size() {
            let result = do_rol::<u8>(cpu, cpu.accumulator as u8);
            cpu.set_accumulator(result);
        } else {
            let result = do_rol::<u16>(cpu, cpu.accumulator);
            cpu.set_accumulator(result);
        }
    }

    pub fn ror(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.a_reg_size() {
            cpu.do_rmw::<u8, B>(bus, &mode, do_ror);
        } else {
            cpu.do_rmw::<u16, B>(bus, &mode, do_ror);
        }
    }

    pub fn ror_a(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.a_reg_size() {
            let result = do_ror::<u8>(cpu, cpu.accumulator as u8);
            cpu.set_accumulator(result);
        } else {
            let result = do_ror::<u16>(cpu, cpu.accumulator);
            cpu.set_accumulator(result);
        }
    }

    pub fn rti(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(2);
        let new_status = do_pull::<u8, B>(cpu, bus);
        cpu.program_counter = do_pull::<u16, B>(cpu, bus);
        if !cpu.emulation_mode() {
            cpu.pbr = do_pull::<u8, B>(cpu, bus);
        }
        cpu.set_status_register(new_status);
    }

    pub fn rtl(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(2);
        cpu.program_counter = do_pull::<u16, B>(cpu, bus).wrapping_add(1);
        cpu.pbr = do_pull::<u8, B>(cpu, bus);
    }

    pub fn rts(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(3);
        cpu.program_counter = do_pull::<u16, B>(cpu, bus).wrapping_add(1);
    }

    pub fn sbc(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        if cpu.status.a_reg_size() {
            let operand = cpu.get_operand::<u8, B>(bus, &mode);
            if cpu.status.decimal() {
                do_dec_sbc::<u8>(cpu, !operand);
            } else {
                do_bin_adc::<u8>(cpu, !operand);
            }
        } else {
            let operand = cpu.get_operand::<u16, B>(bus, &mode);
            if cpu.status.decimal() {
                do_dec_sbc::<u16>(cpu, !operand);
            } else {
                do_bin_adc::<u16>(cpu, !operand);
            }
        }
    }

    pub fn sec(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        cpu.status.set_carry(true);
    }

    pub fn sed(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        cpu.status.set_decimal(true);
    }

    pub fn sei(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        cpu.status.set_irq_disable(true);
    }

    pub fn sep(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        bus.add_io_cycles(1);
        let mask = cpu.get_operand::<u8, B>(bus, &mode);
        let src = cpu.status.0;
        cpu.set_status_register(src | mask);
    }

    pub fn sta(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        if cpu.status.a_reg_size() {
            cpu.do_write(bus, &mode, cpu.accumulator as u8);
        } else {
            cpu.do_write(bus, &mode, cpu.accumulator);
        }
    }

    pub fn stp(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        cpu.stopped = true;
        bus.add_io_cycles(3);
    }

    pub fn stx(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        if cpu.status.index_regs_size() {
            cpu.do_write(bus, &mode, cpu.index_x as u8);
        } else {
            cpu.do_write(bus, &mode, cpu.index_x);
        }
    }

    pub fn sty(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        if cpu.status.index_regs_size() {
            cpu.do_write(bus, &mode, cpu.index_y as u8);
        } else {
            cpu.do_write(bus, &mode, cpu.index_y);
        }
    }

    pub fn stz(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        if cpu.status.a_reg_size() {
            cpu.do_write::<u8, B>(bus, &mode, 0);
        } else {
            cpu.do_write::<u16, B>(bus, &mode, 0);
        }
    }

    pub fn tax(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.index_regs_size() {
            let value = cpu.accumulator as u8;
            cpu.set_nz(value);
            cpu.set_index_x(value);
        } else {
            cpu.set_nz(cpu.accumulator);
            cpu.set_index_x(cpu.accumulator);
        }
    }

    pub fn tay(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.index_regs_size() {
            let value = cpu.accumulator as u8;
            cpu.set_nz(value);
            cpu.set_index_y(value);
        } else {
            cpu.set_nz(cpu.accumulator);
            cpu.set_index_y(cpu.accumulator);
        }
    }

    pub fn tcd(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        cpu.dpr = cpu.accumulator;
        cpu.set_nz(cpu.accumulator);
    }

    pub fn tcs(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        //todo in emulation mode only 8bit are transferred
        bus.add_io_cycles(1);
        cpu.stack_pointer = cpu.accumulator;
    }

    pub fn tdc(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        cpu.accumulator = cpu.dpr;
        cpu.set_nz(cpu.dpr);
    }

    pub fn trb(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.a_reg_size() {
            cpu.do_rmw::<u8, B>(bus, &mode, do_trb);
        } else {
            cpu.do_rmw::<u16, B>(bus, &mode, do_trb);
        }
    }

    pub fn tsb(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.a_reg_size() {
            cpu.do_rmw::<u8, B>(bus, &mode, do_tsb);
        } else {
            cpu.do_rmw::<u16, B>(bus, &mode, do_tsb);
        }
    }

    pub fn tsc(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        cpu.accumulator = cpu.stack_pointer;
        cpu.set_nz(cpu.stack_pointer);
    }

    pub fn tsx(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.index_regs_size() {
            let value = cpu.stack_pointer as u8;
            cpu.set_nz(value);
            cpu.set_index_x(value as u16);
        } else {
            cpu.set_nz(cpu.stack_pointer);
            cpu.set_index_x(cpu.stack_pointer);
        }
    }

    pub fn txa(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.a_reg_size() {
            let value = cpu.index_x as u8;
            cpu.set_nz(value);
            cpu.set_accumulator(value);
        } else {
            cpu.set_nz(cpu.index_x);
            cpu.set_accumulator(cpu.index_x);
            if cpu.status.index_regs_size() {
                cpu.accumulator &= 0xFF;
            }
        }
    }

    pub fn txs(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        //todo in emulation mode only 8bit are transferred
        bus.add_io_cycles(1);
        cpu.stack_pointer = cpu.index_x;
    }

    pub fn txy(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.index_regs_size() {
            let value = cpu.index_x as u8;
            cpu.set_nz(value);
            cpu.set_index_y(value);
        } else {
            cpu.set_nz(cpu.index_x);
            cpu.set_index_y(cpu.index_x);
        }
    }

    pub fn tya(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.a_reg_size() {
            let value = cpu.index_y as u8;
            cpu.set_nz(value);
            cpu.set_accumulator(value);
        } else {
            cpu.set_nz(cpu.index_y);
            cpu.set_accumulator(cpu.index_y);
            if cpu.status.index_regs_size() {
                cpu.accumulator &= 0xFF;
            }
        }
    }

    pub fn tyx(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        if cpu.status.index_regs_size() {
            let value = cpu.index_y as u8;
            cpu.set_nz(value);
            cpu.set_index_x(value);
        } else {
            cpu.set_nz(cpu.index_y);
            cpu.set_index_x(cpu.index_y);
        }
    }

    pub fn wai(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(3);
        cpu.waiting_interrupt = true;
    }

    pub fn wdm(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode) {
        bus.add_io_cycles(1);
        cpu.program_counter += 1;
    }

    pub fn xba(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(2);
        cpu.accumulator = cpu.accumulator.swap_bytes();
        cpu.set_nz(cpu.accumulator as u8);
    }

    pub fn xce(cpu: &mut Cpu, bus: &mut B, _mode: AddressingMode) {
        bus.add_io_cycles(1);
        let carry = cpu.status.carry();
        cpu.status.set_carry(cpu.emulation_mode());
        cpu.set_emulation_mode(carry);
    }
}
