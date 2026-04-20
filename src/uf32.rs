use core::cmp::Ordering;
use core::fmt;
use core::ops::{Add, Div, Mul, Sub};

use crate::{ConversionError, dispatch};

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct Uf32E8M24(u32);

pub type Uf32 = Uf32E8M24;

impl Uf32E8M24 {
    pub const EXPONENT_BITS: u32 = 8;
    pub const MANTISSA_BITS: u32 = 24;
    pub const EXPONENT_BIAS: i32 = 127;
    pub const EXPONENT_MASK: u32 = 0xff00_0000;
    pub const MANTISSA_MASK: u32 = 0x00ff_ffff;

    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(0x7f00_0000);
    pub const INFINITY: Self = Self(0xff00_0000);
    pub const NAN: Self = Self(0xff80_0000);
    pub const MAX: Self = Self(0xfeff_ffff);
    pub const MIN_POSITIVE: Self = Self(0x0000_0001);
    pub const MIN_NORMAL: Self = Self(0x0100_0000);

    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    pub const fn to_bits(self) -> u32 {
        self.0
    }

    pub fn from_f64(value: f64) -> Self {
        Self(dispatch::f64_to_uf32(value))
    }

    pub fn to_f64(self) -> f64 {
        dispatch::uf32_to_f64(self.0)
    }

    pub fn try_from_f64(value: f64) -> Result<Self, ConversionError> {
        crate::convert::check_finite_non_negative(value)?;

        let encoded = Self::from_f64(value);
        crate::convert::check_encoded(value, encoded.is_zero(), encoded.is_infinite())?;

        Ok(encoded)
    }

    pub fn from_f32(value: f32) -> Self {
        Self::from_f64(value as f64)
    }

    pub fn to_f32(self) -> f32 {
        self.to_f64() as f32
    }

    #[cfg(feature = "f16")]
    pub fn from_f16(value: f16) -> Self {
        Self::from_f64(value as f64)
    }

    #[cfg(feature = "f16")]
    pub fn to_f16(self) -> f16 {
        self.to_f64() as f16
    }

    pub const fn exponent(self) -> u32 {
        (self.0 & Self::EXPONENT_MASK) >> Self::MANTISSA_BITS
    }

    pub const fn mantissa(self) -> u32 {
        self.0 & Self::MANTISSA_MASK
    }

    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub const fn is_nan(self) -> bool {
        self.exponent() == 0xff && self.mantissa() != 0
    }

    pub const fn is_infinite(self) -> bool {
        self.0 == Self::INFINITY.0
    }

    pub const fn is_finite(self) -> bool {
        self.exponent() != 0xff
    }

    pub const fn is_subnormal(self) -> bool {
        self.exponent() == 0 && self.mantissa() != 0
    }
}

impl From<f32> for Uf32E8M24 {
    fn from(value: f32) -> Self {
        Self::from_f32(value)
    }
}

#[cfg(feature = "f16")]
impl From<f16> for Uf32E8M24 {
    fn from(value: f16) -> Self {
        Self::from_f16(value)
    }
}

#[cfg(feature = "f16")]
impl From<Uf32E8M24> for f16 {
    fn from(value: Uf32E8M24) -> Self {
        value.to_f16()
    }
}

impl From<Uf32E8M24> for f32 {
    fn from(value: Uf32E8M24) -> Self {
        value.to_f32()
    }
}

impl From<Uf32E8M24> for f64 {
    fn from(value: Uf32E8M24) -> Self {
        value.to_f64()
    }
}

impl Ord for Uf32E8M24 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Uf32E8M24 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Uf32E8M24 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(dispatch::add_uf32(self.0, rhs.0))
    }
}

impl Sub for Uf32E8M24 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(dispatch::sub_uf32(self.0, rhs.0))
    }
}

impl Mul for Uf32E8M24 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(dispatch::mul_uf32(self.0, rhs.0))
    }
}

impl Div for Uf32E8M24 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(dispatch::div_uf32(self.0, rhs.0))
    }
}

impl fmt::Debug for Uf32E8M24 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Uf32").field(&self.to_f64()).finish()
    }
}
