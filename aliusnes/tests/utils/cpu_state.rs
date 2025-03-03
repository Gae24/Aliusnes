use std::{collections::HashMap, path::PathBuf};

use crate::{Cycle, OpcodeTest, TomHarteBus};
use aliusnes::{
    apu::spc700::{
        cpu::{Cpu as Spc, Status as Psw},
        Spc700,
    },
    w65c816::{
        cpu::{Cpu, Status},
        W65C816,
    },
};
use serde::{Deserialize, Deserializer};

pub(crate) fn deserialize_as_map<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<HashMap<u32, u8>, D::Error> {
    Vec::deserialize(deserializer).map(|vec| vec.into_iter().collect())
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct CpuState {
    pc: u16,
    s: u16,
    p: u8,
    a: u16,
    x: u16,
    y: u16,
    dbr: u8,
    d: u16,
    pbr: u8,
    e: u8,
    #[serde(deserialize_with = "deserialize_as_map")]
    ram: HashMap<u32, u8>,
}

impl CpuState {
    pub fn convert_state(&self) -> (W65C816<TomHarteBus>, TomHarteBus) {
        let mut w65c816 = W65C816::new();
        w65c816.cpu.accumulator = self.a;
        w65c816.cpu.index_x = self.x;
        w65c816.cpu.index_y = self.y;
        w65c816.cpu.stack_pointer = self.s;
        w65c816.cpu.program_counter = self.pc;
        w65c816.cpu.status = Status(self.p);
        w65c816.cpu.dpr = self.d;
        w65c816.cpu.pbr = self.pbr;
        w65c816.cpu.dbr = self.dbr;
        w65c816.cpu.emulation_mode = self.e == 1;
        w65c816.cpu.stopped = false;
        w65c816.cpu.waiting_interrupt = false;

        let bus = TomHarteBus {
            memory: self.ram.clone(),
            ..Default::default()
        };

        (w65c816, bus)
    }
}

impl From<(Cpu, TomHarteBus)> for CpuState {
    fn from(value: (Cpu, TomHarteBus)) -> Self {
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
            ram: value.1.memory,
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

impl OpcodeTest for CpuState {
    type Proc = Cpu;

    fn test_path(name: &str) -> PathBuf {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        root_dir.join(format!("tests/65816/{name}.json.xz"))
    }

    fn do_step(&mut self, other: &Self, cycles_len: usize) -> (Self::Proc, TomHarteBus, bool) {
        let (mut w65c816, mut bus) = self.convert_state();

        let opcode = w65c816.peek_opcode(&bus);
        let skip_cycles = if opcode.code == 0x44 || opcode.code == 0x54 {
            loop {
                if bus.cycles.len() >= (cycles_len - 2) {
                    break;
                }
                w65c816.step(&mut bus);
            }
            w65c816.cpu.program_counter = other.pc;
            true
        } else {
            w65c816.step(&mut bus);
            false
        };

        (w65c816.cpu, bus, skip_cycles)
    }

    fn deserialize_cycles<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<Cycle>, D::Error> {
        let v: Vec<(Option<u32>, Option<u8>, String)> = Deserialize::deserialize(deserializer)?;
        let mut cycles: Vec<Cycle> = v
            .iter()
            .map(|(addr, value, state)| {
                if !(state.contains('p') || state.contains('d')) {
                    Cycle::Internal
                } else if state.contains('r') {
                    Cycle::Read(addr.unwrap_or_default(), *value)
                } else if state.contains('w') {
                    Cycle::Write(addr.unwrap_or_default(), value.unwrap_or_default())
                } else {
                    Cycle::Internal
                }
            })
            .collect();
        cycles.sort();
        Ok(cycles)
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Spc700State {
    pc: u16,
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    psw: u8,
    #[serde(deserialize_with = "deserialize_as_map")]
    ram: HashMap<u32, u8>,
}

impl Spc700State {
    pub fn convert_state(&self) -> (Spc700<TomHarteBus>, TomHarteBus) {
        let mut spc700 = Spc700::new();
        spc700.cpu.accumulator = self.a;
        spc700.cpu.index_x = self.x;
        spc700.cpu.index_y = self.y;
        spc700.cpu.program_counter = self.pc;
        spc700.cpu.stack_pointer = self.sp;
        spc700.cpu.status = Psw(self.psw);

        let bus = TomHarteBus {
            memory: self.ram.clone(),
            ..Default::default()
        };

        (spc700, bus)
    }
}

impl From<(Spc, TomHarteBus)> for Spc700State {
    fn from(value: (Spc, TomHarteBus)) -> Self {
        Self {
            pc: value.0.program_counter,
            a: value.0.accumulator,
            x: value.0.index_x,
            y: value.0.index_y,
            sp: value.0.stack_pointer,
            psw: value.0.status.0,
            ram: value.1.memory,
        }
    }
}

impl std::fmt::Display for Spc700State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pc:{:04X} sp:{:04X} psw:{:02X} a:{:04X} x:{:04X} y:{:04X} \n\t  ram:{:02X?}",
            self.pc, self.sp, self.psw, self.a, self.x, self.y, self.ram
        )
    }
}

impl OpcodeTest for Spc700State {
    type Proc = Spc;

    fn test_path(name: &str) -> std::path::PathBuf {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        root_dir.join(format!("tests/spc700/{name}.json.xz"))
    }

    fn do_step(&mut self, _other: &Self, _cycles_len: usize) -> (Self::Proc, TomHarteBus, bool) {
        let (mut spc700, mut bus) = self.convert_state();
        let skip_cycles = false;

        spc700.step(&mut bus);

        (spc700.cpu, bus, skip_cycles)
    }

    fn deserialize_cycles<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<Cycle>, D::Error> {
        let v: Vec<(Option<u32>, Option<u8>, String)> = Deserialize::deserialize(deserializer)?;
        let mut cycles: Vec<Cycle> = v
            .iter()
            .map(|(addr, value, state)| {
                if state == "wait" {
                    Cycle::Internal
                } else if state == "read" {
                    Cycle::Read(addr.unwrap_or_default(), *value)
                } else if state == "write" {
                    Cycle::Write(addr.unwrap_or_default(), value.unwrap_or_default())
                } else {
                    panic!("Unknown state: {state}");
                }
            })
            .collect();
        cycles.sort();
        Ok(cycles)
    }
}
