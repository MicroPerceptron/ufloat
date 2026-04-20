mod soft;

#[cfg(all(feature = "f16", not(feature = "soft-float")))]
mod fp16;

#[cfg(any(not(feature = "f16"), feature = "soft-float"))]
mod lut;

const UF8_EXP_BITS: u32 = 4;
const UF8_MANTISSA_BITS: u32 = 4;
const UF8_BIAS: i32 = 7;

const UF8_E5M3_EXP_BITS: u32 = 5;
const UF8_E5M3_MANTISSA_BITS: u32 = 3;
const UF8_E5M3_BIAS: i32 = 15;

const UF16_EXP_BITS: u32 = 5;
const UF16_MANTISSA_BITS: u32 = 11;
const UF16_BIAS: i32 = 15;

const UF16_E6M10_EXP_BITS: u32 = 6;
const UF16_E6M10_MANTISSA_BITS: u32 = 10;
const UF16_E6M10_BIAS: i32 = 31;

const UF32_EXP_BITS: u32 = 8;
const UF32_MANTISSA_BITS: u32 = 24;
const UF32_BIAS: i32 = 127;

#[cfg(feature = "f128")]
const UF64_EXP_BITS: u32 = 11;
#[cfg(feature = "f128")]
const UF64_MANTISSA_BITS: u32 = 52;
#[cfg(feature = "f128")]
const UF64_BIAS: i32 = 1023;

pub(crate) fn uf8_to_f32(bits: u8) -> f32 {
    soft::decode_to_f64(bits as u64, UF8_EXP_BITS, UF8_MANTISSA_BITS, UF8_BIAS) as f32
}

pub(crate) fn f32_to_uf8(value: f32) -> u8 {
    soft::encode_from_f64(value as f64, UF8_EXP_BITS, UF8_MANTISSA_BITS, UF8_BIAS) as u8
}

pub(crate) fn uf8_e5m3_to_f32(bits: u8) -> f32 {
    soft::decode_to_f64(
        bits as u64,
        UF8_E5M3_EXP_BITS,
        UF8_E5M3_MANTISSA_BITS,
        UF8_E5M3_BIAS,
    ) as f32
}

pub(crate) fn f32_to_uf8_e5m3(value: f32) -> u8 {
    soft::encode_from_f64(
        value as f64,
        UF8_E5M3_EXP_BITS,
        UF8_E5M3_MANTISSA_BITS,
        UF8_E5M3_BIAS,
    ) as u8
}

pub(crate) fn uf16_to_f32(bits: u16) -> f32 {
    uf_to_f32(bits as u64, UF16_EXP_BITS, UF16_MANTISSA_BITS, UF16_BIAS)
}

pub(crate) fn f32_to_uf16(value: f32) -> u16 {
    soft::encode_from_f64(value as f64, UF16_EXP_BITS, UF16_MANTISSA_BITS, UF16_BIAS) as u16
}

pub(crate) fn uf16_e6m10_to_f32(bits: u16) -> f32 {
    uf_to_f32(
        bits as u64,
        UF16_E6M10_EXP_BITS,
        UF16_E6M10_MANTISSA_BITS,
        UF16_E6M10_BIAS,
    )
}

pub(crate) fn f32_to_uf16_e6m10(value: f32) -> u16 {
    soft::encode_from_f64(
        value as f64,
        UF16_E6M10_EXP_BITS,
        UF16_E6M10_MANTISSA_BITS,
        UF16_E6M10_BIAS,
    ) as u16
}

pub(crate) fn uf32_to_f64(bits: u32) -> f64 {
    let exp = (bits >> UF32_MANTISSA_BITS) & ((1 << UF32_EXP_BITS) - 1);
    let mantissa = bits & ((1 << UF32_MANTISSA_BITS) - 1);

    if exp == 0 || exp == ((1 << UF32_EXP_BITS) - 1) {
        return soft::decode_to_f64(bits as u64, UF32_EXP_BITS, UF32_MANTISSA_BITS, UF32_BIAS);
    }

    let f64_exp = (exp as i32 - UF32_BIAS + 1023) as u64;
    let f64_mantissa = (mantissa as u64) << (52 - UF32_MANTISSA_BITS);
    f64::from_bits((f64_exp << 52) | f64_mantissa)
}

fn uf_to_f32(bits: u64, exp_bits: u32, mantissa_bits: u32, bias: i32) -> f32 {
    let exp = (bits >> mantissa_bits) & ((1 << exp_bits) - 1);
    let mantissa = bits & ((1 << mantissa_bits) - 1);

    if exp == 0 || exp == ((1 << exp_bits) - 1) {
        return soft::decode_to_f64(bits, exp_bits, mantissa_bits, bias) as f32;
    }

    let f32_exp = (exp as i32 - bias + 127) as u32;
    let f32_mantissa = (mantissa as u32) << (23 - mantissa_bits);
    f32::from_bits((f32_exp << 23) | f32_mantissa)
}

pub(crate) fn f64_to_uf32(value: f64) -> u32 {
    soft::encode_from_f64(value, UF32_EXP_BITS, UF32_MANTISSA_BITS, UF32_BIAS) as u32
}

#[cfg(feature = "f128")]
pub(crate) fn uf64_to_f128(bits: u64) -> f128 {
    let exp = (bits >> UF64_MANTISSA_BITS) & ((1 << UF64_EXP_BITS) - 1);
    let mantissa = bits & ((1 << UF64_MANTISSA_BITS) - 1);

    if exp == 0 || exp == ((1 << UF64_EXP_BITS) - 1) {
        return uf64_special_to_f128(exp, mantissa);
    }

    let f128_exp = (exp as i32 - UF64_BIAS + 16383) as u128;
    let f128_mantissa = (mantissa as u128) << (112 - UF64_MANTISSA_BITS);
    f128::from_bits((f128_exp << 112) | f128_mantissa)
}

#[cfg(feature = "f128")]
pub(crate) fn f128_to_uf64(value: f128) -> u64 {
    let bits = value.to_bits();
    let sign = bits >> 127;
    let f128_exp = ((bits >> 112) & 0x7fff) as i32;
    let f128_fraction = bits & ((1_u128 << 112) - 1);
    let max_exp = (1_u64 << UF64_EXP_BITS) - 1;
    let quiet_nan = (max_exp << UF64_MANTISSA_BITS) | (1_u64 << (UF64_MANTISSA_BITS - 1));

    if f128_exp == 0x7fff {
        return if f128_fraction == 0 && sign == 0 {
            max_exp << UF64_MANTISSA_BITS
        } else {
            quiet_nan
        };
    }

    if (bits << 1) == 0 {
        return 0;
    }

    if sign != 0 {
        return quiet_nan;
    }

    let (significand, exponent) = if f128_exp == 0 {
        (f128_fraction, 1 - 16383 - 112)
    } else {
        ((1_u128 << 112) | f128_fraction, f128_exp - 16383 - 112)
    };

    if significand == 0 {
        return 0;
    }

    let max_finite_unbiased = max_exp as i32 - 1 - UF64_BIAS;
    let min_normal_unbiased = 1 - UF64_BIAS;
    let value_unbiased = floor_log2_u128(significand) as i32 + exponent;

    if value_unbiased > max_finite_unbiased {
        return max_exp << UF64_MANTISSA_BITS;
    }

    if value_unbiased < min_normal_unbiased {
        let scale = min_normal_unbiased - UF64_MANTISSA_BITS as i32;
        let rounded = round_shift_u128(significand, exponent - scale);

        if rounded == 0 {
            return 0;
        }

        if rounded >= (1_u128 << UF64_MANTISSA_BITS) {
            return 1_u64 << UF64_MANTISSA_BITS;
        }

        return rounded as u64;
    }

    let mut unbiased = value_unbiased;
    let shift = UF64_MANTISSA_BITS as i32 - floor_log2_u128(significand) as i32;
    let mut rounded = round_shift_u128(significand, shift);

    if rounded == (1_u128 << (UF64_MANTISSA_BITS + 1)) {
        rounded >>= 1;
        unbiased += 1;
    }

    let biased = unbiased + UF64_BIAS;
    if biased >= max_exp as i32 {
        return max_exp << UF64_MANTISSA_BITS;
    }

    let mantissa = (rounded - (1_u128 << UF64_MANTISSA_BITS)) as u64;
    ((biased as u64) << UF64_MANTISSA_BITS) | mantissa
}

#[cfg(feature = "f128")]
fn uf64_special_to_f128(exp: u64, mantissa: u64) -> f128 {
    let max_exp = (1 << UF64_EXP_BITS) - 1;

    if exp == max_exp {
        let f128_mantissa = if mantissa == 0 {
            0
        } else {
            (mantissa as u128) << (112 - UF64_MANTISSA_BITS)
        };
        return f128::from_bits((0x7fff_u128 << 112) | f128_mantissa);
    }

    if mantissa == 0 {
        return f128::from_bits(0);
    }

    let top = u64::BITS - 1 - mantissa.leading_zeros();
    let unbiased = top as i32 + 1 - UF64_BIAS - UF64_MANTISSA_BITS as i32;
    let f128_exp = (unbiased + 16383) as u128;
    let f128_mantissa = ((mantissa as u128) - (1_u128 << top)) << (112 - top);
    f128::from_bits((f128_exp << 112) | f128_mantissa)
}

#[cfg(feature = "f128")]
fn floor_log2_u128(value: u128) -> u32 {
    u128::BITS - 1 - value.leading_zeros()
}

#[cfg(feature = "f128")]
fn round_shift_u128(value: u128, shift: i32) -> u128 {
    if shift >= 0 {
        let shift = shift as u32;
        return if shift >= u128::BITS {
            u128::MAX
        } else {
            value << shift
        };
    }

    let shift = (-shift) as u32;
    if shift >= u128::BITS {
        return 0;
    }

    let quotient = value >> shift;
    let remainder_mask = (1_u128 << shift) - 1;
    let remainder = value & remainder_mask;
    let halfway = 1_u128 << (shift - 1);

    if remainder > halfway || (remainder == halfway && (quotient & 1) == 1) {
        quotient + 1
    } else {
        quotient
    }
}

pub(crate) fn add_uf8(a: u8, b: u8) -> u8 {
    #[cfg(all(feature = "f16", not(feature = "soft-float")))]
    {
        fp16::add_uf8(a, b)
    }

    #[cfg(any(not(feature = "f16"), feature = "soft-float"))]
    {
        lut::add_uf8(a, b)
    }
}

pub(crate) fn sub_uf8(a: u8, b: u8) -> u8 {
    #[cfg(all(feature = "f16", not(feature = "soft-float")))]
    {
        fp16::sub_uf8(a, b)
    }

    #[cfg(any(not(feature = "f16"), feature = "soft-float"))]
    {
        lut::sub_uf8(a, b)
    }
}

pub(crate) fn mul_uf8(a: u8, b: u8) -> u8 {
    #[cfg(all(feature = "f16", not(feature = "soft-float")))]
    {
        fp16::mul_uf8(a, b)
    }

    #[cfg(any(not(feature = "f16"), feature = "soft-float"))]
    {
        lut::mul_uf8(a, b)
    }
}

pub(crate) fn div_uf8(a: u8, b: u8) -> u8 {
    #[cfg(all(feature = "f16", not(feature = "soft-float")))]
    {
        fp16::div_uf8(a, b)
    }

    #[cfg(any(not(feature = "f16"), feature = "soft-float"))]
    {
        lut::div_uf8(a, b)
    }
}

pub(crate) fn add_uf8_e5m3(a: u8, b: u8) -> u8 {
    #[cfg(all(feature = "f16", not(feature = "soft-float")))]
    {
        fp16::add_uf8_e5m3(a, b)
    }

    #[cfg(any(not(feature = "f16"), feature = "soft-float"))]
    {
        lut::add_uf8_e5m3(a, b)
    }
}

pub(crate) fn sub_uf8_e5m3(a: u8, b: u8) -> u8 {
    #[cfg(all(feature = "f16", not(feature = "soft-float")))]
    {
        fp16::sub_uf8_e5m3(a, b)
    }

    #[cfg(any(not(feature = "f16"), feature = "soft-float"))]
    {
        lut::sub_uf8_e5m3(a, b)
    }
}

pub(crate) fn mul_uf8_e5m3(a: u8, b: u8) -> u8 {
    #[cfg(all(feature = "f16", not(feature = "soft-float")))]
    {
        fp16::mul_uf8_e5m3(a, b)
    }

    #[cfg(any(not(feature = "f16"), feature = "soft-float"))]
    {
        lut::mul_uf8_e5m3(a, b)
    }
}

pub(crate) fn div_uf8_e5m3(a: u8, b: u8) -> u8 {
    #[cfg(all(feature = "f16", not(feature = "soft-float")))]
    {
        fp16::div_uf8_e5m3(a, b)
    }

    #[cfg(any(not(feature = "f16"), feature = "soft-float"))]
    {
        lut::div_uf8_e5m3(a, b)
    }
}

pub(crate) fn add_uf16(a: u16, b: u16) -> u16 {
    f32_to_uf16(uf16_to_f32(a) + uf16_to_f32(b))
}

pub(crate) fn sub_uf16(a: u16, b: u16) -> u16 {
    f32_to_uf16(uf16_to_f32(a) - uf16_to_f32(b))
}

pub(crate) fn mul_uf16(a: u16, b: u16) -> u16 {
    f32_to_uf16(uf16_to_f32(a) * uf16_to_f32(b))
}

pub(crate) fn div_uf16(a: u16, b: u16) -> u16 {
    f32_to_uf16(uf16_to_f32(a) / uf16_to_f32(b))
}

pub(crate) fn add_uf16_e6m10(a: u16, b: u16) -> u16 {
    f32_to_uf16_e6m10(uf16_e6m10_to_f32(a) + uf16_e6m10_to_f32(b))
}

pub(crate) fn sub_uf16_e6m10(a: u16, b: u16) -> u16 {
    f32_to_uf16_e6m10(uf16_e6m10_to_f32(a) - uf16_e6m10_to_f32(b))
}

pub(crate) fn mul_uf16_e6m10(a: u16, b: u16) -> u16 {
    f32_to_uf16_e6m10(uf16_e6m10_to_f32(a) * uf16_e6m10_to_f32(b))
}

pub(crate) fn div_uf16_e6m10(a: u16, b: u16) -> u16 {
    f32_to_uf16_e6m10(uf16_e6m10_to_f32(a) / uf16_e6m10_to_f32(b))
}

pub(crate) fn add_uf32(a: u32, b: u32) -> u32 {
    f64_to_uf32(uf32_to_f64(a) + uf32_to_f64(b))
}

pub(crate) fn sub_uf32(a: u32, b: u32) -> u32 {
    f64_to_uf32(uf32_to_f64(a) - uf32_to_f64(b))
}

pub(crate) fn mul_uf32(a: u32, b: u32) -> u32 {
    f64_to_uf32(uf32_to_f64(a) * uf32_to_f64(b))
}

pub(crate) fn div_uf32(a: u32, b: u32) -> u32 {
    f64_to_uf32(uf32_to_f64(a) / uf32_to_f64(b))
}

#[cfg(feature = "f128")]
pub(crate) fn add_uf64(a: u64, b: u64) -> u64 {
    f128_to_uf64(uf64_to_f128(a) + uf64_to_f128(b))
}

#[cfg(feature = "f128")]
pub(crate) fn sub_uf64(a: u64, b: u64) -> u64 {
    f128_to_uf64(uf64_to_f128(a) - uf64_to_f128(b))
}

#[cfg(feature = "f128")]
pub(crate) fn mul_uf64(a: u64, b: u64) -> u64 {
    f128_to_uf64(uf64_to_f128(a) * uf64_to_f128(b))
}

#[cfg(feature = "f128")]
pub(crate) fn div_uf64(a: u64, b: u64) -> u64 {
    f128_to_uf64(uf64_to_f128(a) / uf64_to_f128(b))
}
