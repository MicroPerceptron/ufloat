use super::{
    UF8_BIAS, UF8_E5M3_BIAS, UF8_E5M3_EXP_BITS, UF8_E5M3_MANTISSA_BITS, UF8_EXP_BITS,
    UF8_MANTISSA_BITS, f32_to_uf8, f32_to_uf8_e5m3,
};

pub(crate) fn add_uf8(a: u8, b: u8) -> u8 {
    f16_to_uf8(uf8_to_f16(a) + uf8_to_f16(b))
}

pub(crate) fn sub_uf8(a: u8, b: u8) -> u8 {
    f16_to_uf8(uf8_to_f16(a) - uf8_to_f16(b))
}

pub(crate) fn mul_uf8(a: u8, b: u8) -> u8 {
    f16_to_uf8(uf8_to_f16(a) * uf8_to_f16(b))
}

pub(crate) fn div_uf8(a: u8, b: u8) -> u8 {
    f16_to_uf8(uf8_to_f16(a) / uf8_to_f16(b))
}

fn uf8_to_f16(bits: u8) -> f16 {
    let exp_mask = ((1_u16 << UF8_EXP_BITS) - 1) as u8;
    let mantissa_mask = ((1_u16 << UF8_MANTISSA_BITS) - 1) as u8;
    let exp = (bits >> UF8_MANTISSA_BITS) & exp_mask;
    let mantissa = bits & mantissa_mask;

    if exp == exp_mask {
        let fp16_mantissa = if mantissa == 0 {
            0
        } else {
            (mantissa as u16) << (10 - UF8_MANTISSA_BITS)
        };
        return f16::from_bits((0x1f << 10) | fp16_mantissa);
    }

    if exp == 0 {
        if mantissa == 0 {
            return f16::from_bits(0);
        }

        let top = u8::BITS - 1 - mantissa.leading_zeros();
        let unbiased = top as i32 + 1 - UF8_BIAS - UF8_MANTISSA_BITS as i32;
        let fp16_exp = (unbiased + 15) as u16;
        let fp16_mantissa = ((mantissa as u16) - (1_u16 << top)) << (10 - top);
        return f16::from_bits((fp16_exp << 10) | fp16_mantissa);
    }

    let fp16_exp = (exp as i32 - UF8_BIAS + 15) as u16;
    let fp16_mantissa = (mantissa as u16) << (10 - UF8_MANTISSA_BITS);
    f16::from_bits((fp16_exp << 10) | fp16_mantissa)
}

fn f16_to_uf8(value: f16) -> u8 {
    f32_to_uf8(value as f32)
}

pub(crate) fn add_uf8_e5m3(a: u8, b: u8) -> u8 {
    f16_to_uf8_e5m3(uf8_e5m3_to_f16(a) + uf8_e5m3_to_f16(b))
}

pub(crate) fn sub_uf8_e5m3(a: u8, b: u8) -> u8 {
    f16_to_uf8_e5m3(uf8_e5m3_to_f16(a) - uf8_e5m3_to_f16(b))
}

pub(crate) fn mul_uf8_e5m3(a: u8, b: u8) -> u8 {
    f16_to_uf8_e5m3(uf8_e5m3_to_f16(a) * uf8_e5m3_to_f16(b))
}

pub(crate) fn div_uf8_e5m3(a: u8, b: u8) -> u8 {
    f16_to_uf8_e5m3(uf8_e5m3_to_f16(a) / uf8_e5m3_to_f16(b))
}

fn uf8_e5m3_to_f16(bits: u8) -> f16 {
    uf8_layout_to_f16(
        bits,
        UF8_E5M3_EXP_BITS,
        UF8_E5M3_MANTISSA_BITS,
        UF8_E5M3_BIAS,
    )
}

fn f16_to_uf8_e5m3(value: f16) -> u8 {
    f32_to_uf8_e5m3(value as f32)
}

fn uf8_layout_to_f16(bits: u8, exp_bits: u32, mantissa_bits: u32, bias: i32) -> f16 {
    let exp_mask = ((1_u16 << exp_bits) - 1) as u8;
    let mantissa_mask = ((1_u16 << mantissa_bits) - 1) as u8;
    let exp = (bits >> mantissa_bits) & exp_mask;
    let mantissa = bits & mantissa_mask;

    if exp == exp_mask {
        let fp16_mantissa = if mantissa == 0 {
            0
        } else {
            (mantissa as u16) << (10 - mantissa_bits)
        };
        return f16::from_bits((0x1f << 10) | fp16_mantissa);
    }

    if exp == 0 {
        if mantissa == 0 {
            return f16::from_bits(0);
        }

        let top = u8::BITS - 1 - mantissa.leading_zeros();
        let unbiased = top as i32 + 1 - bias - mantissa_bits as i32;
        let fp16_exp = (unbiased + 15) as u16;
        let fp16_mantissa = ((mantissa as u16) - (1_u16 << top)) << (10 - top);
        return f16::from_bits((fp16_exp << 10) | fp16_mantissa);
    }

    let fp16_exp = (exp as i32 - bias + 15) as u16;
    let fp16_mantissa = (mantissa as u16) << (10 - mantissa_bits);
    f16::from_bits((fp16_exp << 10) | fp16_mantissa)
}
