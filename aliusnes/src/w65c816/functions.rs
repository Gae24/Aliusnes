use super::{
    cpu::{AddressingMode, Cpu, CpuFlags},
    regsize::RegSize,
};
use crate::bus::bus::Bus;

pub(super) fn do_bin_adc<T: RegSize>(cpu: &mut Cpu, operand: T) {
    if T::IS_U16 {
        let src = cpu.accumulator as u32;
        let operand = operand.as_u16() as u32;
        let result = src + operand + cpu.status_register.contains(CpuFlags::CARRY) as u32;
        let is_overflow = !(src ^ operand) & (src ^ result) & 1 << 15 != 0;
        cpu.status_register.set(CpuFlags::CARRY, result >> 16 != 0);
        cpu.status_register.set(CpuFlags::OVERFLOW, is_overflow);
        let result = result as u16;
        cpu.set_nz(result);
        cpu.accumulator = result;
    } else {
        let src = cpu.accumulator & 0xFF;
        let operand = operand.as_u16();
        let result = src + operand + cpu.status_register.contains(CpuFlags::CARRY) as u16;
        let is_overflow = !(src ^ operand) & (src ^ result) & 1 << 7 != 0;
        cpu.status_register.set(CpuFlags::CARRY, result >> 8 != 0);
        cpu.status_register.set(CpuFlags::OVERFLOW, is_overflow);
        let result = result as u8;
        cpu.set_nz(result);
        cpu.set_accumulator(result);
    }
}

pub(super) fn do_dec_adc<T: RegSize>(cpu: &mut Cpu, operand: T) {
    if T::IS_U16 {
        let src = cpu.accumulator as u32;
        let operand = operand.as_u16() as u32;
        let mut result =
            (src & 0xF) + (operand & 0xF) + cpu.status_register.contains(CpuFlags::CARRY) as u32;
        if result > 9 {
            result += 6;
        }
        result = (src & 0xF0) + (operand & 0xF0) + (result & 0xF) + (((result > 0xF) as u32) << 4);
        if result > 0x9F {
            result += 0x60;
        }
        result =
            (src & 0xF00) + (operand & 0xF00) + (result & 0xFF) + (((result > 0xFF) as u32) << 8);
        if result > 0x9FF {
            result += 0x600;
        }
        result = (src & 0xF000)
            + (operand & 0xF000)
            + (result & 0xFFF)
            + (((result > 0xFFF) as u32) << 12);
        let is_overflow = !(src ^ operand) & (src ^ result) & 1 << 15 != 0;
        cpu.status_register.set(CpuFlags::OVERFLOW, is_overflow);
        if result > 0x9FFF {
            result += 0x6000;
        }
        cpu.status_register.set(CpuFlags::CARRY, result >> 16 != 0);
        let result = result as u16;
        cpu.set_nz(result);
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
        let result = result as u8;
        cpu.set_nz(result);
        cpu.set_accumulator(result);
    }
}

pub(super) fn do_asl<T: RegSize>(cpu: &mut Cpu, operand: T) -> T {
    if T::IS_U16 {
        let operand = operand.as_u16();
        let result = operand << 1;
        cpu.status_register.set(CpuFlags::CARRY, operand >> 15 != 0);
        cpu.set_nz(result);
        T::trunc_u16(result)
    } else {
        let operand = operand.as_u8();
        let result = operand << 1;
        cpu.status_register.set(CpuFlags::CARRY, operand >> 7 != 0);
        cpu.set_nz(result);
        T::ext_u8(result)
    }
}

pub(super) fn do_branch(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode, cond: bool) {
    let offset = cpu.get_operand::<u8>(bus, mode) as i8;
    if cond {
        cpu.extra_cycles += 1;
        cpu.program_counter = cpu.program_counter.wrapping_add(offset as u16);
    }
}

pub(super) fn do_cmp<T: RegSize>(cpu: &mut Cpu, src: T, operand: T) {
    let result = src.wrapping_sub(operand);
    cpu.status_register.set(CpuFlags::CARRY, src >= operand);
    cpu.set_nz(result);
}

pub(super) fn do_dec<T: RegSize>(cpu: &mut Cpu, src: T) -> T {
    let result = src.wrapping_sub(T::ext_u8(1));
    cpu.set_nz(result);
    result
}

pub(super) fn do_inc<T: RegSize>(cpu: &mut Cpu, src: T) -> T {
    let result = src.wrapping_add(T::ext_u8(1));
    cpu.set_nz(result);
    result
}

pub(super) fn do_lsr<T: RegSize>(cpu: &mut Cpu, operand: T) -> T {
    if T::IS_U16 {
        let operand = operand.as_u16();
        let result = operand >> 1;
        cpu.status_register.set(CpuFlags::CARRY, operand & 1 != 0);
        cpu.set_nz(result);
        T::trunc_u16(result)
    } else {
        let operand = operand.as_u8();
        let result = operand >> 1;
        cpu.status_register.set(CpuFlags::CARRY, operand & 1 != 0);
        cpu.set_nz(result);
        T::ext_u8(result)
    }
}

pub(super) fn do_push<T: RegSize>(cpu: &mut Cpu, bus: &mut Bus, value: T) {
    if T::IS_U16 {
        cpu.extra_cycles += 1;
        cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(2);
        Cpu::write_16(
            bus,
            cpu.stack_pointer.wrapping_add(1).into(),
            value.as_u16(),
        );
    } else {
        cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(1);
        Cpu::write_8(bus, cpu.stack_pointer.wrapping_add(1).into(), value.as_u8());
    }
}

pub(super) fn do_pull<T: RegSize>(cpu: &mut Cpu, bus: &Bus) -> T {
    if T::IS_U16 {
        cpu.extra_cycles += 1;
        let low = Cpu::read_8(bus, cpu.stack_pointer.wrapping_add(1).into());
        let high = Cpu::read_8(bus, cpu.stack_pointer.wrapping_add(2).into());
        cpu.stack_pointer = cpu.stack_pointer.wrapping_add(2);
        T::trunc_u16(low as u16 | (high as u16) << 8)
    } else {
        let value = Cpu::read_8(bus, cpu.stack_pointer.wrapping_add(1).into());
        cpu.stack_pointer = cpu.stack_pointer.wrapping_add(1);
        T::ext_u8(value)
    }
}

pub(super) fn do_rol<T: RegSize>(cpu: &mut Cpu, operand: T) -> T {
    if T::IS_U16 {
        let operand = operand.as_u16();
        let result = operand << 1 | cpu.status_register.contains(CpuFlags::CARRY) as u16;
        cpu.status_register.set(CpuFlags::CARRY, operand >> 15 != 0);
        cpu.set_nz(result);
        T::trunc_u16(result)
    } else {
        let operand = operand.as_u8();
        let result = operand << 1 | cpu.status_register.contains(CpuFlags::CARRY) as u8;
        cpu.status_register.set(CpuFlags::CARRY, operand >> 7 != 0);
        cpu.set_nz(result);
        T::ext_u8(result)
    }
}

pub(super) fn do_ror<T: RegSize>(cpu: &mut Cpu, operand: T) -> T {
    if T::IS_U16 {
        let operand = operand.as_u16();
        let result = operand >> 1 | (cpu.status_register.contains(CpuFlags::CARRY) as u16) << 15;
        cpu.status_register.set(CpuFlags::CARRY, operand & 1 != 0);
        cpu.set_nz(result);
        T::trunc_u16(result)
    } else {
        let operand = operand.as_u8();
        let result = operand >> 1 | (cpu.status_register.contains(CpuFlags::CARRY) as u8) << 7;
        cpu.status_register.set(CpuFlags::CARRY, operand & 1 != 0);
        cpu.set_nz(result);
        T::ext_u8(result)
    }
}

pub(super) fn do_dec_sbc<T: RegSize>(cpu: &mut Cpu, operand: T) {
    if T::IS_U16 {
        let src = cpu.accumulator as i32;
        let operand = operand.as_u16() as i32;
        let mut result =
            (src & 0xF) + (operand & 0xF) + cpu.status_register.contains(CpuFlags::CARRY) as i32;
        if result <= 0xF {
            result -= 6;
        }
        result = (src & 0xF0) + (operand & 0xF0) + (result & 0xF) + (((result > 0xF) as i32) << 4);
        if result <= 0xFF {
            result -= 0x60;
        }
        result =
            (src & 0xF00) + (operand & 0xF00) + (result & 0xFF) + (((result > 0xFF) as i32) << 8);
        if result <= 0xFFF {
            result -= 0x600;
        }
        result = (src & 0xF000)
            + (operand & 0xF000)
            + (result & 0xFFF)
            + (((result > 0xFFF) as i32) << 12);
        let is_overflow = !(src ^ operand) & (src ^ result) & 1 << 15 != 0;
        cpu.status_register.set(CpuFlags::OVERFLOW, is_overflow);
        if result <= 0xFFFF {
            result = result.wrapping_sub(0x6000);
        }
        cpu.status_register.set(CpuFlags::CARRY, result > 0xFFFF);
        let result = result as u16;
        cpu.set_nz(result);
        cpu.accumulator = result;
    } else {
        let src = cpu.accumulator as i16 & 0xFF;
        let operand = operand.as_u16() as i16;
        let mut result =
            (src & 0xF) + (operand & 0xF) + cpu.status_register.contains(CpuFlags::CARRY) as i16;
        if result <= 0xF {
            result = result.wrapping_sub(6);
        }
        result = (src & 0xF0) + (operand & 0xF0) + (result & 0xF) + (((result > 0xF) as i16) << 4);
        let is_overflow = !(src ^ operand) & (src ^ result) & 1 << 7 != 0;
        cpu.status_register.set(CpuFlags::OVERFLOW, is_overflow);
        if result <= 0xFF {
            result = result.wrapping_sub(0x60);
        }
        cpu.status_register.set(CpuFlags::CARRY, result > 0xFF);
        let result = result as u8;
        cpu.set_nz(result);
        cpu.set_accumulator(result);
    }
}

pub(super) fn do_trb<T: RegSize>(cpu: &mut Cpu, operand: T) -> T {
    let a = T::trunc_u16(cpu.accumulator);
    cpu.status_register
        .set(CpuFlags::ZERO, (operand & a).is_zero());
    operand & !a
}

pub(super) fn do_tsb<T: RegSize>(cpu: &mut Cpu, operand: T) -> T {
    let a = T::trunc_u16(cpu.accumulator);
    cpu.status_register
        .set(CpuFlags::ZERO, (operand & a).is_zero());
    operand | a
}
