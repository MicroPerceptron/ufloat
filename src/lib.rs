#![no_std]
#![cfg_attr(all(feature = "f16", not(feature = "soft-float")), feature(f16))]

#[cfg(test)]
extern crate std;

mod dispatch;
mod uf16;
mod uf32;
mod uf8;

pub use uf8::Uf8;
pub use uf16::Uf16;
pub use uf32::Uf32;

#[cfg(test)]
mod tests {
    use super::{Uf8, Uf16, Uf32};

    #[test]
    fn canonical_one_bits_match_the_layouts() {
        assert_eq!(Uf8::ONE.to_bits(), 0x70);
        assert_eq!(Uf16::ONE.to_bits(), 0x7800);
        assert_eq!(Uf32::ONE.to_bits(), 0x7f00_0000);
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
    fn conversions_handle_special_values() {
        assert!(Uf8::from_f32(f32::NAN).is_nan());
        assert!(Uf16::from_f32(f32::NEG_INFINITY).is_nan());
        assert!(Uf32::from_f64(-1.0).is_nan());

        assert!(Uf8::from_f32(f32::INFINITY).is_infinite());
        assert!(Uf16::from_f32(f32::INFINITY).is_infinite());
        assert!(Uf32::from_f64(f64::INFINITY).is_infinite());
    }

    #[test]
    fn subnormal_values_decode_correctly() {
        assert_eq!(Uf8::MIN_POSITIVE.to_f32(), 2.0_f32.powi(-10));
        assert_eq!(Uf16::MIN_POSITIVE.to_f32(), 2.0_f32.powi(-25));
        assert_eq!(Uf32::MIN_POSITIVE.to_f64(), 2.0_f64.powi(-150));
    }

    #[test]
    fn arithmetic_promotes_computes_and_demotes() {
        assert_eq!((Uf8::from_f32(1.0) + Uf8::from_f32(1.0)).to_f32(), 2.0);
        assert_eq!((Uf16::from_f32(3.0) * Uf16::from_f32(0.5)).to_f32(), 1.5);
        assert_eq!((Uf32::from_f64(9.0) / Uf32::from_f64(3.0)).to_f64(), 3.0);
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

    #[test]
    fn negative_subtraction_result_is_nan() {
        assert!((Uf8::from_f32(1.0) - Uf8::from_f32(2.0)).is_nan());
        assert!((Uf16::from_f32(1.0) - Uf16::from_f32(2.0)).is_nan());
        assert!((Uf32::from_f64(1.0) - Uf32::from_f64(2.0)).is_nan());
    }

    #[test]
    fn raw_bits_define_total_ordering() {
        assert!(Uf8::ZERO < Uf8::MIN_POSITIVE);
        assert!(Uf8::MAX < Uf8::INFINITY);
        assert!(Uf8::INFINITY < Uf8::NAN);

        assert!(Uf16::ZERO < Uf16::MIN_POSITIVE);
        assert!(Uf16::MAX < Uf16::INFINITY);
        assert!(Uf16::INFINITY < Uf16::NAN);

        assert!(Uf32::ZERO < Uf32::MIN_POSITIVE);
        assert!(Uf32::MAX < Uf32::INFINITY);
        assert!(Uf32::INFINITY < Uf32::NAN);
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
