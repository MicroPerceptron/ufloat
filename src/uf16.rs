use core::cmp::Ordering;
use core::fmt;
use core::ops::{Add, Div, Mul, Sub};

use crate::{ConversionError, dispatch};

macro_rules! impl_float_format {
    ($ty:ty, $to_float:ident) => {
        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.$to_float(), f)
            }
        }

        impl fmt::LowerExp for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::LowerExp::fmt(&self.$to_float(), f)
            }
        }

        impl fmt::UpperExp for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::UpperExp::fmt(&self.$to_float(), f)
            }
        }
    };
}

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
#[repr(transparent)]
/// A 16-bit unsigned float with 5 exponent bits and 11 mantissa bits.
pub struct Uf16E5M11(u16);

/// Default 16-bit unsigned float layout.
pub type Uf16 = Uf16E5M11;

impl Uf16E5M11 {
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

impl From<f32> for Uf16E5M11 {
    fn from(value: f32) -> Self {
        Self::from_f32(value)
    }
}

#[cfg(feature = "f16")]
impl From<f16> for Uf16E5M11 {
    fn from(value: f16) -> Self {
        Self::from_f16(value)
    }
}

impl From<Uf16E5M11> for f32 {
    fn from(value: Uf16E5M11) -> Self {
        value.to_f32()
    }
}

#[cfg(feature = "f16")]
impl From<Uf16E5M11> for f16 {
    fn from(value: Uf16E5M11) -> Self {
        value.to_f16()
    }
}

impl From<Uf16E5M11> for f64 {
    fn from(value: Uf16E5M11) -> Self {
        value.to_f64()
    }
}

impl Ord for Uf16E5M11 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Uf16E5M11 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Uf16E5M11 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(dispatch::add_uf16(self.0, rhs.0))
    }
}

impl Sub for Uf16E5M11 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(dispatch::sub_uf16(self.0, rhs.0))
    }
}

impl Mul for Uf16E5M11 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(dispatch::mul_uf16(self.0, rhs.0))
    }
}

impl Div for Uf16E5M11 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(dispatch::div_uf16(self.0, rhs.0))
    }
}

impl fmt::Debug for Uf16E5M11 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Uf16E5M11").field(&self.to_f32()).finish()
    }
}

impl_float_format!(Uf16E5M11, to_f32);

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
#[repr(transparent)]
/// A 16-bit unsigned float with 6 exponent bits and 10 mantissa bits.
pub struct Uf16E6M10(u16);

impl Uf16E6M10 {
    pub const EXPONENT_BITS: u32 = 6;
    pub const MANTISSA_BITS: u32 = 10;
    pub const EXPONENT_BIAS: i32 = 31;
    pub const EXPONENT_MASK: u16 = 0xfc00;
    pub const MANTISSA_MASK: u16 = 0x03ff;

    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(0x7c00);
    pub const INFINITY: Self = Self(0xfc00);
    pub const NAN: Self = Self(0xfe00);
    pub const MAX: Self = Self(0xfbff);
    pub const MIN_POSITIVE: Self = Self(0x0001);
    pub const MIN_NORMAL: Self = Self(0x0400);

    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    pub const fn to_bits(self) -> u16 {
        self.0
    }

    pub fn from_f32(value: f32) -> Self {
        Self(dispatch::f32_to_uf16_e6m10(value))
    }

    pub fn to_f32(self) -> f32 {
        dispatch::uf16_e6m10_to_f32(self.0)
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
        self.exponent() == 0x3f && self.mantissa() != 0
    }

    pub const fn is_infinite(self) -> bool {
        self.0 == Self::INFINITY.0
    }

    pub const fn is_finite(self) -> bool {
        self.exponent() != 0x3f
    }

    pub const fn is_subnormal(self) -> bool {
        self.exponent() == 0 && self.mantissa() != 0
    }
}

impl From<f32> for Uf16E6M10 {
    fn from(value: f32) -> Self {
        Self::from_f32(value)
    }
}

#[cfg(feature = "f16")]
impl From<f16> for Uf16E6M10 {
    fn from(value: f16) -> Self {
        Self::from_f16(value)
    }
}

impl From<Uf16E6M10> for f32 {
    fn from(value: Uf16E6M10) -> Self {
        value.to_f32()
    }
}

#[cfg(feature = "f16")]
impl From<Uf16E6M10> for f16 {
    fn from(value: Uf16E6M10) -> Self {
        value.to_f16()
    }
}

impl From<Uf16E6M10> for f64 {
    fn from(value: Uf16E6M10) -> Self {
        value.to_f64()
    }
}

impl Ord for Uf16E6M10 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Uf16E6M10 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Uf16E6M10 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(dispatch::add_uf16_e6m10(self.0, rhs.0))
    }
}

impl Sub for Uf16E6M10 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(dispatch::sub_uf16_e6m10(self.0, rhs.0))
    }
}

impl Mul for Uf16E6M10 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(dispatch::mul_uf16_e6m10(self.0, rhs.0))
    }
}

impl Div for Uf16E6M10 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(dispatch::div_uf16_e6m10(self.0, rhs.0))
    }
}

impl fmt::Debug for Uf16E6M10 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Uf16E6M10").field(&self.to_f32()).finish()
    }
}

impl_float_format!(Uf16E6M10, to_f32);
