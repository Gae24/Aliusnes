#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use cart::{header::Header, Cart};
use emu::Emu;

pub mod bus;
pub mod cart;
pub mod emu;
mod ppu;
mod utils;
pub mod w65c816;

#[macro_use]
extern crate proc_bitfield;

pub fn run_emu(rom: &[u8], ram: Vec<u8>) {
    let header = Header::guess_from_rom(rom).expect("Cartridge not recognised");
    let cart = Cart::new(header, rom, ram);

    let mut emu = Emu::new(cart);

    loop {
        emu.step();
    }
}

pub fn load_cart(rom: &[u8], ram: Vec<u8>) -> Cart {
    let header = Header::guess_from_rom(rom).expect("Cartridge not recognised");
    Cart::new(header, rom, ram)
}
