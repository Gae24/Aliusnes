use cpu::Cpu;
use opcode::{opcode_table, Meta, OpCode};

use crate::bus::Bus;

mod addressing;
mod cpu;
mod instructions;
mod opcode;

pub(crate) struct Spc700<B: Bus> {
    cpu: Cpu,
    instruction_set: [OpCode<B>; 256],
}

impl<B: Bus> Spc700<B> {
    pub(crate) fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            instruction_set: opcode_table(),
        }
    }

    pub(crate) fn step(&mut self, bus: &mut B) {
        let op = self.cpu.get_imm::<B>(bus);
        let opcode = &self.instruction_set[op as usize];

        let instr = opcode.function;
        instr(&mut self.cpu, bus, opcode.meta.mode);
    }

    pub(crate) fn peek_opcode(&self, bus: &B) -> Meta {
        let op = bus
            .peek_at(self.cpu.program_counter.into())
            .unwrap_or_default();
        self.instruction_set[op as usize].meta
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apu::spc700::cpu::Status;
    use crate::utils::testbus::{deserialize_as_map, Cycle, TomHarteBus};
    use crate::utils::testrun::{run_test, OpcodeTest};
    use serde::{Deserialize, Deserializer};
    use std::{collections::HashMap, path::PathBuf};

    #[derive(Debug, PartialEq, Deserialize)]
    pub struct CpuState {
        pc: u16,
        a: u8,
        x: u8,
        y: u8,
        sp: u8,
        psw: u8,
        #[serde(deserialize_with = "deserialize_as_map")]
        ram: HashMap<u32, u8>,
    }

    impl CpuState {
        pub fn convert_state(&self) -> (Spc700<TomHarteBus>, TomHarteBus) {
            let mut spc700 = Spc700::new();
            spc700.cpu.accumulator = self.a;
            spc700.cpu.index_x = self.x;
            spc700.cpu.index_y = self.y;
            spc700.cpu.program_counter = self.pc;
            spc700.cpu.stack_pointer = self.sp;
            spc700.cpu.status = Status(self.psw);

            let bus = TomHarteBus {
                memory: self.ram.clone(),
                ..Default::default()
            };

            (spc700, bus)
        }
    }

    impl From<(Cpu, TomHarteBus)> for CpuState {
        fn from(value: (Cpu, TomHarteBus)) -> Self {
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

    impl std::fmt::Display for CpuState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "pc:{:04X} sp:{:04X} psw:{:02X} a:{:04X} x:{:04X} y:{:04X} \n\t  ram:{:02X?}",
                self.pc, self.sp, self.psw, self.a, self.x, self.y, self.ram
            )
        }
    }

    impl OpcodeTest for CpuState {
        type Proc = Cpu;

        fn test_path(name: &str) -> std::path::PathBuf {
            let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            root_dir.join(format!("tests/spc700/{name}.json.xz"))
        }

        fn do_step(
            &mut self,
            _other: &Self,
            _cycles_len: usize,
        ) -> (Self::Proc, TomHarteBus, bool) {
            let (mut spc700, mut bus) = self.convert_state();
            let opcode = spc700.peek_opcode(&bus);
            let skip_cycles = opcode.code == 0xFE;

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

    include!(concat!(env!("OUT_DIR"), "/tomharte_spc700.rs"));
}
