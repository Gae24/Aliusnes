pub enum Vectors {
    COP,
    BRK,
    ABORT,
    NMI,
    IRQ,
    EMU_COP,
    EMU_ABORT,
    EMU_NMI,
    EMU_RESET,
    EMU_BRK,
}

impl Vectors {
    pub fn get_interrupt_addr(&self) -> u32 {
        match self {
            Vectors::COP => 0xFFE4,
            Vectors::BRK => 0xFFE6,
            Vectors::ABORT => 0xFFE8,
            Vectors::NMI => 0xFFEA,
            Vectors::IRQ => 0xFFEE,
            Vectors::EMU_COP => 0xFFF4,
            Vectors::EMU_ABORT => 0xFFF8,
            Vectors::EMU_NMI => 0xFFFA,
            Vectors::EMU_RESET => 0xFFFC,
            Vectors::EMU_BRK => 0xFFFE,
        }
    }
}
