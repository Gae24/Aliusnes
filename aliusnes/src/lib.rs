#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use cart::{header::Header, Cart};

pub mod bus;
pub mod cart;
pub mod emu;
mod ppu;
mod utils;
pub mod w65c816;

#[macro_use]
extern crate proc_bitfield;

pub fn load_cart(rom: &[u8], ram: Vec<u8>) -> Cart {
    let header = Header::guess_from_rom(rom).expect("Cartridge not recognised");
    Cart::new(header, rom, ram)
}
