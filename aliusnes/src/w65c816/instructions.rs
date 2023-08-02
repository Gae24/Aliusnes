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

// pub fn tsb<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
//     let (addr, extra_cycles) = cpu.get_operand_address(
//         bus,
//         mode,
//         cpu.status_register.contains(CpuFlags::A_REG_SIZE),
//     );
//     if cpu.status_register.contains(CpuFlags::A_REG_SIZE) {
//         let data = bus.read(addr);
//         let result = data | (cpu.accumulator as u8);
//         bus.write(addr, result);
//         cpu.status_register.set(CpuFlags::ZERO, result == 0x00);
//     } else {
//         let data = bus.read_16bit(addr);
//         let result = data | cpu.accumulator;
//         bus.write_16bit(addr, result);
//         cpu.status_register.set(CpuFlags::ZERO, result == 0x00);
//     }
//     let (operand, extra_cycles) = cpu.get_operand::<A>(bus, mode);
//     extra_cycles
// }
