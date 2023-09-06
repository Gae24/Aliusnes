use std::fs::File;

use cart::{cart::Cart, header::Header};
use emu::Emu;
use log::LevelFilter;
use simplelog::{Config, WriteLogger};

mod bus;
mod cart;
mod emu;
mod w65c816;
mod wram;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

pub fn run_emu(rom: &[u8], ram: Vec<u8>) {
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("aliusnesTest.log").unwrap(),
    )
    .unwrap();

    let header = Header::guess_from_rom(&rom).expect("Cartridge not recognised");
    let cart = Cart::new(header, &rom, ram);

    let adc_count = 22839;
    let bit_count = 8635;
    let mut i = 0;
    let mut emu = Emu::new(cart);
    while i <= bit_count {
        emu.step();
        i += 1;
    }
}
