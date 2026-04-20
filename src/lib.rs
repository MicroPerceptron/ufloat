//! Unsigned floating-point formats for values that can never be negative.
//!
//! This crate provides compact unsigned float newtypes with IEEE-like exponent
//! and mantissa fields, but no sign bit. The missing sign bit can be spent on
//! precision or range, removes negative zero, and makes total ordering a raw
//! unsigned integer comparison.
//!
//! The ergonomic aliases [`Uf8`], [`Uf16`], and [`Uf32`] point at the default
//! concrete layouts [`Uf8E4M4`], [`Uf16E5M11`], and [`Uf32E8M24`]. Alternate
//! layouts such as [`Uf8E5M3`] and [`Uf16E6M10`] are exported as distinct types
//! so their range and precision tradeoffs stay explicit.
//! With the `f128` feature enabled, [`Uf64`] is also available and promotes
//! through nightly primitive `f128`.
//!
//! # Conversions
//!
//! Explicit constructors such as [`Uf8::from_f32`] encode the input into the
//! target format. Negative native values become NaN, and overflow becomes
//! infinity.
//!
//! Use [`TryFrom`] when invalid or unrepresentable inputs should be rejected:
//!
//! ```
//! use unsigned_float::{ConversionError, Uf16};
//!
//! assert_eq!(Uf16::try_from(42_u32), Ok(Uf16::from_f32(42.0)));
//! assert_eq!(Uf16::try_from(-1_i32), Err(ConversionError::Negative));
//! ```
//!
//! # Exponents
//!
//! Use [`PowUf`] to raise native floats to unsigned-float exponents:
//!
//! ```
//! use unsigned_float::{PowUf, Uf16};
//!
//! let root = 9.0_f32.powuf(Uf16::from_f32(0.5));
//! assert_eq!(root, 3.0);
//! ```
//!
//! `PowUf` uses exact kernels for common exponent shapes such as zero, one,
//! one-half, and small integers, then falls back to `libm` for the general
//! fractional case.
//!
#![no_std]
#![cfg_attr(feature = "f16", feature(f16))]
#![cfg_attr(feature = "f128", feature(f128))]

#[cfg(test)]
extern crate std;

mod convert;
mod dispatch;
mod pow;
mod uf16;
mod uf32;
#[cfg(feature = "f128")]
mod uf64;
mod uf8;

pub use convert::ConversionError;
pub use pow::PowUf;
pub use uf8::{Uf8, Uf8E4M4, Uf8E5M3};
pub use uf16::{Uf16, Uf16E5M11, Uf16E6M10};
pub use uf32::{Uf32, Uf32E8M24};
#[cfg(feature = "f128")]
pub use uf64::{Uf64, Uf64E11M52};

#[cfg(test)]
mod tests {
    #[cfg(feature = "f128")]
    use super::Uf64;
    use super::{ConversionError, PowUf, Uf8, Uf8E5M3, Uf16, Uf16E6M10, Uf32};

    #[test]
    fn canonical_one_bits_match_the_layouts() {
        assert_eq!(Uf8::ONE.to_bits(), 0x70);
        assert_eq!(Uf8E5M3::ONE.to_bits(), 0x78);
        assert_eq!(Uf16::ONE.to_bits(), 0x7800);
        assert_eq!(Uf16E6M10::ONE.to_bits(), 0x7c00);
        assert_eq!(Uf32::ONE.to_bits(), 0x7f00_0000);
        #[cfg(feature = "f128")]
        assert_eq!(Uf64::ONE.to_bits(), 0x3ff0_0000_0000_0000);
    }

    #[test]
    fn uf8_finite_values_round_trip_through_f32() {
        for bits in u8::MIN..=u8::MAX {
            let value = Uf8::from_bits(bits);

            if value.is_nan() {
                continue;
            }

            assert_eq!(Uf8::from_f32(value.to_f32()).to_bits(), bits);
        }
    }

    #[test]
    fn uf8_e5m3_finite_values_round_trip_through_f32() {
        for bits in u8::MIN..=u8::MAX {
            let value = Uf8E5M3::from_bits(bits);

            if value.is_nan() {
                continue;
            }

            assert_eq!(Uf8E5M3::from_f32(value.to_f32()).to_bits(), bits);
        }
    }

    #[test]
    fn conversions_handle_special_values() {
        assert!(Uf8::from_f32(f32::NAN).is_nan());
        assert!(Uf8E5M3::from_f32(f32::NAN).is_nan());
        assert!(Uf16::from_f32(f32::NEG_INFINITY).is_nan());
        assert!(Uf16E6M10::from_f32(f32::NEG_INFINITY).is_nan());
        assert!(Uf32::from_f64(-1.0).is_nan());
        #[cfg(feature = "f128")]
        assert!(Uf64::from_f64(-1.0).is_nan());

        assert!(Uf8::from_f32(f32::INFINITY).is_infinite());
        assert!(Uf8E5M3::from_f32(f32::INFINITY).is_infinite());
        assert!(Uf16::from_f32(f32::INFINITY).is_infinite());
        assert!(Uf16E6M10::from_f32(f32::INFINITY).is_infinite());
        assert!(Uf32::from_f64(f64::INFINITY).is_infinite());
        #[cfg(feature = "f128")]
        assert!(Uf64::from_f64(f64::INFINITY).is_infinite());
    }

    #[test]
    fn try_from_f64_rejects_invalid_or_unrepresentable_values() {
        assert_eq!(Uf8::try_from(-1.0_f64), Err(ConversionError::Negative));
        assert_eq!(Uf16::try_from(f64::NAN), Err(ConversionError::Nan));
        assert_eq!(
            Uf32::try_from(f64::INFINITY),
            Err(ConversionError::Infinite)
        );

        assert_eq!(Uf8::try_from(1.0e20_f64), Err(ConversionError::Overflow));
        assert_eq!(Uf16::try_from(1.0e20_f64), Err(ConversionError::Overflow));
        assert_eq!(Uf8::try_from(1.0e-20_f64), Err(ConversionError::Underflow));

        assert_eq!(Uf8::try_from(2.0_f64), Ok(Uf8::from_f32(2.0)));
        assert_eq!(Uf8E5M3::try_from(2.0_f64), Ok(Uf8E5M3::from_f32(2.0)));
        assert_eq!(Uf16::try_from(2.0_f64), Ok(Uf16::from_f32(2.0)));
        assert_eq!(Uf16E6M10::try_from(2.0_f64), Ok(Uf16E6M10::from_f32(2.0)));
        assert_eq!(Uf32::try_from(2.0_f64), Ok(Uf32::from_f64(2.0)));
        #[cfg(feature = "f128")]
        assert_eq!(Uf64::try_from_f64(2.0_f64), Ok(Uf64::from_f64(2.0)));
    }

    #[test]
    fn try_from_integer_types() {
        assert_eq!(Uf8::try_from(2_u8), Ok(Uf8::from_f32(2.0)));
        assert_eq!(Uf8E5M3::try_from(2_u8), Ok(Uf8E5M3::from_f32(2.0)));
        assert_eq!(Uf16::try_from(1024_u32), Ok(Uf16::from_f32(1024.0)));
        assert_eq!(
            Uf16E6M10::try_from(1024_u32),
            Ok(Uf16E6M10::from_f32(1024.0))
        );
        assert_eq!(Uf32::try_from(1024_u64), Ok(Uf32::from_f64(1024.0)));
        #[cfg(feature = "f128")]
        assert_eq!(Uf64::try_from(1024_u64), Ok(Uf64::from_f64(1024.0)));

        assert_eq!(Uf8::try_from(-1_i8), Err(ConversionError::Negative));
        assert_eq!(Uf8::try_from(u128::MAX), Err(ConversionError::Overflow));
    }

    #[cfg(feature = "f16")]
    #[test]
    fn f16_conversions_are_available_when_enabled() {
        let native = 2.0_f16;

        assert_eq!(Uf8::from_f16(native).to_f16(), native);
        assert_eq!(Uf8E5M3::from_f16(native).to_f16(), native);
        assert_eq!(Uf16::from_f16(native).to_f16(), native);
        assert_eq!(Uf16E6M10::from_f16(native).to_f16(), native);
        assert_eq!(Uf32::from_f16(native).to_f16(), native);
        #[cfg(feature = "f128")]
        assert_eq!(Uf64::from_f16(native).to_f16(), native);

        assert_eq!(Uf8::from(native), Uf8::from_f16(native));
        assert_eq!(Uf8E5M3::from(native), Uf8E5M3::from_f16(native));
        assert_eq!(Uf16::from(native), Uf16::from_f16(native));
        assert_eq!(Uf16E6M10::from(native), Uf16E6M10::from_f16(native));
        assert_eq!(Uf32::from(native), Uf32::from_f16(native));
        #[cfg(feature = "f128")]
        assert_eq!(Uf64::from(native), Uf64::from_f16(native));

        let _: f16 = Uf8::from_f16(native).into();
        let _: f16 = Uf8E5M3::from_f16(native).into();
        let _: f16 = Uf16::from_f16(native).into();
        let _: f16 = Uf16E6M10::from_f16(native).into();
        let _: f16 = Uf32::from_f16(native).into();
        #[cfg(feature = "f128")]
        let _: f16 = Uf64::from_f16(native).into();
    }

    #[test]
    fn subnormal_values_decode_correctly() {
        assert_eq!(Uf8::MIN_POSITIVE.to_f32(), 2.0_f32.powi(-10));
        assert_eq!(Uf8E5M3::MIN_POSITIVE.to_f32(), 2.0_f32.powi(-17));
        assert_eq!(Uf16::MIN_POSITIVE.to_f32(), 2.0_f32.powi(-25));
        assert_eq!(Uf16E6M10::MIN_POSITIVE.to_f32(), 2.0_f32.powi(-40));
        assert_eq!(Uf32::MIN_POSITIVE.to_f64(), 2.0_f64.powi(-150));
        #[cfg(feature = "f128")]
        assert_eq!(
            Uf64::MIN_POSITIVE.to_f64(),
            f64::MIN_POSITIVE / 2.0_f64.powi(52)
        );
    }

    #[test]
    fn arithmetic_promotes_computes_and_demotes() {
        assert_eq!((Uf8::from_f32(1.0) + Uf8::from_f32(1.0)).to_f32(), 2.0);
        assert_eq!(
            (Uf8E5M3::from_f32(1.0) + Uf8E5M3::from_f32(1.0)).to_f32(),
            2.0
        );
        assert_eq!((Uf16::from_f32(3.0) * Uf16::from_f32(0.5)).to_f32(), 1.5);
        assert_eq!(
            (Uf16E6M10::from_f32(3.0) * Uf16E6M10::from_f32(0.5)).to_f32(),
            1.5
        );
        assert_eq!((Uf32::from_f64(9.0) / Uf32::from_f64(3.0)).to_f64(), 3.0);
        #[cfg(feature = "f128")]
        assert_eq!((Uf64::from_f64(9.0) / Uf64::from_f64(3.0)).to_f64(), 3.0);
    }

    #[test]
    fn native_float_bases_can_use_unsigned_float_exponents() {
        assert_eq!(9.0_f32.powuf(Uf8::from_f32(0.5)), 3.0);
        assert_eq!(9.0_f32.powuf(Uf8E5M3::from_f32(0.5)), 3.0);
        assert_eq!(9.0_f32.powuf(Uf16::from_f32(0.5)), 3.0);
        assert_eq!(9.0_f64.powuf(Uf16E6M10::from_f32(0.5)), 3.0);
        assert_eq!(9.0_f64.powuf(Uf32::from_f64(0.5)), 3.0);
        assert_eq!(2.0_f32.powuf(Uf16::from_f32(8.0)), 256.0);
        assert_eq!((-2.0_f32).powuf(Uf8::from_f32(3.0)), -8.0);
        assert_eq!(f32::NAN.powuf(Uf8::ZERO), 1.0);
        assert!((16.0_f64.powuf(Uf32::from_f64(1.25)) - 32.0).abs() < 1.0e-12);

        #[cfg(feature = "f128")]
        {
            assert_eq!(9.0_f64.powuf(Uf64::from_f64(0.5)), 3.0);
            assert_eq!(2.0_f64.powuf(Uf64::from_f64(10.0)), 1024.0);
        }
    }

    #[cfg(any(not(feature = "f16"), feature = "soft-float"))]
    #[test]
    fn uf8_lut_matches_promoted_arithmetic() {
        for a_bits in u8::MIN..=u8::MAX {
            for b_bits in u8::MIN..=u8::MAX {
                let a = Uf8::from_bits(a_bits);
                let b = Uf8::from_bits(b_bits);
                let a_f32 = a.to_f32();
                let b_f32 = b.to_f32();

                assert_eq!((a + b).to_bits(), Uf8::from_f32(a_f32 + b_f32).to_bits());
                assert_eq!((a - b).to_bits(), Uf8::from_f32(a_f32 - b_f32).to_bits());
                assert_eq!((a * b).to_bits(), Uf8::from_f32(a_f32 * b_f32).to_bits());
                assert_eq!((a / b).to_bits(), Uf8::from_f32(a_f32 / b_f32).to_bits());
            }
        }
    }

    #[cfg(any(not(feature = "f16"), feature = "soft-float"))]
    #[test]
    fn uf8_e5m3_lut_matches_promoted_arithmetic() {
        for a_bits in u8::MIN..=u8::MAX {
            for b_bits in u8::MIN..=u8::MAX {
                let a = Uf8E5M3::from_bits(a_bits);
                let b = Uf8E5M3::from_bits(b_bits);
                let a_f32 = a.to_f32();
                let b_f32 = b.to_f32();

                assert_eq!(
                    (a + b).to_bits(),
                    Uf8E5M3::from_f32(a_f32 + b_f32).to_bits()
                );
                assert_eq!(
                    (a - b).to_bits(),
                    Uf8E5M3::from_f32(a_f32 - b_f32).to_bits()
                );
                assert_eq!(
                    (a * b).to_bits(),
                    Uf8E5M3::from_f32(a_f32 * b_f32).to_bits()
                );
                assert_eq!(
                    (a / b).to_bits(),
                    Uf8E5M3::from_f32(a_f32 / b_f32).to_bits()
                );
            }
        }
    }

    #[test]
    fn negative_subtraction_result_is_nan() {
        assert!((Uf8::from_f32(1.0) - Uf8::from_f32(2.0)).is_nan());
        assert!((Uf8E5M3::from_f32(1.0) - Uf8E5M3::from_f32(2.0)).is_nan());
        assert!((Uf16::from_f32(1.0) - Uf16::from_f32(2.0)).is_nan());
        assert!((Uf16E6M10::from_f32(1.0) - Uf16E6M10::from_f32(2.0)).is_nan());
        assert!((Uf32::from_f64(1.0) - Uf32::from_f64(2.0)).is_nan());
        #[cfg(feature = "f128")]
        assert!((Uf64::from_f64(1.0) - Uf64::from_f64(2.0)).is_nan());
    }

    #[test]
    fn raw_bits_define_total_ordering() {
        assert!(Uf8::ZERO < Uf8::MIN_POSITIVE);
        assert!(Uf8::MAX < Uf8::INFINITY);
        assert!(Uf8::INFINITY < Uf8::NAN);

        assert!(Uf8E5M3::ZERO < Uf8E5M3::MIN_POSITIVE);
        assert!(Uf8E5M3::MAX < Uf8E5M3::INFINITY);
        assert!(Uf8E5M3::INFINITY < Uf8E5M3::NAN);

        assert!(Uf16::ZERO < Uf16::MIN_POSITIVE);
        assert!(Uf16::MAX < Uf16::INFINITY);
        assert!(Uf16::INFINITY < Uf16::NAN);

        assert!(Uf16E6M10::ZERO < Uf16E6M10::MIN_POSITIVE);
        assert!(Uf16E6M10::MAX < Uf16E6M10::INFINITY);
        assert!(Uf16E6M10::INFINITY < Uf16E6M10::NAN);

        assert!(Uf32::ZERO < Uf32::MIN_POSITIVE);
        assert!(Uf32::MAX < Uf32::INFINITY);
        assert!(Uf32::INFINITY < Uf32::NAN);

        #[cfg(feature = "f128")]
        {
            assert!(Uf64::ZERO < Uf64::MIN_POSITIVE);
            assert!(Uf64::MAX < Uf64::INFINITY);
            assert!(Uf64::INFINITY < Uf64::NAN);
        }
    }

    #[test]
    fn round_to_nearest_even_when_encoding() {
        assert_eq!(Uf8::from_f32(1.0 + 1.0 / 32.0).to_bits(), 0x70);
        assert_eq!(Uf8::from_f32(1.0 + 3.0 / 32.0).to_bits(), 0x72);

        assert_eq!(
            Uf32::from_f64(1.0 + 2.0_f64.powi(-25)).to_bits(),
            Uf32::ONE.to_bits()
        );
        assert_eq!(
            Uf32::from_f64(1.0 + 3.0 * 2.0_f64.powi(-25)).to_bits(),
            Uf32::ONE.to_bits() + 2
        );
    }
}
