pub enum NativeVectors {
    COP,
    BRK,
    ABORT,
    NMI,
    IRQ,
}

pub enum EmulationVectors {
    COP,
    ABORT,
    NMI,
    RESET,
    BRK,
}

impl NativeVectors {
    pub fn get_interrupt_addr(&self) -> u32 {
        match self {
            NativeVectors::COP => 0x00FFE4,
            NativeVectors::BRK => 0x00FFE6,
            NativeVectors::ABORT => 0x00FFE8,
            NativeVectors::NMI => 0x00FFEA,
            NativeVectors::IRQ => 0x00FFEE,
        }
    }
}

impl EmulationVectors {
    pub fn get_interrupt_addr(&self) -> u32 {
        match self {
            EmulationVectors::COP => 0x00FFF4,
            EmulationVectors::ABORT => 0x00FFF8,
            EmulationVectors::NMI => 0x00FFFA,
            EmulationVectors::RESET => 0x00FFFC,
            EmulationVectors::BRK => 0x00FFFE,
        }
    }
}
