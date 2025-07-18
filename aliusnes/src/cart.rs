pub mod header;
pub mod info;

use header::Header;
use info::{Mapper, Model};

pub struct Cart {
    header: Header,
    pub model: Model,
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_mask: usize,
    ram_mask: usize,
}

impl Cart {
    pub fn new(header: Header, rom: &[u8], ram: Vec<u8>) -> Self {
        Cart {
            rom_mask: rom_mask(rom.len()),
            ram_mask: (header.ram_size - 1) as usize,
            model: header.country.to_model(),
            header,
            rom: rom.to_vec(),
            ram,
        }
    }

    pub fn read(&self, bank: u8, addr: u16) -> Option<u8> {
        match self.header.mapper {
            Mapper::LoROM => self.read_lo_rom(bank.into(), addr.into()),
            Mapper::HiROM => todo!(),
            Mapper::SA1ROM => todo!(),
            Mapper::SDD1ROM => todo!(),
            Mapper::ExHiROM => todo!(),
        }
    }

    pub fn write(&mut self, bank: u8, addr: u16, val: u8) {
        match self.header.mapper {
            Mapper::LoROM => self.write_lo_rom(bank, addr, val),
            Mapper::HiROM => todo!(),
            Mapper::SA1ROM => todo!(),
            Mapper::SDD1ROM => todo!(),
            Mapper::ExHiROM => todo!(),
        }
    }

    pub fn read_lo_rom(&self, mut bank: usize, addr: usize) -> Option<u8> {
        if ((0x70..0x7E).contains(&bank) || bank >= 0xF0)
            && (self.rom_mask < 0x200000 || addr < 0x8000)
            && self.header.chipset.has_ram
        {
            return Some(self.ram[(((bank & 0xF) << 15) | addr) & self.ram_mask]);
        }
        bank &= 0x7F;
        if addr >= 0x8000 || bank >= 0x40 {
            return Some(self.rom[((bank << 15) | (addr & 0x7FFF)) & self.rom_mask]);
        }
        println!("Attempt to read at 0x{:02x}{:04x}", bank, (addr & 0x7FFF));
        None
    }

    pub fn write_lo_rom(&mut self, bank: u8, addr: u16, val: u8) {
        if ((0x70..0x7E).contains(&bank) || bank >= 0xF0)
            && addr < 0x8000
            && self.header.chipset.has_ram
        {
            self.ram[(u32::from(u16::from(bank & 0xF) << 15) | u32::from(addr)) as usize] = val;
        }
    }
}

fn rom_mask(len: usize) -> usize {
    let mut mask = 0x8000;
    loop {
        if len <= mask {
            break;
        }
        mask *= 2;
    }
    mask - 1
}
