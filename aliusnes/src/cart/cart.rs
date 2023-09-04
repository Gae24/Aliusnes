use super::header::Header;

pub struct Cart {
    header: Header,
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl Cart {
    pub fn new(header: Header, rom: &[u8], ram: Vec<u8>) -> Self {
        Cart {
            header,
            rom: rom.to_vec(),
            ram,
        }
    }

    pub fn read(&self, bank: u8, addr: u16) -> u8 {
        match self.header.mapper {
            super::info::Mapper::LoROM => self.read_lo_rom(bank, addr),
            super::info::Mapper::HiROM => todo!(),
            super::info::Mapper::SA1ROM => todo!(),
            super::info::Mapper::SDD1ROM => todo!(),
            super::info::Mapper::ExHiROM => todo!(),
        }
    }

    pub fn write(&mut self, bank: u8, addr: u16, val: u8) {
        match self.header.mapper {
            super::info::Mapper::LoROM => self.write_lo_rom(bank, addr, val),
            super::info::Mapper::HiROM => todo!(),
            super::info::Mapper::SA1ROM => todo!(),
            super::info::Mapper::SDD1ROM => todo!(),
            super::info::Mapper::ExHiROM => todo!(),
        }
    }

    pub fn read_lo_rom(&self, mut bank: u8, addr: u16) -> u8 {
        if ((bank >= 0x70 && bank < 0x7E) || bank >= 0xF0)
            && addr < 0x8000
            && self.header.chipset.has_ram
        {
            return self.ram[((((bank & 0xF) << 15) as u32 | addr as u32)
                & (self.header.ram_size - 1)) as usize];
        }
        bank &= 0x7F;
        if addr >= 0x8000 || bank >= 0x40 {
            return self.rom[(((bank << 15) as u32 | (addr & 0x7FFF) as u32)
                & (self.header.ram_size - 1)) as usize];
        }
        return todo!();
    }

    pub fn write_lo_rom(&mut self, bank: u8, addr: u16, val: u8) {
        if ((bank >= 0x70 && bank < 0x7E) || bank >= 0xF0)
            && addr < 0x8000
            && self.header.chipset.has_ram
        {
            self.ram[((((bank & 0xF) << 15) as u32 | addr as u32) & (self.header.ram_size - 1))
                as usize] = val;
        }
    }
}
