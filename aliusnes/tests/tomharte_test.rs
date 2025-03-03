mod utils;

use std::{
    fmt::{Debug, Display},
    fs::File,
    io::BufReader,
    path::PathBuf,
};

use pretty_assertions::Comparison;
use serde::{Deserialize, Deserializer};
use utils::{
    cpu_state::CpuState,
    test_bus::{Cycle, TomHarteBus},
};
use xz2::read::XzDecoder;

include!(concat!(env!("OUT_DIR"), "/tomharte_65816.rs"));

#[derive(Deserialize)]
struct TestCase<T> {
    name: String,
    #[serde(bound(deserialize = "T: OpcodeTest"))]
    initial: T,
    #[serde(rename = "final")]
    #[serde(bound(deserialize = "T: OpcodeTest"))]
    final_state: T,
    #[serde(deserialize_with = "T::deserialize_cycles")]
    #[serde(bound(deserialize = "T: OpcodeTest"))]
    cycles: Vec<Cycle>,
}

impl<T: OpcodeTest> TestCase<T> {
    fn iter_json(path: &PathBuf) -> impl Iterator<Item = Self> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(XzDecoder::new(file));

        serde_json::from_reader::<_, Vec<Self>>(reader)
            .unwrap()
            .into_iter()
    }
}

pub(crate) fn run_test<T: OpcodeTest>(name: &str) {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let json_path = root_dir.join(format!("tests/65816/{name}.json.xz"));

    for mut test_case in TestCase::<T>::iter_json(&json_path) {
        let (proc, bus, skip_cycles) = test_case
            .initial
            .do_step(&test_case.final_state, test_case.cycles.len());

        let mut cycles = bus.cycles.clone();
        cycles.sort();

        let state = T::from((proc, bus));

        let state_match = state == test_case.final_state;
        let cycles_match = cycles == test_case.cycles || skip_cycles;

        if state_match && cycles_match {
            continue;
        }

        println!("Test {} failed", test_case.name,);

        if !state_match {
            println!("Initial: {}", &test_case.initial);
            println!(
                "Result: {}",
                Comparison::new(&state, &test_case.final_state)
            );
        }
        if !cycles_match {
            println!("Cycles: {}", Comparison::new(&cycles, &test_case.cycles));
        }
        panic!();
    }
}

pub trait OpcodeTest:
    Debug + Display + PartialEq + for<'de> Deserialize<'de> + From<(Self::Proc, TomHarteBus)>
{
    type Proc;

    fn do_step(&mut self, other: &Self, cycles_len: usize) -> (Self::Proc, TomHarteBus, bool);
    fn deserialize_cycles<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<Cycle>, D::Error>;
}
