pub struct Wram {
    pub ram: [u8; 0x20000],
    // todo impl wmaddr
}

impl Wram {
    pub fn new() -> Self {
        Self { ram: [0; 0x20000] }
    }
}
