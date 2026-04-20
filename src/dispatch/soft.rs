pub(crate) fn decode_to_f64(bits: u64, exp_bits: u32, mantissa_bits: u32, bias: i32) -> f64 {
    let exp_mask = (1_u64 << exp_bits) - 1;
    let mantissa_mask = (1_u64 << mantissa_bits) - 1;
    let exp = (bits >> mantissa_bits) & exp_mask;
    let mantissa = bits & mantissa_mask;

    if exp == exp_mask {
        return if mantissa == 0 {
            f64::INFINITY
        } else {
            f64::NAN
        };
    }

    if exp == 0 {
        if mantissa == 0 {
            return 0.0;
        }

        let top = floor_log2(mantissa);
        let unbiased_exp = top as i32 + 1 - bias - mantissa_bits as i32;
        let f64_exp = (unbiased_exp + 1023) as u64;
        let f64_mantissa = (mantissa - (1_u64 << top)) << (52 - top);
        return f64::from_bits((f64_exp << 52) | f64_mantissa);
    }

    let unbiased_exp = exp as i32 - bias;
    let f64_exp = (unbiased_exp + 1023) as u64;
    let f64_mantissa = mantissa << (52 - mantissa_bits);
    f64::from_bits((f64_exp << 52) | f64_mantissa)
}

pub(crate) fn encode_from_f64(value: f64, exp_bits: u32, mantissa_bits: u32, bias: i32) -> u64 {
    let bits = value.to_bits();
    let sign = bits >> 63;
    let f64_exp = ((bits >> 52) & 0x7ff) as i32;
    let f64_fraction = bits & ((1_u64 << 52) - 1);
    let max_exp = (1_u64 << exp_bits) - 1;
    let quiet_nan = (max_exp << mantissa_bits) | quiet_nan_payload(mantissa_bits);

    if f64_exp == 0x7ff {
        return if f64_fraction == 0 && sign == 0 {
            max_exp << mantissa_bits
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

    let (significand, exponent) = if f64_exp == 0 {
        (f64_fraction, 1 - 1023 - 52)
    } else {
        ((1_u64 << 52) | f64_fraction, f64_exp - 1023 - 52)
    };

    if significand == 0 {
        return 0;
    }

    let max_finite_unbiased = max_exp as i32 - 1 - bias;
    let min_normal_unbiased = 1 - bias;
    let value_unbiased = floor_log2(significand) as i32 + exponent;

    if value_unbiased > max_finite_unbiased {
        return max_exp << mantissa_bits;
    }

    if value_unbiased < min_normal_unbiased {
        let scale = min_normal_unbiased - mantissa_bits as i32;
        let rounded = round_shift(significand as u128, exponent - scale);

        if rounded == 0 {
            return 0;
        }

        if rounded >= (1_u128 << mantissa_bits) {
            return 1_u64 << mantissa_bits;
        }

        return rounded as u64;
    }

    let mut unbiased = value_unbiased;
    let shift = mantissa_bits as i32 - floor_log2(significand) as i32;
    let mut rounded = round_shift(significand as u128, shift);

    if rounded == (1_u128 << (mantissa_bits + 1)) {
        rounded >>= 1;
        unbiased += 1;
    }

    let biased = unbiased + bias;
    if biased >= max_exp as i32 {
        return max_exp << mantissa_bits;
    }

    let mantissa = (rounded - (1_u128 << mantissa_bits)) as u64;
    ((biased as u64) << mantissa_bits) | mantissa
}

fn quiet_nan_payload(mantissa_bits: u32) -> u64 {
    if mantissa_bits == 0 {
        0
    } else {
        1_u64 << (mantissa_bits - 1)
    }
}

fn floor_log2(value: u64) -> u32 {
    u64::BITS - 1 - value.leading_zeros()
}

fn round_shift(value: u128, shift: i32) -> u128 {
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
