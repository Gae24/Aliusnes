use crate::bus::Bus;

use super::cpu::{AddressingMode, Cpu, CpuFlags};

pub fn brk(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    0
}

pub fn ora(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let (addr, extra_cycles) = cpu.get_operand_address(bus, mode, cpu.status_register.contains(CpuFlags::A_REG_SIZE));
    if cpu.status_register.contains(CpuFlags::A_REG_SIZE) {
        let data = bus.read(addr);
        let result = data | (cpu.accumulator as u8);
        cpu.set_low_a(result);
        cpu.status_register
            .set(CpuFlags::NEGATIVE, result & 0x80 == 0x80);
        cpu.status_register.set(CpuFlags::ZERO, result == 0x00);
    } else {
        let data = bus.read_16bit(addr);
        cpu.accumulator |= data;
        cpu.status_register
            .set(CpuFlags::NEGATIVE, cpu.accumulator & 0x80 == 0x80);
        cpu.status_register
            .set(CpuFlags::ZERO, cpu.accumulator == 0x00);
    }
    extra_cycles
}

pub fn cop(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    0
}

pub fn tsb(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    let (addr, extra_cycles) = cpu.get_operand_address(bus,mode, cpu.status_register.contains(CpuFlags::A_REG_SIZE));
    if cpu.status_register.contains(CpuFlags::A_REG_SIZE) {
        let data = bus.read(addr);
        let result = data | (cpu.accumulator as u8);
        bus.write(addr, result);
        cpu.status_register.set(CpuFlags::ZERO, result == 0x00);
    } else {
        let data = bus.read_16bit(addr);
        let result = data | cpu.accumulator;
        bus.write_16bit(addr, result);
        cpu.status_register.set(CpuFlags::ZERO, result == 0x00);
    }
    extra_cycles
}

pub fn asl(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) -> u8 {
    0
}
