use crate::bus::Bus;

use super::{
    cpu::{AddressingMode, Cpu, CpuFlags},
    functions::*,
    regsize::RegSize,
};

pub fn adc<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let (operand, extra_cycles) = cpu.get_operand::<A>(bus, mode);
    if cpu.status_register.contains(CpuFlags::DECIMAL) {
        do_dec_adc::<A>(cpu, operand);
    } else {
        do_bin_adc::<A>(cpu, operand);
    }
    extra_cycles
}

pub fn and<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let (operand, extra_cycles) = cpu.get_operand::<A>(bus, mode);
    let result = A::trunc_u16(cpu.accumulator) & operand;
    cpu.status_register
        .set(CpuFlags::NEGATIVE, result.is_negative());
    cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
    cpu.accumulator = result.as_u16();
    extra_cycles
}

pub fn asl<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    cpu.do_rmw(bus, mode, do_asl::<A>);
    0
}

pub fn asl_a<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let result = do_asl(cpu, A::trunc_u16(cpu.accumulator));
    cpu.accumulator = result.as_u16();
    0
}

pub fn brk<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    0
}

pub fn cop<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    0
}

pub fn clc(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    cpu.status_register.remove(CpuFlags::CARRY);
    0
}

pub fn cld(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    cpu.status_register.remove(CpuFlags::DECIMAL);
    0
}

pub fn cli(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    todo!();
    0
}

pub fn clv(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    cpu.status_register.remove(CpuFlags::OVERFLOW);
    0
}

pub fn cmp<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let (operand, extra_cycles) = cpu.get_operand::<A>(bus, mode);
    do_cmp(cpu, A::trunc_u16(cpu.accumulator), operand);
    extra_cycles
}

pub fn cpx<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let (operand, extra_cycles) = cpu.get_operand::<A>(bus, mode);
    do_cmp(cpu, A::trunc_u16(cpu.index_x), operand);
    extra_cycles
}

pub fn cpy<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let (operand, extra_cycles) = cpu.get_operand::<A>(bus, mode);
    do_cmp(cpu, A::trunc_u16(cpu.index_y), operand);
    extra_cycles
}

pub fn dec<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    cpu.do_rmw(bus, mode, do_dec::<A>);
    0
}

pub fn dec_a<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let result = do_dec(cpu, A::trunc_u16(cpu.accumulator));
    cpu.accumulator = result.as_u16();
    0
}

pub fn dex<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let result = do_dec(cpu, A::trunc_u16(cpu.index_x));
    cpu.index_x = result.as_u16();
    0
}

pub fn dey<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let result = do_dec(cpu, A::trunc_u16(cpu.index_y));
    cpu.index_y = result.as_u16();
    0
}

pub fn eor<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let (operand, extra_cycles) = cpu.get_operand::<A>(bus, mode);
    let result = A::trunc_u16(cpu.accumulator) ^ operand;
    cpu.status_register
        .set(CpuFlags::NEGATIVE, result.is_negative());
    cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
    cpu.accumulator = result.as_u16();
    extra_cycles
}

pub fn inc<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    cpu.do_rmw(bus, mode, do_inc::<A>);
    0
}

pub fn inc_a<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let result = do_inc(cpu, A::trunc_u16(cpu.accumulator));
    cpu.accumulator = result.as_u16();
    0
}

pub fn inx<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let result = do_inc(cpu, A::trunc_u16(cpu.index_x));
    cpu.index_x = result.as_u16();
    0
}

pub fn iny<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let result = do_inc(cpu, A::trunc_u16(cpu.index_y));
    cpu.index_y = result.as_u16();
    0
}

pub fn lda<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let (operand, extra_cycles) = cpu.get_operand::<A>(bus, mode);
    cpu.accumulator = operand.as_u16();
    extra_cycles
}

pub fn ldx<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let (operand, extra_cycles) = cpu.get_operand::<A>(bus, mode);
    cpu.index_x = operand.as_u16();
    extra_cycles
}

pub fn ldy<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let (operand, extra_cycles) = cpu.get_operand::<A>(bus, mode);
    cpu.index_y = operand.as_u16();
    extra_cycles
}

pub fn lsr<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    cpu.do_rmw(bus, mode, do_lsr::<A>);
    0
}

pub fn lsr_a<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let result = do_lsr(cpu, A::trunc_u16(cpu.accumulator));
    cpu.accumulator = result.as_u16();
    0
}

pub fn nop(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    todo!();
    0
}

pub fn ora<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let (operand, extra_cycles) = cpu.get_operand::<A>(bus, mode);
    let result = A::trunc_u16(cpu.accumulator) | operand;
    cpu.status_register
        .set(CpuFlags::NEGATIVE, result.is_negative());
    cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
    cpu.accumulator = result.as_u16();
    extra_cycles
}

pub fn pea<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let result = bus.read(cpu.program_couter.into()) as u16;
    do_push(cpu, bus, result);
    todo!();
    0
}

pub fn pei<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let addr = cpu.get_address(bus, mode) as u16;
    do_push(cpu, bus, addr);
    0
}

pub fn per<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let addr = cpu.get_address(bus, mode) as u16;
    do_push(cpu, bus, addr);
    0
}

pub fn pha<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    do_push(cpu, bus, cpu.accumulator);
    0
}

pub fn phb<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    do_push(cpu, bus, cpu.dbr);
    0
}

pub fn phd<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    do_push(cpu, bus, cpu.dpr);
    0
}

pub fn phk<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    do_push(cpu, bus, cpu.pbr);
    0
}

pub fn php<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    do_push::<u8>(cpu, bus, cpu.status_register.bits());
    0
}

pub fn phx<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    do_push(cpu, bus, cpu.index_x);
    0
}

pub fn phy<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    do_push(cpu, bus, cpu.index_y);
    0
}

pub fn pla<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let result = do_pull::<A>(cpu, bus);
    cpu.accumulator = result.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, result.is_negative());
    0
}

pub fn plb<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let result = do_pull::<u8>(cpu, bus);
    cpu.dbr = result;
    cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, result.is_negative());
    0
}

pub fn pld<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let result = do_pull::<u16>(cpu, bus);
    cpu.dpr = result;
    cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, result.is_negative());
    0
}

pub fn plp<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    todo!();
    0
}

pub fn plx<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let result = do_pull::<A>(cpu, bus);
    cpu.index_x = result.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, result.is_negative());
    0
}

pub fn ply<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let result = do_pull::<A>(cpu, bus);
    cpu.index_y = result.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, result.is_negative());
    0
}

pub fn rol<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    cpu.do_rmw(bus, mode, do_rol::<A>);
    0
}

pub fn rol_a<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let result = do_rol(cpu, A::trunc_u16(cpu.accumulator));
    cpu.accumulator = result.as_u16();
    0
}

pub fn ror<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    cpu.do_rmw(bus, mode, do_ror::<A>);
    0
}

pub fn ror_a<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let result = do_ror(cpu, A::trunc_u16(cpu.accumulator));
    cpu.accumulator = result.as_u16();
    0
}

pub fn sbc<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let (operand, extra_cycles) = cpu.get_operand::<A>(bus, mode);
    if cpu.status_register.contains(CpuFlags::DECIMAL) {
        do_dec_sbc::<A>(cpu, !operand);
    } else {
        do_bin_adc::<A>(cpu, !operand);
    }
    extra_cycles
}

pub fn sec(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    cpu.status_register.insert(CpuFlags::CARRY);
    0
}

pub fn sed(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    cpu.status_register.insert(CpuFlags::DECIMAL);
    0
}

pub fn sei(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    todo!();
    0
}

pub fn sta<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let data = A::trunc_u16(cpu.accumulator);
    let addr = cpu.get_address(bus, mode);
    if A::IS_U16 {
        Cpu::write_16(bus, addr, data.as_u16());
    } else {
        Cpu::write_8(&bus, addr, data.as_u8());
    }
    0
}

pub fn stx<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let data = A::trunc_u16(cpu.index_x);
    let addr = cpu.get_address(bus, mode);
    if A::IS_U16 {
        Cpu::write_16(bus, addr, data.as_u16());
    } else {
        Cpu::write_8(&bus, addr, data.as_u8());
    }
    0
}

pub fn sty<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let data = A::trunc_u16(cpu.index_y);
    let addr = cpu.get_address(bus, mode);
    if A::IS_U16 {
        Cpu::write_16(bus, addr, data.as_u16());
    } else {
        Cpu::write_8(&bus, addr, data.as_u8());
    }
    0
}

pub fn stz<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let data = A::trunc_u16(0);
    let addr = cpu.get_address(bus, mode);
    if A::IS_U16 {
        Cpu::write_16(bus, addr, data.as_u16());
    } else {
        Cpu::write_8(&bus, addr, data.as_u8());
    }
    0
}

pub fn tax<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let value = A::trunc_u16(cpu.accumulator);
    cpu.index_x = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
    0
}

pub fn tay<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let value = A::trunc_u16(cpu.accumulator);
    cpu.index_y = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
    0
}

pub fn tcd<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let value = A::trunc_u16(cpu.accumulator);
    cpu.dpr = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
    0
}

pub fn tcs<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let value = A::trunc_u16(cpu.accumulator);
    cpu.stack_pointer = value.as_u16();
    0
}

pub fn tdc<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let value = A::trunc_u16(cpu.dpr);
    cpu.accumulator = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
    0
}

pub fn trb<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    cpu.do_rmw(bus, mode, do_trb::<A>);
    0
}

pub fn tsb<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    cpu.do_rmw(bus, mode, do_tsb::<A>);
    0
}

pub fn tsc<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let value = A::trunc_u16(cpu.stack_pointer);
    cpu.accumulator = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
    0
}

pub fn tsx<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let value = A::trunc_u16(cpu.stack_pointer);
    cpu.index_x = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
    0
}

pub fn txa<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let value = A::trunc_u16(cpu.index_x);
    cpu.accumulator = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
    0
}

pub fn txs<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let value = A::trunc_u16(cpu.index_x);
    cpu.stack_pointer = value.as_u16();
    0
}

pub fn txy<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let value = A::trunc_u16(cpu.index_x);
    cpu.index_y = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
    0
}

pub fn tya<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let value = A::trunc_u16(cpu.index_y);
    cpu.accumulator = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
    0
}

pub fn tyx<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let value = A::trunc_u16(cpu.index_y);
    cpu.index_x = value.as_u16();
    cpu.status_register.set(CpuFlags::ZERO, value.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, value.is_negative());
    0
}
