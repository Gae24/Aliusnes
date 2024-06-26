#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mapper {
    LoROM,
    HiROM,
    SA1ROM,
    SDD1ROM,
    ExHiROM,
}

impl Mapper {
    pub fn get_base_mapper(self) -> Self {
        match self {
            Self::LoROM | Self::SA1ROM | Self::SDD1ROM => Self::LoROM,
            Self::HiROM => Self::HiROM,
            Self::ExHiROM => Self::ExHiROM,
        }
    }
}

pub struct Chipset {
    pub has_coprocessor: bool,
    pub has_ram: bool,
    pub has_battery: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Model {
    Ntsc,
    Pal,
}

#[derive(Debug)]
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
    Unknown(u8),
}

impl Region {
    pub(super) fn to_model(&self) -> Model {
        match self {
            Region::Japan
            | Region::SouthKorea
            | Region::NorthAmerica
            | Region::Canada
            | Region::Brazil
            | Region::Unknown(_) => Model::Ntsc,
            _ => Model::Pal,
        }
    }
}
