mod utils;

use core::panic;
use pretty_assertions::Comparison;
use serde::{Deserialize, Deserializer};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
use utils::{cpu_state::CpuState, test_bus::Cycle};

include!(concat!(env!("OUT_DIR"), "/tomharte_65816.rs"));

#[derive(Deserialize)]
struct TestCase {
    name: String,
    initial: CpuState,
    #[serde(rename = "final")]
    final_state: CpuState,
    #[serde(deserialize_with = "deserialize_cycles")]
    cycles: Vec<Cycle>,
}

fn deserialize_cycles<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<Cycle>, D::Error> {
    let v: Vec<(Option<u32>, Option<u8>, String)> = Deserialize::deserialize(deserializer)?;
    let cycles = v
        .iter()
        .map(|(addr, value, state)| {
            if state.contains('r') {
                Cycle::Read(addr.unwrap_or_default(), *value)
            } else if state.contains('w') {
                Cycle::Write(addr.unwrap_or_default(), value.unwrap_or_default())
            } else {
                Cycle::Internal
            }
        })
        .collect();
    Ok(cycles)
}

impl TestCase {
    fn iter_json(path: &PathBuf) -> impl Iterator<Item = Self> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        reader.lines().map(|line| {
            let line = line.unwrap();
            let trimmed = line
                .trim_end_matches(']')
                .trim_end_matches(',')
                .trim_start_matches('[');
            serde_json::from_str::<Self>(trimmed).unwrap()
        })
    }
}

pub fn run_test(name: &str) {
    let mut success = 0;
    let json_path = PathBuf::from(format!("../../65816/v1/{name}.json"));

    for test_case in TestCase::iter_json(&json_path) {
        let (mut w65c816, mut bus) = test_case.initial.from_state();

        let (final_cpu, mut final_bus) = test_case.final_state.from_state();
        final_bus.cycles = test_case.cycles;

        let opcode = w65c816.peek_opcode(&mut bus);
        w65c816.test_step(&mut bus);

        bus.cycles.sort();
        final_bus.cycles.sort();

        let cpu_match = w65c816.cpu == final_cpu.cpu;
        let memory_match = bus.memory == final_bus.memory;
        let cycles_match = bus.cycles == final_bus.cycles;

        if cpu_match && memory_match && cycles_match {
            success += 1;
            continue;
        }

        if !cpu_match {
            println!("Initial:  {}", &test_case.initial);
            println!("Result: {}", Comparison::new(&w65c816.cpu, &final_cpu.cpu));
        }
        if !memory_match {
            println!(
                "Memory: {}",
                Comparison::new(&bus.memory, &final_bus.memory)
            );
        }
        if !cycles_match {
            println!(
                "Cycles: {}",
                Comparison::new(&bus.cycles, &final_bus.cycles)
            );
        }
        panic!(
            "\nTest {} failed: {:#04X} {} {:?}",
            test_case.name, opcode.code, opcode.mnemonic, opcode.mode
        );
    }
    println!("{name} Passed({success}/10000)");
    assert_eq!(success, 10000);
}
