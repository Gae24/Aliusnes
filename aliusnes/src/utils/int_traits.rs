pub trait ManipulateU16 {
    fn low_byte(self) -> u8;
    fn high_byte(self) -> u8;
    fn set_low_byte(&mut self, low: u8);
    fn set_high_byte(&mut self, high: u8);
}

impl ManipulateU16 for u16 {
    #[inline]
    fn low_byte(self) -> u8 {
        self as u8
    }

    #[inline]
    fn high_byte(self) -> u8 {
        (self >> 8) as u8
    }

    #[inline]
    fn set_low_byte(&mut self, low: u8) {
        *self = (*self & 0xFF00) | u16::from(low);
    }

    #[inline]
    fn set_high_byte(&mut self, high: u8) {
        *self = (*self & 0x00FF) | (u16::from(high) << 8);
    }
}
