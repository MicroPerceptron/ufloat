use core::cmp::Ordering;
use core::fmt;
use core::ops::{Add, Div, Mul, Sub};

use crate::dispatch;

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct Uf16(u16);

impl Uf16 {
    pub const EXPONENT_BITS: u32 = 5;
    pub const MANTISSA_BITS: u32 = 11;
    pub const EXPONENT_BIAS: i32 = 15;
    pub const EXPONENT_MASK: u16 = 0xf800;
    pub const MANTISSA_MASK: u16 = 0x07ff;

    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(0x7800);
    pub const INFINITY: Self = Self(0xf800);
    pub const NAN: Self = Self(0xfc00);
    pub const MAX: Self = Self(0xf7ff);
    pub const MIN_POSITIVE: Self = Self(0x0001);
    pub const MIN_NORMAL: Self = Self(0x0800);

    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    pub const fn to_bits(self) -> u16 {
        self.0
    }

    pub fn from_f32(value: f32) -> Self {
        Self(dispatch::f32_to_uf16(value))
    }

    pub fn to_f32(self) -> f32 {
        dispatch::uf16_to_f32(self.0)
    }

    pub const fn exponent(self) -> u16 {
        (self.0 & Self::EXPONENT_MASK) >> Self::MANTISSA_BITS
    }

    pub const fn mantissa(self) -> u16 {
        self.0 & Self::MANTISSA_MASK
    }

    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub const fn is_nan(self) -> bool {
        self.exponent() == 0x1f && self.mantissa() != 0
    }

    pub const fn is_infinite(self) -> bool {
        self.0 == Self::INFINITY.0
    }

    pub const fn is_finite(self) -> bool {
        self.exponent() != 0x1f
    }

    pub const fn is_subnormal(self) -> bool {
        self.exponent() == 0 && self.mantissa() != 0
    }
}

impl From<f32> for Uf16 {
    fn from(value: f32) -> Self {
        Self::from_f32(value)
    }
}

impl From<Uf16> for f32 {
    fn from(value: Uf16) -> Self {
        value.to_f32()
    }
}

impl Ord for Uf16 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Uf16 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Uf16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(dispatch::add_uf16(self.0, rhs.0))
    }
}

impl Sub for Uf16 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(dispatch::sub_uf16(self.0, rhs.0))
    }
}

impl Mul for Uf16 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(dispatch::mul_uf16(self.0, rhs.0))
    }
}

impl Div for Uf16 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(dispatch::div_uf16(self.0, rhs.0))
    }
}

impl fmt::Debug for Uf16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Uf16").field(&self.to_f32()).finish()
    }
}
