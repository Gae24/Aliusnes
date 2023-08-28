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

pub enum Mapper {
    LoROM,
    HiROM,
    SA1ROM,
    SDD1ROM,
    ExHiROM,
}

pub struct Chipset {
    pub has_coprocessor: bool,
    pub has_ram: bool,
    pub has_battery: bool,
}

pub enum Region {
    Japan,
    NorthAmerica,
    Europe,
    Sweden,
    Finland,
    Denmark,
    France,
    Netherlands,
    Spain,
    Germany,
    Italy,
    China,
    Indonesia,
    SouthKorea,
    International,
    Canada,
    Brazil,
    Australia,
}
