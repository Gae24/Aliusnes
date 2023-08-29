use super::header::Header;

pub struct Cart {
    header: Header,
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl Cart {
    pub fn new(header: Header, rom: Vec<u8>, ram: Vec<u8>) -> Self {
        Cart { header, rom, ram }
    }
}
