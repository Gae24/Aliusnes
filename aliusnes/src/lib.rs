use cart::{cart::Cart, header::Header};
use emu::Emu;

pub mod bus;
pub mod cart;
mod emu;
pub mod w65c816;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

pub fn run_emu(rom: &[u8], ram: Vec<u8>) {
    let playing = true;
    let header = Header::guess_from_rom(&rom).expect("Cartridge not recognised");
    let cart = Cart::new(header, &rom, ram);

    let mut emu = Emu::new(cart);
    while playing {
        emu.step();
    }
}
