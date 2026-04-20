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

fn table_index(a: u8, b: u8) -> usize {
    ((a as usize) << 8) | b as usize
}

fn uf8_to_f32(bits: u8, exp_bits: u32, mantissa_bits: u32, bias: i32) -> f32 {
    soft::decode_to_f64(bits as u64, exp_bits, mantissa_bits, bias) as f32
}

fn f32_to_uf8(value: f32, exp_bits: u32, mantissa_bits: u32, bias: i32) -> u8 {
    soft::encode_from_f64(value as f64, exp_bits, mantissa_bits, bias) as u8
}
