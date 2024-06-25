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
use xz2::read::XzDecoder;

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

impl TestCase {
    fn iter_json(path: &PathBuf) -> impl Iterator<Item = Self> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(XzDecoder::new(file));

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
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let json_path = root_dir.join(format!("tests/65816/{name}.json.xz"));

    for mut test_case in TestCase::iter_json(&json_path) {
        let (mut w65c816, mut bus) = test_case.initial.from_state();

        let opcode = w65c816.peek_opcode(&bus);
        w65c816.step(&mut bus);

        let mut cycles = bus.cycles.clone();
        cycles.sort();

        let cpu_state = CpuState::from((w65c816.cpu, bus));
        test_case.final_state.ram.sort();

        let state_match = cpu_state == test_case.final_state;
        let cycles_match = cycles == test_case.cycles;

        if state_match && cycles_match {
            continue;
        }

        println!(
            "Test {} failed: {:#04X} {} {:?}",
            test_case.name, opcode.code, opcode.mnemonic, opcode.mode
        );
        if !state_match {
            println!("Initial: {}", &test_case.initial);
            println!(
                "Result: {}",
                Comparison::new(&cpu_state, &test_case.final_state)
            );
        }
        if !cycles_match {
            println!("Cycles: {}", Comparison::new(&cycles, &test_case.cycles));
        }
        panic!();
    }
}
