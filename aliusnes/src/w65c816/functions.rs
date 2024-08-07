use super::{
    addressing::{Address, AddressingMode},
    cpu::Cpu,
    regsize::RegSize,
};
use crate::{bus::Bus, utils::int_traits::ManipulateU16};

pub(super) fn do_bin_adc<T: RegSize>(cpu: &mut Cpu, operand: T) {
    if T::IS_U16 {
        let src = u32::from(cpu.accumulator);
        let operand = u32::from(operand.as_u16());
        let result = src + operand + u32::from(cpu.status.carry());
        let is_overflow = !(src ^ operand) & (src ^ result) & 1 << 15 != 0;
        cpu.status.set_carry(result >> 16 != 0);
        cpu.status.set_overflow(is_overflow);
        let result = result as u16;
        cpu.set_nz(result);
        cpu.accumulator = result;
    } else {
        let src = cpu.accumulator & 0xFF;
        let operand = operand.as_u16();
        let result = src + operand + u16::from(cpu.status.carry());
        let is_overflow = !(src ^ operand) & (src ^ result) & 1 << 7 != 0;
        cpu.status.set_carry(result >> 8 != 0);
        cpu.status.set_overflow(is_overflow);
        cpu.set_nz(result.low_byte());
        cpu.set_accumulator(result.low_byte());
    }
}

pub(super) fn do_dec_adc<T: RegSize>(cpu: &mut Cpu, operand: T) {
    if T::IS_U16 {
        let src = u32::from(cpu.accumulator);
        let operand = u32::from(operand.as_u16());
        let mut result = (src & 0xF) + (operand & 0xF) + u32::from(cpu.status.carry());
        if result > 9 {
            result += 6;
        }
        result = (src & 0xF0) + (operand & 0xF0) + (result & 0xF) + (u32::from(result > 0xF) << 4);
        if result > 0x9F {
            result += 0x60;
        }
        result =
            (src & 0xF00) + (operand & 0xF00) + (result & 0xFF) + (u32::from(result > 0xFF) << 8);
        if result > 0x9FF {
            result += 0x600;
        }
        result = (src & 0xF000)
            + (operand & 0xF000)
            + (result & 0xFFF)
            + (u32::from(result > 0xFFF) << 12);
        let is_overflow = !(src ^ operand) & (src ^ result) & 1 << 15 != 0;
        cpu.status.set_overflow(is_overflow);
        if result > 0x9FFF {
            result += 0x6000;
        }
        cpu.status.set_carry(result >> 16 != 0);
        let result = result as u16;
        cpu.set_nz(result);
        cpu.accumulator = result;
    } else {
        let src = cpu.accumulator & 0xFF;
        let operand = operand.as_u16();
        let mut result = (src & 0xF) + (operand & 0xF) + u16::from(cpu.status.carry());
        if result > 9 {
            result += 6;
        }
        result = (src & 0xF0) + (operand & 0xF0) + (result & 0xF) + (u16::from(result > 0xF) << 4);
        let is_overflow = !(src ^ operand) & (src ^ result) & 1 << 7 != 0;
        cpu.status.set_overflow(is_overflow);
        if result > 0x9F {
            result += 0x60;
        }
        cpu.status.set_carry(result >> 8 != 0);
        cpu.set_nz(result.low_byte());
        cpu.set_accumulator(result.low_byte());
    }
}

pub(super) fn do_asl<T: RegSize>(cpu: &mut Cpu, operand: T) -> T {
    if T::IS_U16 {
        let operand = operand.as_u16();
        let result = operand << 1;
        cpu.status.set_carry(operand >> 15 != 0);
        cpu.set_nz(result);
        T::from_u16(result)
    } else {
        let operand = operand.as_u8();
        let result = operand << 1;
        cpu.status.set_carry(operand >> 7 != 0);
        cpu.set_nz(result);
        T::from_u8(result)
    }
}

pub(super) fn do_bit<T: RegSize>(cpu: &mut Cpu, operand: T, mode: AddressingMode) {
    let result = T::from_u16(cpu.accumulator) & operand;
    if mode == AddressingMode::Immediate {
        cpu.status.set_zero(result.is_zero());
    } else {
        cpu.status.set_negative(operand.is_negative());
        cpu.status.set_overflow(operand.is_overflow());
        cpu.status.set_zero(result.is_zero());
    }
}

pub(super) fn do_block_move<T: RegSize, B: Bus>(cpu: &mut Cpu, bus: &mut B, op: fn(T, T) -> T) {
    let banks = cpu.get_operand::<u16, B>(bus, &AddressingMode::BlockMove);
    let src_bank = banks.high_byte();
    cpu.dbr = banks.low_byte();
    let src = Address::new(cpu.index_x, src_bank);
    let dst = Address::new(cpu.index_y, cpu.dbr);

    let val = bus.read_and_tick(src);
    bus.write_and_tick(dst, val);
    bus.add_io_cycles(2);

    cpu.index_x = op(T::from_u16(cpu.index_x), T::from_u8(1)).as_u16();
    cpu.index_y = op(T::from_u16(cpu.index_y), T::from_u8(1)).as_u16();
    cpu.accumulator = cpu.accumulator.wrapping_sub(1);
    if cpu.accumulator != 0xFFFF {
        cpu.program_counter = cpu.program_counter.wrapping_sub(3);
    }
}

pub(super) fn do_branch<B: Bus>(cpu: &mut Cpu, bus: &mut B, mode: AddressingMode, cond: bool) {
    let offset = cpu.get_operand::<u8, B>(bus, &mode) as i8;
    if cond {
        bus.add_io_cycles(1);
        cpu.program_counter = cpu.program_counter.wrapping_add(offset as u16);
    }
}

pub(super) fn do_cmp<T: RegSize>(cpu: &mut Cpu, src: T, operand: T) {
    let result = src.wrapping_sub(operand);
    cpu.status.set_carry(src >= operand);
    cpu.set_nz(result);
}

pub(super) fn do_dec<T: RegSize>(cpu: &mut Cpu, src: T) -> T {
    let result = src.wrapping_sub(T::from_u8(1));
    cpu.set_nz(result);
    result
}

pub(super) fn do_inc<T: RegSize>(cpu: &mut Cpu, src: T) -> T {
    let result = src.wrapping_add(T::from_u8(1));
    cpu.set_nz(result);
    result
}

pub(super) fn do_lsr<T: RegSize>(cpu: &mut Cpu, operand: T) -> T {
    if T::IS_U16 {
        let operand = operand.as_u16();
        let result = operand >> 1;
        cpu.status.set_carry(operand & 1 != 0);
        cpu.set_nz(result);
        T::from_u16(result)
    } else {
        let operand = operand.as_u8();
        let result = operand >> 1;
        cpu.status.set_carry(operand & 1 != 0);
        cpu.set_nz(result);
        T::from_u8(result)
    }
}

pub(super) fn do_push<T: RegSize, B: Bus>(cpu: &mut Cpu, bus: &mut B, value: T) {
    if T::IS_U16 {
        cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(2);
        cpu.write_bank0(bus, cpu.stack_pointer.wrapping_add(1), value.as_u16());
    } else {
        cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(1);
        bus.write_and_tick(cpu.stack_pointer.wrapping_add(1).into(), value.as_u8());
    }
}

pub(super) fn do_pull<T: RegSize, B: Bus>(cpu: &mut Cpu, bus: &mut B) -> T {
    if T::IS_U16 {
        let value = cpu.read_bank0(bus, cpu.stack_pointer.wrapping_add(1));
        cpu.stack_pointer = cpu.stack_pointer.wrapping_add(2);
        T::from_u16(value)
    } else {
        let value = bus.read_and_tick(cpu.stack_pointer.wrapping_add(1).into());
        cpu.stack_pointer = cpu.stack_pointer.wrapping_add(1);
        T::from_u8(value)
    }
}

pub(super) fn do_rol<T: RegSize>(cpu: &mut Cpu, operand: T) -> T {
    if T::IS_U16 {
        let operand = operand.as_u16();
        let result = operand << 1 | u16::from(cpu.status.carry());
        cpu.status.set_carry(operand >> 15 != 0);
        cpu.set_nz(result);
        T::from_u16(result)
    } else {
        let operand = operand.as_u8();
        let result = operand << 1 | u8::from(cpu.status.carry());
        cpu.status.set_carry(operand >> 7 != 0);
        cpu.set_nz(result);
        T::from_u8(result)
    }
}

pub(super) fn do_ror<T: RegSize>(cpu: &mut Cpu, operand: T) -> T {
    if T::IS_U16 {
        let operand = operand.as_u16();
        let result = operand >> 1 | u16::from(cpu.status.carry()) << 15;
        cpu.status.set_carry(operand & 1 != 0);
        cpu.set_nz(result);
        T::from_u16(result)
    } else {
        let operand = operand.as_u8();
        let result = operand >> 1 | u8::from(cpu.status.carry()) << 7;
        cpu.status.set_carry(operand & 1 != 0);
        cpu.set_nz(result);
        T::from_u8(result)
    }
}

pub(super) fn do_dec_sbc<T: RegSize>(cpu: &mut Cpu, operand: T) {
    if T::IS_U16 {
        let src = i32::from(cpu.accumulator);
        let operand = i32::from(operand.as_u16());
        let mut result = (src & 0xF) + (operand & 0xF) + i32::from(cpu.status.carry());
        if result <= 0xF {
            result -= 6;
        }
        result = (src & 0xF0) + (operand & 0xF0) + (result & 0xF) + (i32::from(result > 0xF) << 4);
        if result <= 0xFF {
            result -= 0x60;
        }
        result =
            (src & 0xF00) + (operand & 0xF00) + (result & 0xFF) + (i32::from(result > 0xFF) << 8);
        if result <= 0xFFF {
            result -= 0x600;
        }
        result = (src & 0xF000)
            + (operand & 0xF000)
            + (result & 0xFFF)
            + (i32::from(result > 0xFFF) << 12);
        let is_overflow = !(src ^ operand) & (src ^ result) & 1 << 15 != 0;
        cpu.status.set_overflow(is_overflow);
        if result <= 0xFFFF {
            result = result.wrapping_sub(0x6000);
        }
        cpu.status.set_carry(result > 0xFFFF);
        let result = result as u16;
        cpu.set_nz(result);
        cpu.accumulator = result;
    } else {
        let src = cpu.accumulator as i16 & 0xFF;
        let operand = operand.as_u16() as i16;
        let mut result = (src & 0xF) + (operand & 0xF) + i16::from(cpu.status.carry());
        if result <= 0xF {
            result = result.wrapping_sub(6);
        }
        result = (src & 0xF0) + (operand & 0xF0) + (result & 0xF) + (i16::from(result > 0xF) << 4);
        let is_overflow = !(src ^ operand) & (src ^ result) & 1 << 7 != 0;
        cpu.status.set_overflow(is_overflow);
        if result <= 0xFF {
            result = result.wrapping_sub(0x60);
        }
        cpu.status.set_carry(result > 0xFF);
        let result = result as u8;
        cpu.set_nz(result);
        cpu.set_accumulator(result);
    }
}

pub(super) fn do_store<T: RegSize, B: Bus>(
    cpu: &mut Cpu,
    bus: &mut B,
    mode: AddressingMode,
    val: T,
) {
    match mode {
        AddressingMode::Direct
        | AddressingMode::DirectX
        | AddressingMode::DirectY
        | AddressingMode::StackRelative => {
            let addr = cpu.direct_page_address(bus, &mode);
            if T::IS_U16 {
                cpu.write_bank0(bus, addr, val.as_u16());
            } else {
                bus.write_and_tick(Address::new(addr, 0), val.as_u8());
            }
        }
        _ => {
            let addr = cpu.decode_addressing_mode::<true, B>(bus, mode);
            if T::IS_U16 {
                cpu.write_16(bus, addr, val.as_u16());
            } else {
                bus.write_and_tick(addr, val.as_u8());
            }
        }
    }
}

pub(super) fn do_trb<T: RegSize>(cpu: &mut Cpu, operand: T) -> T {
    let a = T::from_u16(cpu.accumulator);
    cpu.status.set_zero((operand & a).is_zero());
    operand & !a
}

pub(super) fn do_tsb<T: RegSize>(cpu: &mut Cpu, operand: T) -> T {
    let a = T::from_u16(cpu.accumulator);
    cpu.status.set_zero((operand & a).is_zero());
    operand | a
}
