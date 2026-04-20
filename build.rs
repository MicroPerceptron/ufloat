use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[path = "src/dispatch/soft.rs"]
mod soft;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set by Cargo"));

    write_uf8_layout_tables(&out_dir, "uf8_e4m4", 4, 4, 7);
    write_uf8_layout_tables(&out_dir, "uf8_e5m3", 5, 3, 15);
    write_uf8_cross_layout_pow_tables(&out_dir, "uf8_e4m4", "uf8_e5m3", (4, 4, 7), (5, 3, 15));
    write_uf8_cross_layout_pow_tables(&out_dir, "uf8_e5m3", "uf8_e4m4", (5, 3, 15), (4, 4, 7));
}

fn write_uf8_layout_tables(
    out_dir: &Path,
    name: &str,
    exp_bits: u32,
    mantissa_bits: u32,
    bias: i32,
) {
    write_uf8_table(
        out_dir,
        &format!("{name}_add.bin"),
        exp_bits,
        mantissa_bits,
        bias,
        |a, b| a + b,
    );
    write_uf8_table(
        out_dir,
        &format!("{name}_sub.bin"),
        exp_bits,
        mantissa_bits,
        bias,
        |a, b| a - b,
    );
    write_uf8_table(
        out_dir,
        &format!("{name}_mul.bin"),
        exp_bits,
        mantissa_bits,
        bias,
        |a, b| a * b,
    );
    write_uf8_table(
        out_dir,
        &format!("{name}_div.bin"),
        exp_bits,
        mantissa_bits,
        bias,
        |a, b| a / b,
    );
    write_uf8_table(
        out_dir,
        &format!("{name}_pow.bin"),
        exp_bits,
        mantissa_bits,
        bias,
        powuf_f32,
    );
    write_uf8_table(
        out_dir,
        &format!("{name}_pow1m.bin"),
        exp_bits,
        mantissa_bits,
        bias,
        pow1muf_f32,
    );
}

fn write_uf8_table(
    out_dir: &Path,
    file_name: &str,
    exp_bits: u32,
    mantissa_bits: u32,
    bias: i32,
    op: fn(f32, f32) -> f32,
) {
    let mut table = vec![0; 256 * 256];

    for a in u8::MIN..=u8::MAX {
        for b in u8::MIN..=u8::MAX {
            let index = table_index(a, b);
            let a = uf8_to_f32(a, exp_bits, mantissa_bits, bias);
            let b = uf8_to_f32(b, exp_bits, mantissa_bits, bias);
            table[index] = f32_to_uf8(op(a, b), exp_bits, mantissa_bits, bias);
        }
    }

    fs::write(out_dir.join(file_name), table).expect("write UF8 lookup table");
}

fn write_uf8_cross_layout_pow_tables(
    out_dir: &Path,
    base_name: &str,
    exponent_name: &str,
    base_layout: (u32, u32, i32),
    exponent_layout: (u32, u32, i32),
) {
    write_uf8_cross_layout_table(
        out_dir,
        &format!("{base_name}_pow_{exponent_name}.bin"),
        base_layout,
        exponent_layout,
        powuf_f32,
    );
    write_uf8_cross_layout_table(
        out_dir,
        &format!("{base_name}_pow1m_{exponent_name}.bin"),
        base_layout,
        exponent_layout,
        pow1muf_f32,
    );
}

fn write_uf8_cross_layout_table(
    out_dir: &Path,
    file_name: &str,
    base_layout: (u32, u32, i32),
    exponent_layout: (u32, u32, i32),
    op: fn(f32, f32) -> f32,
) {
    let mut table = vec![0; 256 * 256];
    let (base_exp_bits, base_mantissa_bits, base_bias) = base_layout;
    let (exponent_exp_bits, exponent_mantissa_bits, exponent_bias) = exponent_layout;

    for a in u8::MIN..=u8::MAX {
        for b in u8::MIN..=u8::MAX {
            let index = table_index(a, b);
            let a = uf8_to_f32(a, base_exp_bits, base_mantissa_bits, base_bias);
            let b = uf8_to_f32(b, exponent_exp_bits, exponent_mantissa_bits, exponent_bias);
            table[index] = f32_to_uf8(op(a, b), base_exp_bits, base_mantissa_bits, base_bias);
        }
    }

    fs::write(out_dir.join(file_name), table).expect("write UF8 cross-layout lookup table");
}

fn table_index(a: u8, b: u8) -> usize {
    ((a as usize) << 8) | b as usize
}

fn uf8_to_f32(bits: u8, exp_bits: u32, mantissa_bits: u32, bias: i32) -> f32 {
    soft::decode_to_f64(bits as u64, exp_bits, mantissa_bits, bias) as f32
}

fn f32_to_uf8(value: f32, exp_bits: u32, mantissa_bits: u32, bias: i32) -> u8 {
    soft::encode_from_f64(value as f64, exp_bits, mantissa_bits, bias) as u8
}

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

fn pow1muf_f32(u: f32, exponent: f32) -> f32 {
    if exponent == 0.0 || u == 0.0 {
        1.0
    } else if u == 1.0 {
        0.0
    } else if !(0.0..=1.0).contains(&u) {
        powuf_f32(1.0 - u, exponent)
    } else if exponent == 0.5 {
        libm::sqrtf(1.0 - u)
    } else {
        libm::expm1f(exponent * libm::log1pf(-u)) + 1.0
    }
}

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
