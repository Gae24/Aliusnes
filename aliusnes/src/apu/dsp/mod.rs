pub(crate) struct Dsp {
    dsp_addr: u8,
}

impl Dsp {
    pub(crate) fn new() -> Dsp {
        Dsp { dsp_addr: 0 }
    }

    pub(crate) fn set_dsp_addr(&mut self, value: u8) {
        self.dsp_addr = value;
    }

    pub(crate) fn read_dsp_addr(&self) -> u8 {
        self.dsp_addr
    }

    pub(crate) fn read(&self) -> u8 {
        // TODO read from registers
        self.dsp_addr
    }

    pub(crate) fn write(&mut self, _: u8) {
        // TODO write to registers
    }
}
