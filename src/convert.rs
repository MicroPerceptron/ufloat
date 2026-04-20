use core::fmt;

#[cfg(feature = "f128")]
use crate::Uf64;
use crate::{Uf8, Uf8E5M3, Uf16, Uf16E6M10, Uf32};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Error returned by fallible conversions into unsigned float types.
pub enum ConversionError {
    /// The source value was less than zero.
    Negative,
    /// The source value was NaN.
    Nan,
    /// The source value was infinite.
    Infinite,
    /// The source value was too large for the target format.
    Overflow,
    /// The positive source value was too small for the target format.
    Underflow,
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negative => f.write_str("value is negative"),
            Self::Nan => f.write_str("value is NaN"),
            Self::Infinite => f.write_str("value is infinite"),
            Self::Overflow => f.write_str("value is too large"),
            Self::Underflow => f.write_str("positive value is too small"),
        }
    }
}

pub(crate) fn check_finite_non_negative(value: f64) -> Result<(), ConversionError> {
    if value.is_nan() {
        Err(ConversionError::Nan)
    } else if value.is_infinite() {
        Err(ConversionError::Infinite)
    } else if value < 0.0 {
        Err(ConversionError::Negative)
    } else {
        Ok(())
    }
}

pub(crate) fn check_encoded(
    value: f64,
    is_zero: bool,
    is_infinite: bool,
) -> Result<(), ConversionError> {
    if is_infinite {
        Err(ConversionError::Overflow)
    } else if value != 0.0 && is_zero {
        Err(ConversionError::Underflow)
    } else {
        Ok(())
    }
}

macro_rules! impl_try_from_float {
    ($float:ty, $($uf:ty),* $(,)?) => {
        $(
            impl TryFrom<$float> for $uf {
                type Error = ConversionError;

                fn try_from(value: $float) -> Result<Self, Self::Error> {
                    Self::try_from_f64(value as f64)
                }
            }
        )*
    };
}

macro_rules! impl_try_from_unsigned_int {
    ($($int:ty),* $(,)?) => {
        $(
            impl TryFrom<$int> for Uf8 {
                type Error = ConversionError;

                fn try_from(value: $int) -> Result<Self, Self::Error> {
                    Self::try_from_f64(value as f64)
                }
            }

            impl TryFrom<$int> for Uf16 {
                type Error = ConversionError;

                fn try_from(value: $int) -> Result<Self, Self::Error> {
                    Self::try_from_f64(value as f64)
                }
            }

            impl TryFrom<$int> for Uf8E5M3 {
                type Error = ConversionError;

                fn try_from(value: $int) -> Result<Self, Self::Error> {
                    Self::try_from_f64(value as f64)
                }
            }

            impl TryFrom<$int> for Uf16E6M10 {
                type Error = ConversionError;

                fn try_from(value: $int) -> Result<Self, Self::Error> {
                    Self::try_from_f64(value as f64)
                }
            }

            impl TryFrom<$int> for Uf32 {
                type Error = ConversionError;

                fn try_from(value: $int) -> Result<Self, Self::Error> {
                    Self::try_from_f64(value as f64)
                }
            }
        )*
    };
}

macro_rules! impl_try_from_signed_int {
    ($($int:ty),* $(,)?) => {
        $(
            impl TryFrom<$int> for Uf8 {
                type Error = ConversionError;

                fn try_from(value: $int) -> Result<Self, Self::Error> {
                    if value < 0 {
                        Err(ConversionError::Negative)
                    } else {
                        Self::try_from_f64(value as f64)
                    }
                }
            }

            impl TryFrom<$int> for Uf16 {
                type Error = ConversionError;

                fn try_from(value: $int) -> Result<Self, Self::Error> {
                    if value < 0 {
                        Err(ConversionError::Negative)
                    } else {
                        Self::try_from_f64(value as f64)
                    }
                }
            }

            impl TryFrom<$int> for Uf8E5M3 {
                type Error = ConversionError;

                fn try_from(value: $int) -> Result<Self, Self::Error> {
                    if value < 0 {
                        Err(ConversionError::Negative)
                    } else {
                        Self::try_from_f64(value as f64)
                    }
                }
            }

            impl TryFrom<$int> for Uf16E6M10 {
                type Error = ConversionError;

                fn try_from(value: $int) -> Result<Self, Self::Error> {
                    if value < 0 {
                        Err(ConversionError::Negative)
                    } else {
                        Self::try_from_f64(value as f64)
                    }
                }
            }

            impl TryFrom<$int> for Uf32 {
                type Error = ConversionError;

                fn try_from(value: $int) -> Result<Self, Self::Error> {
                    if value < 0 {
                        Err(ConversionError::Negative)
                    } else {
                        Self::try_from_f64(value as f64)
                    }
                }
            }
        )*
    };
}

impl_try_from_float!(f64, Uf8, Uf8E5M3, Uf16, Uf16E6M10, Uf32);

impl_try_from_unsigned_int!(u8, u16, u32, u64, u128, usize);
impl_try_from_signed_int!(i8, i16, i32, i64, i128, isize);

#[cfg(feature = "f128")]
macro_rules! impl_try_from_unsigned_int_uf64 {
    ($($int:ty),* $(,)?) => {
        $(
            impl TryFrom<$int> for Uf64 {
                type Error = ConversionError;

                fn try_from(value: $int) -> Result<Self, Self::Error> {
                    Self::try_from_f128(value as f128)
                }
            }
        )*
    };
}

#[cfg(feature = "f128")]
macro_rules! impl_try_from_signed_int_uf64 {
    ($($int:ty),* $(,)?) => {
        $(
            impl TryFrom<$int> for Uf64 {
                type Error = ConversionError;

                fn try_from(value: $int) -> Result<Self, Self::Error> {
                    if value < 0 {
                        Err(ConversionError::Negative)
                    } else {
                        Self::try_from_f128(value as f128)
                    }
                }
            }
        )*
    };
}

#[cfg(feature = "f128")]
impl_try_from_unsigned_int_uf64!(u8, u16, u32, u64, u128, usize);

#[cfg(feature = "f128")]
impl_try_from_signed_int_uf64!(i8, i16, i32, i64, i128, isize);
