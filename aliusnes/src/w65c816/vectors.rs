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
            NativeVectors::COP => 0xFFE4,
            NativeVectors::BRK => 0xFFE6,
            NativeVectors::ABORT => 0xFFE8,
            NativeVectors::NMI => 0xFFEA,
            NativeVectors::IRQ => 0xFFEE,
        }
    }
}

impl EmulationVectors {
    pub fn get_interrupt_addr(&self) -> u32 {
        match self {
            EmulationVectors::COP => 0xFFF4,
            EmulationVectors::ABORT => 0xFFF8,
            EmulationVectors::NMI => 0xFFFA,
            EmulationVectors::RESET => 0xFFFC,
            EmulationVectors::BRK => 0xFFFE,
        }
    }
}
