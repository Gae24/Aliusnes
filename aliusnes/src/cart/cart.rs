use super::header::Header;

pub struct Cart {
    header: Header,
    rom: Vec<u8>,
    ram: Vec<u8>,
}
