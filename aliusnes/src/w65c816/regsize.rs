use std::ops::{BitAnd, BitOr, BitXor, Not};

pub trait RegSize:
    Copy
    + Not<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Ord
{
    const IS_U16: bool;

    fn trunc_u16(value: u16) -> Self;
    fn ext_u8(value: u8) -> Self;
    fn is_zero(self) -> bool;
    fn is_negative(self) -> bool;
}

impl RegSize for u8 {
    const IS_U16: bool = false;

    fn trunc_u16(value: u16) -> Self {
        value as u8
    }

    fn ext_u8(value: u8) -> Self {
        value
    }

    fn is_zero(self) -> bool {
        self == 0
    }

    fn is_negative(self) -> bool {
        self >> 7 != 0
    }
}

impl RegSize for u16 {
    const IS_U16: bool = true;

    fn trunc_u16(value: u16) -> Self {
        value
    }

    fn ext_u8(value: u8) -> Self {
        value as u16
    }

    fn is_zero(self) -> bool {
        self == 0
    }

    fn is_negative(self) -> bool {
        self >> 15 != 0
    }
}
