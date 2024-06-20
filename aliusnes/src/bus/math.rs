use super::Access;
use crate::utils::int_traits::ManipulateU16;

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

    fn do_multiplication(&mut self) {
        self.result_or_remainder = self.factor_a as u16 * self.factor_b as u16;
    }

    fn do_division(&mut self) {
        if self.divisor == 0 {
            self.quotient = 0xFFFF;
            self.result_or_remainder = self.dividend;
        } else {
            self.quotient = self.dividend / self.divisor as u16;
            self.result_or_remainder = self.dividend % self.divisor as u16;
        }
    }
}

impl Access for Math {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            0x4214 => Some(self.quotient.low_byte()),
            0x4215 => Some(self.quotient.high_byte()),
            0x4216 => Some(self.result_or_remainder.low_byte()),
            0x4217 => Some(self.result_or_remainder.high_byte()),
            _ => None,
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x4202 => self.factor_a = data,
            0x4203 => {
                self.factor_b = data;
                self.do_multiplication();
            }
            0x4204 => self.dividend.set_low_byte(data),
            0x4205 => self.dividend.set_high_byte(data),
            0x4206 => {
                self.divisor = data;
                self.do_division();
            }
            _ => unreachable!(),
        }
    }
}
