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
/// A 64-bit unsigned float with 11 exponent bits and 52 mantissa bits.
pub struct Uf64E11M52(u64);

/// Default 64-bit unsigned float layout. Requires the `f128` feature.
pub type Uf64 = Uf64E11M52;

impl Uf64E11M52 {
    pub const EXPONENT_BITS: u32 = 11;
    pub const MANTISSA_BITS: u32 = 52;
    pub const EXPONENT_BIAS: i32 = 1023;
    pub const EXPONENT_MASK: u64 = 0x7ff0_0000_0000_0000;
    pub const MANTISSA_MASK: u64 = 0x000f_ffff_ffff_ffff;

    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(0x3ff0_0000_0000_0000);
    pub const INFINITY: Self = Self(0x7ff0_0000_0000_0000);
    pub const NAN: Self = Self(0x7ff8_0000_0000_0000);
    pub const MAX: Self = Self(0x7fef_ffff_ffff_ffff);
    pub const MIN_POSITIVE: Self = Self(0x0000_0000_0000_0001);
    pub const MIN_NORMAL: Self = Self(0x0010_0000_0000_0000);

    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    pub const fn to_bits(self) -> u64 {
        self.0
    }

    pub fn from_f128(value: f128) -> Self {
        Self(dispatch::f128_to_uf64(value))
    }

    pub fn to_f128(self) -> f128 {
        dispatch::uf64_to_f128(self.0)
    }

    pub fn try_from_f128(value: f128) -> Result<Self, ConversionError> {
        if value.is_nan() {
            return Err(ConversionError::Nan);
        }
        if value.is_infinite() {
            return Err(ConversionError::Infinite);
        }
        if value < 0.0_f128 {
            return Err(ConversionError::Negative);
        }

        let encoded = Self::from_f128(value);
        if encoded.is_infinite() {
            Err(ConversionError::Overflow)
        } else if value != 0.0_f128 && encoded.is_zero() {
            Err(ConversionError::Underflow)
        } else {
            Ok(encoded)
        }
    }

    pub fn from_f64(value: f64) -> Self {
        Self::from_f128(value as f128)
    }

    pub fn to_f64(self) -> f64 {
        self.to_f128() as f64
    }

    pub fn try_from_f64(value: f64) -> Result<Self, ConversionError> {
        crate::convert::check_finite_non_negative(value)?;

        let encoded = Self::from_f64(value);
        crate::convert::check_encoded(value, encoded.is_zero(), encoded.is_infinite())?;

        Ok(encoded)
    }

    pub fn from_f32(value: f32) -> Self {
        Self::from_f128(value as f128)
    }

    pub fn to_f32(self) -> f32 {
        self.to_f128() as f32
    }

    #[cfg(feature = "f16")]
    pub fn from_f16(value: f16) -> Self {
        Self::from_f128(value as f128)
    }

    #[cfg(feature = "f16")]
    pub fn to_f16(self) -> f16 {
        self.to_f128() as f16
    }

    pub const fn exponent(self) -> u64 {
        (self.0 & Self::EXPONENT_MASK) >> Self::MANTISSA_BITS
    }

    pub const fn mantissa(self) -> u64 {
        self.0 & Self::MANTISSA_MASK
    }

    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub const fn is_nan(self) -> bool {
        self.exponent() == 0x7ff && self.mantissa() != 0
    }

    pub const fn is_infinite(self) -> bool {
        self.0 == Self::INFINITY.0
    }

    pub const fn is_finite(self) -> bool {
        self.exponent() != 0x7ff
    }

    pub const fn is_subnormal(self) -> bool {
        self.exponent() == 0 && self.mantissa() != 0
    }
}

impl From<f32> for Uf64E11M52 {
    fn from(value: f32) -> Self {
        Self::from_f32(value)
    }
}

impl From<f64> for Uf64E11M52 {
    fn from(value: f64) -> Self {
        Self::from_f64(value)
    }
}

#[cfg(feature = "f16")]
impl From<f16> for Uf64E11M52 {
    fn from(value: f16) -> Self {
        Self::from_f16(value)
    }
}

#[cfg(feature = "f16")]
impl From<Uf64E11M52> for f16 {
    fn from(value: Uf64E11M52) -> Self {
        value.to_f16()
    }
}

impl From<Uf64E11M52> for f32 {
    fn from(value: Uf64E11M52) -> Self {
        value.to_f32()
    }
}

impl From<Uf64E11M52> for f64 {
    fn from(value: Uf64E11M52) -> Self {
        value.to_f64()
    }
}

impl From<Uf64E11M52> for f128 {
    fn from(value: Uf64E11M52) -> Self {
        value.to_f128()
    }
}

impl TryFrom<f128> for Uf64E11M52 {
    type Error = ConversionError;

    fn try_from(value: f128) -> Result<Self, Self::Error> {
        Self::try_from_f128(value)
    }
}

impl Ord for Uf64E11M52 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Uf64E11M52 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Uf64E11M52 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(dispatch::add_uf64(self.0, rhs.0))
    }
}

impl Sub for Uf64E11M52 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(dispatch::sub_uf64(self.0, rhs.0))
    }
}

impl Mul for Uf64E11M52 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(dispatch::mul_uf64(self.0, rhs.0))
    }
}

impl Div for Uf64E11M52 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(dispatch::div_uf64(self.0, rhs.0))
    }
}

impl fmt::Debug for Uf64E11M52 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Uf64E11M52").field(&self.to_f64()).finish()
    }
}

impl_float_format!(Uf64E11M52, to_f64);
