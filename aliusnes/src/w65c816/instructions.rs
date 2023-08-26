use crate::bus::Bus;

use super::{
    cpu::{AddressingMode, Cpu, CpuFlags},
    functions::*,
    regsize::RegSize,
    vectors::NativeVectors,
};

pub fn adc<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let operand = cpu.get_operand::<A>(bus, mode);
    if cpu.status_register.contains(CpuFlags::DECIMAL) {
        do_dec_adc::<A>(cpu, operand);
    } else {
        do_bin_adc::<A>(cpu, operand);
    }
}

pub fn and<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let operand = cpu.get_operand::<A>(bus, mode);
    let result = A::trunc_u16(cpu.accumulator) & operand;
    cpu.status_register
        .set(CpuFlags::NEGATIVE, result.is_negative());
    cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
    cpu.accumulator = result.as_u16();
}

pub fn asl<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.do_rmw(bus, mode, do_asl::<A>);
}

pub fn asl_a<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let result = do_asl(cpu, A::trunc_u16(cpu.accumulator));
    cpu.accumulator = result.as_u16();
}

pub fn bcc(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    do_branch(
        cpu,
        bus,
        mode,
        !cpu.status_register.contains(CpuFlags::CARRY),
    );
}

pub fn bcs(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    do_branch(
        cpu,
        bus,
        mode,
        cpu.status_register.contains(CpuFlags::CARRY),
    );
}

pub fn beq(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    do_branch(cpu, bus, mode, cpu.status_register.contains(CpuFlags::ZERO));
}

pub fn bit<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let operand = cpu.get_operand(bus, mode);
    let result = A::trunc_u16(cpu.accumulator) & operand;
    match mode {
        AddressingMode::Immediate => {
            cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
        }
        _ => {
            cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
            cpu.status_register
                .set(CpuFlags::NEGATIVE, result.is_negative());
            cpu.status_register
                .set(CpuFlags::OVERFLOW, result.is_overflow());
        }
    }
}

pub fn bmi(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    do_branch(
        cpu,
        bus,
        mode,
        cpu.status_register.contains(CpuFlags::NEGATIVE),
    );
}

pub fn bne(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    do_branch(
        cpu,
        bus,
        mode,
        !cpu.status_register.contains(CpuFlags::ZERO),
    );
}

pub fn bpl(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    do_branch(
        cpu,
        bus,
        mode,
        !cpu.status_register.contains(CpuFlags::NEGATIVE),
    );
}

pub fn bra(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    do_branch(cpu, bus, mode, true);
}

pub fn brl(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let offset = cpu.get_operand::<u16>(bus, mode) as i16;
    cpu.program_couter = cpu.program_couter.wrapping_add(offset as u16);
}

pub fn bvc(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    do_branch(
        cpu,
        bus,
        mode,
        !cpu.status_register.contains(CpuFlags::OVERFLOW),
    );
}

pub fn bvs(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    do_branch(
        cpu,
        bus,
        mode,
        cpu.status_register.contains(CpuFlags::OVERFLOW),
    );
}

pub fn brk<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let _thrown = cpu.get_operand::<u8>(bus, mode);
    if !cpu.emulation_mode() {
        cpu.handle_native_interrupt(bus, &NativeVectors::BRK);
    }
}

pub fn cop<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let _thrown = cpu.get_operand::<u8>(bus, mode);
    if !cpu.emulation_mode() {
        cpu.handle_native_interrupt(bus, &NativeVectors::COP);
    }
}

pub fn clc(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.status_register.remove(CpuFlags::CARRY);
}

pub fn cld(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.status_register.remove(CpuFlags::DECIMAL);
}

pub fn cli(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.status_register.remove(CpuFlags::IRQ_DISABLE);
}

pub fn clv(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.status_register.remove(CpuFlags::OVERFLOW);
}

pub fn cmp<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let operand = cpu.get_operand::<A>(bus, mode);
    do_cmp(cpu, A::trunc_u16(cpu.accumulator), operand);
}

pub fn cpx<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let operand = cpu.get_operand::<A>(bus, mode);
    do_cmp(cpu, A::trunc_u16(cpu.index_x), operand);
}

pub fn cpy<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let operand = cpu.get_operand::<A>(bus, mode);
    do_cmp(cpu, A::trunc_u16(cpu.index_y), operand);
}

pub fn dec<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.do_rmw(bus, mode, do_dec::<A>);
}

pub fn dec_a<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let result = do_dec(cpu, A::trunc_u16(cpu.accumulator));
    cpu.accumulator = result.as_u16();
}

pub fn dex<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let result = do_dec(cpu, A::trunc_u16(cpu.index_x));
    cpu.index_x = result.as_u16();
}

pub fn dey<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let result = do_dec(cpu, A::trunc_u16(cpu.index_y));
    cpu.index_y = result.as_u16();
}

pub fn eor<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let operand = cpu.get_operand::<A>(bus, mode);
    let result = A::trunc_u16(cpu.accumulator) ^ operand;
    cpu.status_register
        .set(CpuFlags::NEGATIVE, result.is_negative());
    cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
    cpu.accumulator = result.as_u16();
}

pub fn inc<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.do_rmw(bus, mode, do_inc::<A>);
}

pub fn inc_a<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let result = do_inc(cpu, A::trunc_u16(cpu.accumulator));
    cpu.accumulator = result.as_u16();
}

pub fn inx<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let result = do_inc(cpu, A::trunc_u16(cpu.index_x));
    cpu.index_x = result.as_u16();
}

pub fn iny<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let result = do_inc(cpu, A::trunc_u16(cpu.index_y));
    cpu.index_y = result.as_u16();
}

pub fn jml<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let addr = cpu.get_address::<false>(bus, mode) as u16;
    cpu.program_couter = addr;
    cpu.pbr = ((addr >> 16) & 0xff) as u8;
}

pub fn jmp<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let addr = cpu.get_address::<false>(bus, mode) as u16;
    cpu.program_couter = addr;
}

pub fn jsl<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let addr = cpu.get_address::<false>(bus, mode) as u16;
    do_push(cpu, bus, cpu.pbr);
    do_push(cpu, bus, cpu.program_couter);
    cpu.program_couter = addr;
}

pub fn jsr<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let addr = cpu.get_address::<false>(bus, mode) as u16;
    do_push(cpu, bus, cpu.program_couter);
    cpu.program_couter = addr;
}

pub fn lda<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let operand = cpu.get_operand::<A>(bus, mode);
    cpu.accumulator = operand.as_u16();
}

pub fn ldx<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let operand = cpu.get_operand::<A>(bus, mode);
    cpu.index_x = operand.as_u16();
}

pub fn ldy<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let operand = cpu.get_operand::<A>(bus, mode);
    cpu.index_y = operand.as_u16();
}

pub fn lsr<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.do_rmw(bus, mode, do_lsr::<A>);
}

pub fn lsr_a<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let result = do_lsr(cpu, A::trunc_u16(cpu.accumulator));
    cpu.accumulator = result.as_u16();
}

pub fn mvn(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let banks = cpu.get_operand::<u16>(bus, mode);
    let dst_bank = (banks >> 8) as u8;
    let src_bank = (banks & 0xFF) as u8;
    loop {
        let src = Cpu::read_8(bus, cpu.index_x as u32 | (src_bank as u32) << 16);
        let dst = cpu.index_y as u32 | (dst_bank as u32) << 16;
        Cpu::write_8(bus, dst, src);
        cpu.index_x = cpu.index_x.wrapping_add(1);
        cpu.index_y = cpu.index_y.wrapping_add(1);
        cpu.accumulator = cpu.accumulator.wrapping_sub(1);
        if cpu.accumulator == 0xFFFF {
            break;
        }
    }
}

pub fn mvp(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let banks = cpu.get_operand::<u16>(bus, mode);
    let dst_bank = (banks >> 8) as u8;
    let src_bank = (banks & 0xFF) as u8;
    loop {
        let src = Cpu::read_8(bus, cpu.index_x as u32 | (src_bank as u32) << 16);
        let dst = cpu.index_y as u32 | (dst_bank as u32) << 16;
        Cpu::write_8(bus, dst, src);
        cpu.index_x = cpu.index_x.wrapping_sub(1);
        cpu.index_y = cpu.index_y.wrapping_sub(1);
        cpu.accumulator = cpu.accumulator.wrapping_sub(1);
        if cpu.accumulator == 0xFFFF {
            break;
        }
    }
}

pub fn nop(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {}

pub fn ora<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let operand = cpu.get_operand::<A>(bus, mode);
    let result = A::trunc_u16(cpu.accumulator) | operand;
    cpu.status_register
        .set(CpuFlags::NEGATIVE, result.is_negative());
    cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
    cpu.accumulator = result.as_u16();
}

pub fn pea<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let value = cpu.get_operand::<u16>(bus, mode);
    do_push(cpu, bus, value);
}

pub fn pei<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let addr = cpu.get_address::<false>(bus, mode) as u16;
    do_push(cpu, bus, addr);
}

pub fn per<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let value = cpu.get_operand::<u16>(bus, mode);
    do_push(cpu, bus, cpu.program_couter.wrapping_add(value));
}

pub fn pha<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    do_push(cpu, bus, cpu.accumulator);
}

pub fn phb<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    do_push(cpu, bus, cpu.dbr);
}

pub fn phd<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    do_push(cpu, bus, cpu.dpr);
}

pub fn phk<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    do_push(cpu, bus, cpu.pbr);
}

pub fn php<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    do_push::<u8>(cpu, bus, cpu.status_register.bits());
}

pub fn phx<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    do_push(cpu, bus, cpu.index_x);
}

pub fn phy<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    do_push(cpu, bus, cpu.index_y);
}

pub fn pla<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let result = do_pull::<A>(cpu, bus);
    cpu.accumulator = result.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, result.is_negative());
}

pub fn plb<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let result = do_pull::<u8>(cpu, bus);
    cpu.dbr = result;
    cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, result.is_negative());
}

pub fn pld<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let result = do_pull::<u16>(cpu, bus);
    cpu.dpr = result;
    cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, result.is_negative());
}

pub fn plp<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let result = do_pull::<u8>(cpu, bus);
    cpu.status_register = CpuFlags::from_bits_truncate(result);
}

pub fn plx<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let result = do_pull::<A>(cpu, bus);
    cpu.index_x = result.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, result.is_negative());
}

pub fn ply<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let result = do_pull::<A>(cpu, bus);
    cpu.index_y = result.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, result.is_negative());
}

pub fn rep<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let mask = cpu.get_operand::<u8>(bus, mode);
    let src = cpu.status_register.bits();
    cpu.status_register
        .insert(CpuFlags::from_bits_truncate(src & !mask));
}

pub fn rol<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.do_rmw(bus, mode, do_rol::<A>);
}

pub fn rol_a<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let result = do_rol(cpu, A::trunc_u16(cpu.accumulator));
    cpu.accumulator = result.as_u16();
}

pub fn ror<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.do_rmw(bus, mode, do_ror::<A>);
}

pub fn ror_a<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let result = do_ror(cpu, A::trunc_u16(cpu.accumulator));
    cpu.accumulator = result.as_u16();
}

pub fn rti(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let new_status = do_pull::<u8>(cpu, bus);
    cpu.program_couter = do_pull::<u16>(cpu, bus);
    cpu.pbr = do_pull::<u8>(cpu, bus);
    cpu.status_register
        .insert(CpuFlags::from_bits_truncate(new_status));
}

pub fn rtl(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.program_couter = do_pull::<u16>(cpu, bus).wrapping_add(1);
    cpu.pbr = do_pull::<u8>(cpu, bus);
}

pub fn rts(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.program_couter = do_pull::<u16>(cpu, bus).wrapping_add(1);
}

pub fn sbc<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let operand = cpu.get_operand::<A>(bus, mode);
    if cpu.status_register.contains(CpuFlags::DECIMAL) {
        do_dec_sbc::<A>(cpu, !operand);
    } else {
        do_bin_adc::<A>(cpu, !operand);
    }
}

pub fn sec(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.status_register.insert(CpuFlags::CARRY);
}

pub fn sed(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.status_register.insert(CpuFlags::DECIMAL);
}

pub fn sei(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.status_register.insert(CpuFlags::IRQ_DISABLE);
}

pub fn sep<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let mask = cpu.get_operand::<u8>(bus, mode);
    let src = cpu.status_register.bits();
    cpu.status_register
        .insert(CpuFlags::from_bits_truncate(src | mask));
}

pub fn sta<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.do_write(bus, mode, A::trunc_u16(cpu.accumulator));
}

pub fn stp(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.stopped = true;
}

pub fn stx<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.do_write(bus, mode, A::trunc_u16(cpu.index_x));
}

pub fn sty<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.do_write(bus, mode, A::trunc_u16(cpu.index_y));
}

pub fn stz<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.do_write(bus, mode, A::trunc_u16(0));
}

pub fn tax<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let value = A::trunc_u16(cpu.accumulator);
    cpu.index_x = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
}

pub fn tay<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let value = A::trunc_u16(cpu.accumulator);
    cpu.index_y = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
}

pub fn tcd<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let value = A::trunc_u16(cpu.accumulator);
    cpu.dpr = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
}

pub fn tcs<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let value = A::trunc_u16(cpu.accumulator);
    cpu.stack_pointer = value.as_u16();
}

pub fn tdc<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let value = A::trunc_u16(cpu.dpr);
    cpu.accumulator = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
}

pub fn trb<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.do_rmw(bus, mode, do_trb::<A>);
}

pub fn tsb<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.do_rmw(bus, mode, do_tsb::<A>);
}

pub fn tsc<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let value = A::trunc_u16(cpu.stack_pointer);
    cpu.accumulator = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
}

pub fn tsx<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let value = A::trunc_u16(cpu.stack_pointer);
    cpu.index_x = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
}

pub fn txa<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let value = A::trunc_u16(cpu.index_x);
    cpu.accumulator = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
}

pub fn txs<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let value = A::trunc_u16(cpu.index_x);
    cpu.stack_pointer = value.as_u16();
}

pub fn txy<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let value = A::trunc_u16(cpu.index_x);
    cpu.index_y = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
}

pub fn tya<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let value = A::trunc_u16(cpu.index_y);
    cpu.accumulator = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
}

pub fn tyx<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let value = A::trunc_u16(cpu.index_y);
    cpu.index_x = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
}

pub fn wai(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.waiting_interrupt = true;
}

pub fn wdm(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let _thrown = cpu.get_operand::<u8>(bus, mode);
}

pub fn xba(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    cpu.accumulator = cpu.accumulator.swap_bytes();
    let value = cpu.accumulator as u8;
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
}

pub fn xce(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let carry = cpu.status_register.contains(CpuFlags::CARRY);
    cpu.status_register
        .set(CpuFlags::CARRY, cpu.emulation_mode());
    cpu.set_emulation_mode(carry);
}
