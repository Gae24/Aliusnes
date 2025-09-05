use crate::utils::testbus::Cycle;
use serde::{Deserialize, Deserializer};
use std::{fs::File, io::BufReader};
use xz2::read::XzDecoder;

use pretty_assertions::Comparison;

use std::{
    fmt::{Debug, Display},
    path::PathBuf,
};

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
    fn iter_json(path: PathBuf) -> impl Iterator<Item = Self> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(XzDecoder::new(file));

        serde_json::from_reader::<_, Vec<Self>>(reader)
            .unwrap()
            .into_iter()
    }
}

pub(crate) fn run_test<T: OpcodeTest>(name: &str) {
    for test_case in TestCase::<T>::iter_json(T::test_path(name)) {
        let (result, mut cycles, skip_cycles) = test_case
            .initial
            .step(&test_case.final_state, test_case.cycles.len());

        cycles.sort();

        let state_match = result == test_case.final_state;
        let cycles_match = cycles == test_case.cycles || skip_cycles;

        if state_match && cycles_match {
            continue;
        }

        println!("Test {} failed", test_case.name);
        if !state_match {
            println!("Initial: {}", &test_case.initial);
            println!(
                "Result: {}",
                Comparison::new(&result, &test_case.final_state)
            );
        }
        if !cycles_match {
            println!("Cycles: {}", Comparison::new(&cycles, &test_case.cycles));
        }
        panic!();
    }
}

pub(crate) trait OpcodeTest:
    Debug + Display + PartialEq + for<'de> Deserialize<'de>
{
    fn test_path(name: &str) -> PathBuf;
    fn step(&self, other: &Self, cycles_len: usize) -> (Self, Vec<Cycle>, bool);
    fn deserialize_cycles<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<Cycle>, D::Error>;
}
