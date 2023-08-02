use crate::bus::Bus;

use super::{
    cpu::{AddressingMode, Cpu, CpuFlags},
    regsize::RegSize,
};

fn do_bin_adc<A: RegSize>(cpu: &mut Cpu, operand: A) {
    if A::IS_U16 {
        let src = cpu.accumulator;
        let operand = operand.as_u16();
        let result = src + operand + cpu.status_register.contains(CpuFlags::CARRY) as u16;
        let is_overflow = !(src ^ operand) & (src ^ result) & 1 << 15 != 0;

        cpu.status_register.set(CpuFlags::CARRY, result >> 15 != 0);
        cpu.status_register.set(CpuFlags::OVERFLOW, is_overflow);
        cpu.status_register
            .set(CpuFlags::NEGATIVE, result.is_negative());
        cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
        cpu.accumulator = result;
    } else {
        let src = cpu.accumulator & 0xFF;
        let operand = operand.as_u16();
        let result = src + operand + cpu.status_register.contains(CpuFlags::CARRY) as u16;
        let is_overflow = !(src ^ operand) & (src ^ result) & 1 << 7 != 0;
        cpu.status_register.set(CpuFlags::CARRY, result >> 8 != 0);
        cpu.status_register.set(CpuFlags::OVERFLOW, is_overflow);
        cpu.status_register
            .set(CpuFlags::NEGATIVE, result.is_negative());
        cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
        cpu.set_low_a(result);
    }
}

fn do_dec_adc<A: RegSize>(cpu: &mut Cpu, operand: A) {
    if A::IS_U16 {
        let src = cpu.accumulator;
        let operand = operand.as_u16();
        let mut result =
            (src & 0xF) + (operand & 0xF) + cpu.status_register.contains(CpuFlags::CARRY) as u16;
        if result > 9 {
            result += 6;
        }
        result = (src & 0xF0) + (operand & 0xF0) + (result & 0xF) + (((result > 0xF) as u16) << 4);
        if result > 0x9F {
            result += 0x60;
        }
        result =
            (src & 0xF00) + (operand & 0xF00) + (result & 0xFF) + (((result > 0xFF) as u16) << 8);
        if result > 0x9FF {
            result += 0x600;
        }
        result = (src & 0xF000)
            + (operand & 0xF000)
            + (result & 0xFFF)
            + (((result > 0xFFF) as u16) << 12);
        let is_overflow = !(src ^ operand) & (src ^ result) & 1 << 15 != 0;
        cpu.status_register.set(CpuFlags::OVERFLOW, is_overflow);
        if result > 0x9FFF {
            result += 0x6000;
        }
        cpu.status_register.set(CpuFlags::CARRY, result >> 15 != 0);
        cpu.status_register
            .set(CpuFlags::NEGATIVE, result.is_negative());
        cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
        cpu.accumulator = result;
    } else {
        let src = cpu.accumulator & 0xFF;
        let operand = operand.as_u16();
        let mut result =
            (src & 0xF) + (operand & 0xF) + cpu.status_register.contains(CpuFlags::CARRY) as u16;
        if result > 9 {
            result += 6;
        }
        result = (src & 0xF0) + (operand & 0xF0) + (result & 0xF) + (((result > 0xF) as u16) << 4);
        let is_overflow = !(src ^ operand) & (src ^ result) & 1 << 7 != 0;
        cpu.status_register.set(CpuFlags::OVERFLOW, is_overflow);
        if result > 0x9F {
            result += 0x60;
        }
        cpu.status_register.set(CpuFlags::CARRY, result >> 8 != 0);
        cpu.status_register
            .set(CpuFlags::NEGATIVE, result.is_negative());
        cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
        cpu.set_low_a(result);
    }
}

fn do_asl<A: RegSize>(cpu: &mut Cpu, operand: A) -> A {
    if A::IS_U16 {
        let operand = operand.as_u16();
        let result = operand << 1;
        cpu.status_register.set(CpuFlags::CARRY, result >> 15 != 0);
        cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
        cpu.status_register
            .set(CpuFlags::NEGATIVE, result.is_negative());
        A::trunc_u16(result)
    } else {
        let operand = operand.as_u8();
        let result = operand << 1;
        cpu.status_register.set(CpuFlags::CARRY, result >> 7 != 0);
        cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
        cpu.status_register
            .set(CpuFlags::NEGATIVE, result.is_negative());
        A::ext_u8(result)
    }
}

fn do_cmp<A: RegSize>(cpu: &mut Cpu, src: A, operand: A) {
    let result = src.wrapping_sub(operand);
    cpu.status_register.set(CpuFlags::CARRY, src >= operand);
    cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, result.is_negative());
}

fn do_dec<A: RegSize>(cpu: &mut Cpu, src: A) -> A {
    let result = src.wrapping_sub(A::ext_u8(1));
    cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
    cpu.status_register
        .set(CpuFlags::NEGATIVE, result.is_negative());
    result
}

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
    let result = operand & A::trunc_u16(cpu.accumulator);
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

pub fn ora<A: RegSize>(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let (operand, extra_cycles) = cpu.get_operand::<A>(bus, mode);
    let result = operand | A::trunc_u16(cpu.accumulator);
    cpu.status_register
        .set(CpuFlags::NEGATIVE, result.is_negative());
    cpu.status_register.set(CpuFlags::ZERO, result.is_zero());
    cpu.accumulator = result.as_u16();
    extra_cycles
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
