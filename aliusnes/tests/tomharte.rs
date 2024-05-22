use aliusnes::bus::Bus;
use aliusnes::w65c816::cpu::{Cpu, Status};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(PartialEq, Eq, Serialize, Deserialize)]
struct CpuState {
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
    ram: Vec<(u32, u8)>,
}

impl CpuState {
    fn from_state(&self) -> (Cpu, Bus) {
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
            cycles: 0,
        };

        let mut bus = Bus::default();
        for (addr, val) in &self.ram {
            bus.memory.insert(*addr, *val);
        }

        (cpu, bus)
    }
}

impl From<(Cpu, Bus)> for CpuState {
    fn from(value: (Cpu, Bus)) -> Self {
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

#[derive(Serialize, Deserialize)]
struct TestCase {
    name: String,
    initial: CpuState,
    #[serde(rename = "final")]
    final_state: CpuState,
    cycles: Vec<(u32, Option<u8>, String)>,
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
        let (mut cpu, mut bus) = test_case.initial.from_state();
        let opcode = cpu.peek_opcode(&bus);

        cpu.step(&mut bus);

        test_case.final_state.ram.sort();
        let result_state = CpuState::from((cpu, bus));
        let states_match = result_state == test_case.final_state;
        if states_match {
            success += 1;
            continue;
        }

        println!(
            "\nTest {} Failed: {:#04X} {} {:?}",
            test_case.name, opcode.code, opcode.mnemonic, opcode.mode
        );
        println!("Initial:  {}", &test_case.initial);
        println!("Got:      {}", &result_state);
        println!("Expected: {}", &test_case.final_state);
    }
    println!("{name} Passed({success}/{total})");
    assert_eq!(success, total);
}
