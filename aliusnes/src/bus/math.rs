// In hardware multiplication takes 8 cycles,
// while division 16
pub struct Math {
    pub factor_a: u8,
    pub factor_b: u8,
    pub dividend: u16,
    pub divisor: u8,
    pub quotient: u16,
    pub result_or_remainder: u16,
}

impl Math {
    pub fn new() -> Self {
        Math {
            factor_a: 0xFF,
            factor_b: 0,
            dividend: 0xFFFF,
            divisor: 0,
            quotient: 0,
            result_or_remainder: 0,
        }
    }

    pub fn do_multiplication(&mut self) {
        self.result_or_remainder = self.factor_a as u16 * self.factor_b as u16;
    }

    pub fn do_division(&mut self) {
        if self.divisor == 0 {
            self.quotient = 0xFFFF;
            self.result_or_remainder = self.dividend;
        } else {
            self.quotient = self.dividend / self.divisor as u16;
            self.result_or_remainder = self.dividend % self.divisor as u16;
        }
    }
}
