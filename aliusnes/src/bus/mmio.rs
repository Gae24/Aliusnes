use super::bus::Bus;

static mut NMI: u8 = 0xC2;

impl Bus {
    pub fn read_mmio(&self, addr: u16) -> u8 {
        match addr {
            0x4210 => unsafe {
                if NMI == 0x42 {
                    NMI = 0xc2;
                    return NMI;
                } else {
                    NMI = 0x42;
                    return NMI;
                }
            },
            _ => panic!("tried to read at {:#0x}", addr),
        }
    }

    pub fn write_mmio(&mut self, addr: u16, val: u8) {
        println!("attempt write at {:#0x} {:#0x}", addr, val);
    }
}
