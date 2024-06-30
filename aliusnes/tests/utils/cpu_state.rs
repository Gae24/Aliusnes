use aliusnes::w65c816::{
    cpu::{Cpu, Status},
    W65C816,
};

use super::test_bus::TomHarteBus;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct CpuState {
    pub pc: u16,
    s: u16,
    p: u8,
    a: u16,
    x: u16,
    y: u16,
    dbr: u8,
    d: u16,
    pbr: u8,
    e: u8,
    pub ram: Vec<(u32, u8)>,
}

impl CpuState {
    pub fn from_state(&self) -> (W65C816<TomHarteBus>, TomHarteBus) {
        let cpu = Cpu {
            accumulator: self.a,
            index_x: self.x,
            index_y: self.y,
            stack_pointer: self.s,
            program_counter: self.pc,
            status: Status(self.p),
            dpr: self.d,
            pbr: self.pbr,
            dbr: self.dbr,
            emulation_mode: self.e == 1,
            stopped: false,
            waiting_interrupt: false,
        };

        let w65c816 = W65C816 {
            cpu,
            instruction_set: W65C816::opcode_table(),
        };
        let mut bus = TomHarteBus::default();
        for (addr, val) in &self.ram {
            bus.memory.insert(*addr, *val);
        }

        (w65c816, bus)
    }
}

impl From<(Cpu, TomHarteBus)> for CpuState {
    fn from(value: (Cpu, TomHarteBus)) -> Self {
        let mut ram: Vec<(u32, u8)> = value.1.memory.into_iter().collect();
        ram.sort();
        Self {
            pc: value.0.program_counter,
            s: value.0.stack_pointer,
            p: value.0.status.0,
            a: value.0.accumulator,
            x: value.0.index_x,
            y: value.0.index_y,
            dbr: value.0.dbr,
            d: value.0.dpr,
            pbr: value.0.pbr,
            e: value.0.emulation_mode as u8,
            ram,
        }
    }
}

impl std::fmt::Display for CpuState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pc:{:04X} s:{:04X} p:{:02X} a:{:04X} x:{:04X} y:{:04X} dbr:{:02X} d:{:04X} pbr:{:02X} e:{:01X} \n\t  ram:{:02X?}",
            self.pc,
            self.s,
            self.p,
            self.a,
            self.x,
            self.y,
            self.dbr,
            self.d,
            self.pbr,
            self.e,
            self.ram
        )
    }
}
