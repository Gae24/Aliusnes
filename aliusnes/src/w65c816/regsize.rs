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

    fn from_u16(value: u16) -> Self;
    fn from_u8(value: u8) -> Self;
    fn as_u16(self) -> u16;
    fn as_u8(self) -> u8;
    fn is_zero(self) -> bool;
    fn is_negative(self) -> bool;
    fn is_overflow(self) -> bool;
    fn wrapping_add(self, other: Self) -> Self;
    fn wrapping_sub(self, other: Self) -> Self;
}

impl RegSize for u8 {
    const IS_U16: bool = false;

    fn from_u16(value: u16) -> Self {
        value as u8
    }

    fn from_u8(value: u8) -> Self {
        value
    }

    fn as_u16(self) -> u16 {
        u16::from(self)
    }

    fn as_u8(self) -> u8 {
        self
    }

    fn is_zero(self) -> bool {
        self == 0
    }

    fn is_negative(self) -> bool {
        self >> 7 != 0
    }

    fn is_overflow(self) -> bool {
        (self >> 6) & 1 != 0
    }

    fn wrapping_add(self, other: Self) -> Self {
        u8::wrapping_add(self, other)
    }

    fn wrapping_sub(self, other: Self) -> Self {
        u8::wrapping_sub(self, other)
    }
}

impl RegSize for u16 {
    const IS_U16: bool = true;

    fn from_u16(value: u16) -> Self {
        value
    }

    fn from_u8(value: u8) -> Self {
        u16::from(value)
    }

    fn as_u16(self) -> u16 {
        self
    }

    fn as_u8(self) -> u8 {
        self as u8
    }

    fn is_zero(self) -> bool {
        self == 0
    }

    fn is_negative(self) -> bool {
        self >> 15 != 0
    }

    fn is_overflow(self) -> bool {
        (self >> 14) & 1 != 0
    }

    fn wrapping_add(self, other: Self) -> Self {
        u16::wrapping_add(self, other)
    }

    fn wrapping_sub(self, other: Self) -> Self {
        u16::wrapping_sub(self, other)
    }
}
