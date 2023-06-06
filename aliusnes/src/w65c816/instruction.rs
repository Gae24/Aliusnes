use crate::bus::Bus;

use super::cpu::{AddressingMode, Cpu, CpuFlags};

pub fn brk(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {}

pub fn ora(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode);
    if cpu.status.contains(CpuFlags::A_REG_SIZE) {
        let data = bus.read(addr);
        let result = data | (cpu.register_a as u8);
        cpu.set_low_a(result);
        cpu.status.set(CpuFlags::NEGATIVE, result & 0x80 == 0x80);
        cpu.status.set(CpuFlags::ZERO, result == 0x00);
    } else {
        let data = bus.read_16bit(addr);
        cpu.register_a |= data;
        cpu.status.set(CpuFlags::NEGATIVE, cpu.register_a & 0x80 == 0x80);
        cpu.status.set(CpuFlags::ZERO, cpu.register_a == 0x00);
    }
}

pub fn cop(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {}

pub fn tsb(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode);
    if cpu.status.contains(CpuFlags::A_REG_SIZE) {
        let data = bus.read(addr);
        let result = data | (cpu.register_a as u8);
        bus.write(addr, result);
        cpu.status.set(CpuFlags::ZERO, result == 0x00);
    } else {
        let data = bus.read_16bit(addr);
        let result = data | cpu.register_a;
        bus.write_16bit(addr, result);
        cpu.status.set(CpuFlags::ZERO, result == 0x00);
    }
}

pub fn asl(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode) {}
