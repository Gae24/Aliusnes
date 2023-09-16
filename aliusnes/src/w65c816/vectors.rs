pub enum Vectors {
    Cop,
    Brk,
    Abort,
    Nmi,
    Irq,
    EmuCop,
    EmuAbort,
    EmuNmi,
    EmuReset,
    EmuBrk,
}

impl Vectors {
    pub fn get_interrupt_addr(&self) -> u32 {
        match self {
            Vectors::Cop => 0xFFE4,
            Vectors::Brk => 0xFFE6,
            Vectors::Abort => 0xFFE8,
            Vectors::Nmi => 0xFFEA,
            Vectors::Irq => 0xFFEE,
            Vectors::EmuCop => 0xFFF4,
            Vectors::EmuAbort => 0xFFF8,
            Vectors::EmuNmi => 0xFFFA,
            Vectors::EmuReset => 0xFFFC,
            Vectors::EmuBrk => 0xFFFE,
        }
    }
}
