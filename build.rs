use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[path = "src/dispatch/soft.rs"]
mod soft;

const UF8_EXP_BITS: u32 = 4;
const UF8_MANTISSA_BITS: u32 = 4;
const UF8_BIAS: i32 = 7;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set by Cargo"));

    write_uf8_table(&out_dir, "uf8_add.bin", |a, b| a + b);
    write_uf8_table(&out_dir, "uf8_sub.bin", |a, b| a - b);
    write_uf8_table(&out_dir, "uf8_mul.bin", |a, b| a * b);
    write_uf8_table(&out_dir, "uf8_div.bin", |a, b| a / b);
}

fn write_uf8_table(out_dir: &Path, file_name: &str, op: fn(f32, f32) -> f32) {
    let mut table = vec![0; 256 * 256];

    for a in u8::MIN..=u8::MAX {
        for b in u8::MIN..=u8::MAX {
            let index = table_index(a, b);
            table[index] = f32_to_uf8(op(uf8_to_f32(a), uf8_to_f32(b)));
        }
    }

    fs::write(out_dir.join(file_name), table).expect("write UF8 lookup table");
}

fn table_index(a: u8, b: u8) -> usize {
    ((a as usize) << 8) | b as usize
}

fn uf8_to_f32(bits: u8) -> f32 {
    soft::decode_to_f64(bits as u64, UF8_EXP_BITS, UF8_MANTISSA_BITS, UF8_BIAS) as f32
}

fn f32_to_uf8(value: f32) -> u8 {
    soft::encode_from_f64(value as f64, UF8_EXP_BITS, UF8_MANTISSA_BITS, UF8_BIAS) as u8
}
