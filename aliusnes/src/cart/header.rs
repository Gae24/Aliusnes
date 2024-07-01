use std::str::from_utf8;

use super::info::{Chipset, Mapper, Region};

pub struct Header {
    pub title: String,
    pub fast_rom: bool,
    pub mapper: Mapper,
    pub chipset: Chipset,
    pub rom_size: u32,
    pub ram_size: u32,
    pub country: Region,
    pub dev_id: u8,
    pub version: u8,
}

impl Header {
    pub fn new(bytes: &[u8], expected_mapper: Mapper) -> Option<Self> {
        let title = from_utf8(&bytes[0x10..0x25]).ok()?.trim_end().to_string();

        let raw_mapper = bytes[0x25];
        let fast_rom = raw_mapper & 0x10 != 0;

        let mapper = match raw_mapper & 0xF {
            0 => Mapper::LoROM,
            1 => Mapper::HiROM,
            2 => Mapper::SDD1ROM,
            3 => Mapper::SA1ROM,
            5 => Mapper::ExHiROM,
            _ => return None,
        };
        if mapper.get_base_mapper() != expected_mapper {
            return None;
        }

        let _raw_chipset = bytes[0x26];
        // todo chipset recognition
        let chipset = Chipset {
            has_coprocessor: false,
            has_ram: false,
            has_battery: false,
        };

        let rom_size = 0x400 << bytes[0x27];
        let ram_size = 0x400 << bytes[0x28];

        let country: Region = match bytes[0x29] {
            0x00 => Region::Japan,
            0x01 => Region::NorthAmerica,
            0x02 => Region::Europe,
            0x03 => Region::Sweden,
            0x04 => Region::Finland,
            0x05 => Region::Denmark,
            0x06 => Region::France,
            0x07 => Region::Netherlands,
            0x08 => Region::Spain,
            0x09 => Region::Germany,
            0x0A => Region::Italy,
            0x0B => Region::China,
            0x0C => Region::Indonesia,
            0x0D => Region::SouthKorea,
            0x0E => Region::International,
            0x0F => Region::Canada,
            0x10 => Region::Brazil,
            0x11 => Region::Australia,
            other => Region::Unknown(other),
        };

        let dev_id = bytes[0x2A];
        let version = bytes[0x2B];
        let checksum = u16::from_le_bytes([bytes[0x2C], bytes[0x2D]]);
        let complement = u16::from_le_bytes([bytes[0x2E], bytes[0x2F]]);

        println!("Title: {title}");
        println!("Mapper: {mapper:?}");
        println!("Is fast rom: {fast_rom}");
        println!("Rom size: {rom_size}");
        println!("Ram size: {ram_size}");
        println!("Country: {country:?}");
        println!("Checksum: {checksum}");
        println!("Complement: {complement}");

        Some(Header {
            title,
            fast_rom,
            mapper,
            chipset,
            rom_size,
            ram_size,
            country,
            dev_id,
            version,
        })
    }

    pub fn guess_from_rom(rom: &[u8]) -> Option<Self> {
        let header = rom[..]
            .get(0x7FB0..0x8000)
            .and_then(|header_bytes| Header::new(header_bytes, Mapper::LoROM))
            .or_else(|| {
                rom[..]
                    .get(0xFFB0..0x10000)
                    .and_then(|header_bytes| Header::new(header_bytes, Mapper::HiROM))
            })
            .or_else(|| Header::new(&rom[0x0040_FFB0..0x0041_0000], Mapper::ExHiROM))?;

        Some(header)
    }
}
