#[cfg(feature = "f128")]
use crate::Uf64;
use crate::{Uf8, Uf8E5M3, Uf16, Uf16E6M10, Uf32};

/// Extension trait for raising native floats to unsigned-float exponents.
///
/// This mirrors the shape of native `powf` methods while keeping the crate
/// dependency-free.
pub trait PowUf<Rhs> {
    /// The result type of exponentiation.
    type Output;

    /// Raises `self` to the unsigned-float exponent `rhs`.
    fn powuf(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_powuf_f32 {
    ($($rhs:ty),* $(,)?) => {
        $(
            impl PowUf<$rhs> for f32 {
                type Output = f32;

                #[inline]
                fn powuf(self, rhs: $rhs) -> Self::Output {
                    powuf_f32(self, rhs.to_f32())
                }
            }
        )*
    };
}

macro_rules! impl_powuf_f64 {
    ($($rhs:ty),* $(,)?) => {
        $(
            impl PowUf<$rhs> for f64 {
                type Output = f64;

                #[inline]
                fn powuf(self, rhs: $rhs) -> Self::Output {
                    powuf_f64(self, rhs.to_f64())
                }
            }
        )*
    };
}

impl_powuf_f32!(Uf8, Uf8E5M3, Uf16, Uf16E6M10, Uf32);
impl_powuf_f64!(Uf8, Uf8E5M3, Uf16, Uf16E6M10, Uf32);

#[cfg(feature = "f128")]
impl PowUf<Uf64> for f32 {
    type Output = f32;

    #[inline]
    fn powuf(self, rhs: Uf64) -> Self::Output {
        powuf_f32(self, rhs.to_f32())
    }
}

#[cfg(feature = "f128")]
impl PowUf<Uf64> for f64 {
    type Output = f64;

    #[inline]
    fn powuf(self, rhs: Uf64) -> Self::Output {
        powuf_f64(self, rhs.to_f64())
    }
}

#[inline]
fn powuf_f32(base: f32, exponent: f32) -> f32 {
    if exponent == 0.0 {
        1.0
    } else if exponent == 1.0 {
        base
    } else if exponent == 0.5 {
        libm::sqrtf(base)
    } else if let Some(integer) = small_integer_exponent_f32(exponent) {
        powi_u32_f32(base, integer)
    } else {
        libm::powf(base, exponent)
    }
}

#[inline]
fn powuf_f64(base: f64, exponent: f64) -> f64 {
    if exponent == 0.0 {
        1.0
    } else if exponent == 1.0 {
        base
    } else if exponent == 0.5 {
        libm::sqrt(base)
    } else if let Some(integer) = small_integer_exponent_f64(exponent) {
        powi_u32_f64(base, integer)
    } else {
        libm::pow(base, exponent)
    }
}

#[inline]
fn small_integer_exponent_f32(exponent: f32) -> Option<u32> {
    if !(2.0..=32.0).contains(&exponent) {
        return None;
    }

    let integer = exponent as u32;
    if integer as f32 == exponent {
        Some(integer)
    } else {
        None
    }
}

#[inline]
fn small_integer_exponent_f64(exponent: f64) -> Option<u32> {
    if !(2.0..=32.0).contains(&exponent) {
        return None;
    }

    let integer = exponent as u32;
    if integer as f64 == exponent {
        Some(integer)
    } else {
        None
    }
}

#[inline]
fn powi_u32_f32(mut base: f32, mut exponent: u32) -> f32 {
    let mut acc = 1.0;

    while exponent != 0 {
        if exponent & 1 == 1 {
            acc *= base;
        }

        exponent >>= 1;
        if exponent != 0 {
            base *= base;
        }
    }

    acc
}

#[inline]
fn powi_u32_f64(mut base: f64, mut exponent: u32) -> f64 {
    let mut acc = 1.0;

    while exponent != 0 {
        if exponent & 1 == 1 {
            acc *= base;
        }

        exponent >>= 1;
        if exponent != 0 {
            base *= base;
        }
    }

    acc
}
