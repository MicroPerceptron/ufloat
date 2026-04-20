use core::cmp::Ordering;
use core::fmt;
use core::ops::{Add, Div, Mul, Sub};

use crate::{ConversionError, dispatch};

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
#[repr(transparent)]
/// An 8-bit unsigned float with 4 exponent bits and 4 mantissa bits.
pub struct Uf8E4M4(u8);

/// Default 8-bit unsigned float layout.
pub type Uf8 = Uf8E4M4;

impl Uf8E4M4 {
    pub const EXPONENT_BITS: u32 = 4;
    pub const MANTISSA_BITS: u32 = 4;
    pub const EXPONENT_BIAS: i32 = 7;
    pub const EXPONENT_MASK: u8 = 0xf0;
    pub const MANTISSA_MASK: u8 = 0x0f;

    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(0x70);
    pub const INFINITY: Self = Self(0xf0);
    pub const NAN: Self = Self(0xf8);
    pub const MAX: Self = Self(0xef);
    pub const MIN_POSITIVE: Self = Self(0x01);
    pub const MIN_NORMAL: Self = Self(0x10);

    pub const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    pub const fn to_bits(self) -> u8 {
        self.0
    }

    pub fn from_f32(value: f32) -> Self {
        Self(dispatch::f32_to_uf8(value))
    }

    pub fn to_f32(self) -> f32 {
        dispatch::uf8_to_f32(self.0)
    }

    pub fn from_f64(value: f64) -> Self {
        Self(dispatch::f32_to_uf8(value as f32))
    }

    pub fn to_f64(self) -> f64 {
        self.to_f32() as f64
    }

    pub fn try_from_f64(value: f64) -> Result<Self, ConversionError> {
        crate::convert::check_finite_non_negative(value)?;

        let encoded = Self::from_f64(value);
        crate::convert::check_encoded(value, encoded.is_zero(), encoded.is_infinite())?;

        Ok(encoded)
    }

    #[cfg(feature = "f16")]
    pub fn from_f16(value: f16) -> Self {
        Self::from_f32(value as f32)
    }

    #[cfg(feature = "f16")]
    pub fn to_f16(self) -> f16 {
        self.to_f32() as f16
    }

    pub const fn exponent(self) -> u8 {
        (self.0 & Self::EXPONENT_MASK) >> Self::MANTISSA_BITS
    }

    pub const fn mantissa(self) -> u8 {
        self.0 & Self::MANTISSA_MASK
    }

    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub const fn is_nan(self) -> bool {
        self.exponent() == 0x0f && self.mantissa() != 0
    }

    pub const fn is_infinite(self) -> bool {
        self.0 == Self::INFINITY.0
    }

    pub const fn is_finite(self) -> bool {
        self.exponent() != 0x0f
    }

    pub const fn is_subnormal(self) -> bool {
        self.exponent() == 0 && self.mantissa() != 0
    }
}

impl From<f32> for Uf8E4M4 {
    fn from(value: f32) -> Self {
        Self::from_f32(value)
    }
}

#[cfg(feature = "f16")]
impl From<f16> for Uf8E4M4 {
    fn from(value: f16) -> Self {
        Self::from_f16(value)
    }
}

impl From<Uf8E4M4> for f32 {
    fn from(value: Uf8E4M4) -> Self {
        value.to_f32()
    }
}

#[cfg(feature = "f16")]
impl From<Uf8E4M4> for f16 {
    fn from(value: Uf8E4M4) -> Self {
        value.to_f16()
    }
}

impl From<Uf8E4M4> for f64 {
    fn from(value: Uf8E4M4) -> Self {
        value.to_f64()
    }
}

impl Ord for Uf8E4M4 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Uf8E4M4 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Uf8E4M4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(dispatch::add_uf8(self.0, rhs.0))
    }
}

impl Sub for Uf8E4M4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(dispatch::sub_uf8(self.0, rhs.0))
    }
}

impl Mul for Uf8E4M4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(dispatch::mul_uf8(self.0, rhs.0))
    }
}

impl Div for Uf8E4M4 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(dispatch::div_uf8(self.0, rhs.0))
    }
}

impl fmt::Debug for Uf8E4M4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Uf8E4M4").field(&self.to_f32()).finish()
    }
}

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
#[repr(transparent)]
/// An 8-bit unsigned float with 5 exponent bits and 3 mantissa bits.
pub struct Uf8E5M3(u8);

impl Uf8E5M3 {
    pub const EXPONENT_BITS: u32 = 5;
    pub const MANTISSA_BITS: u32 = 3;
    pub const EXPONENT_BIAS: i32 = 15;
    pub const EXPONENT_MASK: u8 = 0xf8;
    pub const MANTISSA_MASK: u8 = 0x07;

    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(0x78);
    pub const INFINITY: Self = Self(0xf8);
    pub const NAN: Self = Self(0xfc);
    pub const MAX: Self = Self(0xf7);
    pub const MIN_POSITIVE: Self = Self(0x01);
    pub const MIN_NORMAL: Self = Self(0x08);

    pub const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    pub const fn to_bits(self) -> u8 {
        self.0
    }

    pub fn from_f32(value: f32) -> Self {
        Self(dispatch::f32_to_uf8_e5m3(value))
    }

    pub fn to_f32(self) -> f32 {
        dispatch::uf8_e5m3_to_f32(self.0)
    }

    pub fn from_f64(value: f64) -> Self {
        Self::from_f32(value as f32)
    }

    pub fn to_f64(self) -> f64 {
        self.to_f32() as f64
    }

    pub fn try_from_f64(value: f64) -> Result<Self, ConversionError> {
        crate::convert::check_finite_non_negative(value)?;

        let encoded = Self::from_f64(value);
        crate::convert::check_encoded(value, encoded.is_zero(), encoded.is_infinite())?;

        Ok(encoded)
    }

    #[cfg(feature = "f16")]
    pub fn from_f16(value: f16) -> Self {
        Self::from_f32(value as f32)
    }

    #[cfg(feature = "f16")]
    pub fn to_f16(self) -> f16 {
        self.to_f32() as f16
    }

    pub const fn exponent(self) -> u8 {
        (self.0 & Self::EXPONENT_MASK) >> Self::MANTISSA_BITS
    }

    pub const fn mantissa(self) -> u8 {
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

impl From<f32> for Uf8E5M3 {
    fn from(value: f32) -> Self {
        Self::from_f32(value)
    }
}

#[cfg(feature = "f16")]
impl From<f16> for Uf8E5M3 {
    fn from(value: f16) -> Self {
        Self::from_f16(value)
    }
}

impl From<Uf8E5M3> for f32 {
    fn from(value: Uf8E5M3) -> Self {
        value.to_f32()
    }
}

#[cfg(feature = "f16")]
impl From<Uf8E5M3> for f16 {
    fn from(value: Uf8E5M3) -> Self {
        value.to_f16()
    }
}

impl From<Uf8E5M3> for f64 {
    fn from(value: Uf8E5M3) -> Self {
        value.to_f64()
    }
}

impl Ord for Uf8E5M3 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Uf8E5M3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Uf8E5M3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(dispatch::add_uf8_e5m3(self.0, rhs.0))
    }
}

impl Sub for Uf8E5M3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(dispatch::sub_uf8_e5m3(self.0, rhs.0))
    }
}

impl Mul for Uf8E5M3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(dispatch::mul_uf8_e5m3(self.0, rhs.0))
    }
}

impl Div for Uf8E5M3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(dispatch::div_uf8_e5m3(self.0, rhs.0))
    }
}

impl fmt::Debug for Uf8E5M3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Uf8E5M3").field(&self.to_f32()).finish()
    }
}
