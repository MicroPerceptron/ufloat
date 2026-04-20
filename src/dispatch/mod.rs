mod soft;

#[cfg(all(feature = "f16", not(feature = "soft-float")))]
mod fp16;

#[cfg(any(not(feature = "f16"), feature = "soft-float"))]
mod lut;

const UF8_EXP_BITS: u32 = 4;
const UF8_MANTISSA_BITS: u32 = 4;
const UF8_BIAS: i32 = 7;

const UF16_EXP_BITS: u32 = 5;
const UF16_MANTISSA_BITS: u32 = 11;
const UF16_BIAS: i32 = 15;

const UF32_EXP_BITS: u32 = 8;
const UF32_MANTISSA_BITS: u32 = 24;
const UF32_BIAS: i32 = 127;

pub(crate) fn uf8_to_f32(bits: u8) -> f32 {
    soft::decode_to_f64(bits as u64, UF8_EXP_BITS, UF8_MANTISSA_BITS, UF8_BIAS) as f32
}

pub(crate) fn f32_to_uf8(value: f32) -> u8 {
    soft::encode_from_f64(value as f64, UF8_EXP_BITS, UF8_MANTISSA_BITS, UF8_BIAS) as u8
}

pub(crate) fn uf16_to_f32(bits: u16) -> f32 {
    let exp = (bits >> UF16_MANTISSA_BITS) & ((1 << UF16_EXP_BITS) - 1);
    let mantissa = bits & ((1 << UF16_MANTISSA_BITS) - 1);

    if exp == 0 || exp == ((1 << UF16_EXP_BITS) - 1) {
        return soft::decode_to_f64(bits as u64, UF16_EXP_BITS, UF16_MANTISSA_BITS, UF16_BIAS)
            as f32;
    }

    let f32_exp = (exp as i32 - UF16_BIAS + 127) as u32;
    let f32_mantissa = (mantissa as u32) << (23 - UF16_MANTISSA_BITS);
    f32::from_bits((f32_exp << 23) | f32_mantissa)
}

pub(crate) fn f32_to_uf16(value: f32) -> u16 {
    soft::encode_from_f64(value as f64, UF16_EXP_BITS, UF16_MANTISSA_BITS, UF16_BIAS) as u16
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

pub(crate) fn f64_to_uf32(value: f64) -> u32 {
    soft::encode_from_f64(value, UF32_EXP_BITS, UF32_MANTISSA_BITS, UF32_BIAS) as u32
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
