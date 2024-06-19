mod utils;

use serde::Deserialize;
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

fn deserialize_cycles<'de, D>(deserializer: D) -> Result<Vec<Cycle>, D::Error>
where
    D: serde::Deserializer<'de>,
{
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
    let mut total = 0;
    let mut success = 0;
    let json_path = PathBuf::from(format!("../../65816/v1/{name}.json"));

    for mut test_case in TestCase::iter_json(&json_path) {
        total += 1;
        let (mut w65c816, mut bus) = test_case.initial.from_state();
        let opcode = w65c816.peek_opcode(&mut bus);

        w65c816.test_step(&mut bus);

        test_case.final_state.ram.sort();
        let result_cycles = bus.cycles.clone();
        let result_state = CpuState::from((w65c816.cpu, bus));
        let states_match = result_state == test_case.final_state;
        let cycles_match = result_cycles == test_case.cycles;

        if states_match && cycles_match {
            success += 1;
            continue;
        }

        println!(
            "\nTest {} Failed: {:#04X} {} {:?}",
            test_case.name, opcode.code, opcode.mnemonic, opcode.mode
        );
        if !states_match {
            println!("Initial:  {}", &test_case.initial);
            println!("Got:      {}", &result_state);
            println!("Expected: {}", &test_case.final_state);
        }
        if !cycles_match {
            println!("Got:");
            println!("{:?}", &result_cycles);
            println!("Expected:");
            println!("{:?}", &test_case.cycles);
        }
    }
    println!("{name} Passed({success}/{total})");
    assert_eq!(success, total);
}
