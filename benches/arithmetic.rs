#![feature(test)]

extern crate test;

use test::{Bencher, black_box};
#[cfg(feature = "f128")]
use unsigned_float::Uf64;
use unsigned_float::{Pow1mUf, PowUf, Uf8, Uf8E5M3, Uf16, Uf16E6M10, Uf32};

const F32_INPUTS: [f32; 16] = [
    0.0,
    0.000_976_562_5,
    0.03125,
    0.125,
    0.5,
    0.75,
    1.0,
    1.25,
    1.5,
    2.0,
    3.0,
    7.5,
    16.0,
    64.0,
    256.0,
    1024.0,
];

const F64_INPUTS: [f64; 16] = [
    0.0,
    0.000_000_000_931_322_574_615_478_5,
    0.000_976_562_5,
    0.125,
    0.5,
    0.75,
    1.0,
    1.25,
    1.5,
    2.0,
    3.0,
    7.5,
    16.0,
    64.0,
    256.0,
    1024.0,
];

const POW_REPEATS: usize = 64;

const UF8_INPUTS: [Uf8; 16] = [
    Uf8::ZERO,
    Uf8::MIN_POSITIVE,
    Uf8::MIN_NORMAL,
    Uf8::from_bits(0x30),
    Uf8::from_bits(0x60),
    Uf8::from_bits(0x68),
    Uf8::ONE,
    Uf8::from_bits(0x74),
    Uf8::from_bits(0x78),
    Uf8::from_bits(0x80),
    Uf8::from_bits(0x88),
    Uf8::from_bits(0x98),
    Uf8::from_bits(0xb0),
    Uf8::from_bits(0xc0),
    Uf8::from_bits(0xd0),
    Uf8::MAX,
];

const UF8_E5M3_INPUTS: [Uf8E5M3; 16] = [
    Uf8E5M3::ZERO,
    Uf8E5M3::MIN_POSITIVE,
    Uf8E5M3::MIN_NORMAL,
    Uf8E5M3::from_bits(0x50),
    Uf8E5M3::from_bits(0x70),
    Uf8E5M3::from_bits(0x74),
    Uf8E5M3::ONE,
    Uf8E5M3::from_bits(0x7a),
    Uf8E5M3::from_bits(0x7c),
    Uf8E5M3::from_bits(0x80),
    Uf8E5M3::from_bits(0x84),
    Uf8E5M3::from_bits(0x8f),
    Uf8E5M3::from_bits(0x98),
    Uf8E5M3::from_bits(0xa8),
    Uf8E5M3::from_bits(0xb8),
    Uf8E5M3::MAX,
];

const UF16_INPUTS: [Uf16; 16] = [
    Uf16::ZERO,
    Uf16::MIN_POSITIVE,
    Uf16::MIN_NORMAL,
    Uf16::from_bits(0x6000),
    Uf16::from_bits(0x7000),
    Uf16::from_bits(0x7400),
    Uf16::ONE,
    Uf16::from_bits(0x7a00),
    Uf16::from_bits(0x7c00),
    Uf16::from_bits(0x8000),
    Uf16::from_bits(0x8400),
    Uf16::from_bits(0x8780),
    Uf16::from_bits(0x9800),
    Uf16::from_bits(0xa800),
    Uf16::from_bits(0xb800),
    Uf16::MAX,
];

const UF16_E6M10_INPUTS: [Uf16E6M10; 16] = [
    Uf16E6M10::ZERO,
    Uf16E6M10::MIN_POSITIVE,
    Uf16E6M10::MIN_NORMAL,
    Uf16E6M10::from_bits(0x7000),
    Uf16E6M10::from_bits(0x7800),
    Uf16E6M10::from_bits(0x7a00),
    Uf16E6M10::ONE,
    Uf16E6M10::from_bits(0x7d00),
    Uf16E6M10::from_bits(0x7e00),
    Uf16E6M10::from_bits(0x8000),
    Uf16E6M10::from_bits(0x8200),
    Uf16E6M10::from_bits(0x83c0),
    Uf16E6M10::from_bits(0x8800),
    Uf16E6M10::from_bits(0x9000),
    Uf16E6M10::from_bits(0x9800),
    Uf16E6M10::MAX,
];

const UF32_INPUTS: [Uf32; 16] = [
    Uf32::ZERO,
    Uf32::MIN_POSITIVE,
    Uf32::MIN_NORMAL,
    Uf32::from_bits(0x7c00_0000),
    Uf32::from_bits(0x7e00_0000),
    Uf32::from_bits(0x7e80_0000),
    Uf32::ONE,
    Uf32::from_bits(0x7f40_0000),
    Uf32::from_bits(0x7f80_0000),
    Uf32::from_bits(0x8000_0000),
    Uf32::from_bits(0x8040_0000),
    Uf32::from_bits(0x80e0_0000),
    Uf32::from_bits(0x8300_0000),
    Uf32::from_bits(0x8500_0000),
    Uf32::from_bits(0x8700_0000),
    Uf32::MAX,
];

#[cfg(feature = "f128")]
const UF64_INPUTS: [Uf64; 16] = [
    Uf64::ZERO,
    Uf64::MIN_POSITIVE,
    Uf64::MIN_NORMAL,
    Uf64::from_bits(0x3fc0_0000_0000_0000),
    Uf64::from_bits(0x3fe0_0000_0000_0000),
    Uf64::from_bits(0x3fe8_0000_0000_0000),
    Uf64::ONE,
    Uf64::from_bits(0x3ff4_0000_0000_0000),
    Uf64::from_bits(0x3ff8_0000_0000_0000),
    Uf64::from_bits(0x4000_0000_0000_0000),
    Uf64::from_bits(0x4008_0000_0000_0000),
    Uf64::from_bits(0x401e_0000_0000_0000),
    Uf64::from_bits(0x4030_0000_0000_0000),
    Uf64::from_bits(0x4050_0000_0000_0000),
    Uf64::from_bits(0x4070_0000_0000_0000),
    Uf64::MAX,
];

macro_rules! bench_from_f32 {
    ($name:ident, $ty:ty) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                let mut acc = 0_u32;
                for value in F32_INPUTS {
                    acc ^= <$ty>::from_f32(black_box(value)).to_bits() as u32;
                }
                black_box(acc)
            });
        }
    };
}

macro_rules! bench_to_f32 {
    ($name:ident, $inputs:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                let mut acc = 0.0_f32;
                for value in $inputs {
                    acc += black_box(value).to_f32();
                }
                black_box(acc)
            });
        }
    };
}

macro_rules! bench_binary_op {
    ($name:ident, $inputs:ident, $op:tt) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                let mut acc = $inputs[0];
                for lhs in $inputs {
                    for rhs in $inputs {
                        acc = black_box(lhs) $op black_box(rhs);
                    }
                }
                black_box(acc)
            });
        }
    };
}

macro_rules! bench_ordering {
    ($name:ident, $inputs:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                let mut acc = 0_usize;
                for lhs in $inputs {
                    for rhs in $inputs {
                        acc ^= black_box(lhs < rhs) as usize;
                    }
                }
                black_box(acc)
            });
        }
    };
}

macro_rules! bench_powuf_f32 {
    ($name:ident, $inputs:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                let mut acc = 0.0_f32;
                for _ in 0..POW_REPEATS {
                    for exponent in $inputs {
                        acc += black_box(9.0_f32).powuf(black_box(exponent));
                    }
                }
                black_box(acc)
            });
        }
    };
}

macro_rules! bench_powuf_f64 {
    ($name:ident, $inputs:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                let mut acc = 0.0_f64;
                for _ in 0..POW_REPEATS {
                    for exponent in $inputs {
                        acc += black_box(9.0_f64).powuf(black_box(exponent));
                    }
                }
                black_box(acc)
            });
        }
    };
}

macro_rules! bench_pow1muf_f32 {
    ($name:ident, $inputs:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                let mut acc = 0.0_f32;
                for _ in 0..POW_REPEATS {
                    for exponent in $inputs {
                        acc += black_box(0.75_f32).pow1muf(black_box(exponent));
                    }
                }
                black_box(acc)
            });
        }
    };
}

macro_rules! bench_pow1muf_f64 {
    ($name:ident, $inputs:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                let mut acc = 0.0_f64;
                for _ in 0..POW_REPEATS {
                    for exponent in $inputs {
                        acc += black_box(0.75_f64).pow1muf(black_box(exponent));
                    }
                }
                black_box(acc)
            });
        }
    };
}

macro_rules! bench_compact_powuf {
    ($name:ident, $inputs:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                let mut acc = 0_u8;
                for _ in 0..POW_REPEATS {
                    for base in $inputs {
                        for exponent in $inputs {
                            acc ^= black_box(base).powuf(black_box(exponent)).to_bits();
                        }
                    }
                }
                black_box(acc)
            });
        }
    };
}

macro_rules! bench_compact_pow1muf {
    ($name:ident, $inputs:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                let mut acc = 0_u8;
                for _ in 0..POW_REPEATS {
                    for u in $inputs {
                        for exponent in $inputs {
                            acc ^= black_box(u).pow1muf(black_box(exponent)).to_bits();
                        }
                    }
                }
                black_box(acc)
            });
        }
    };
}

bench_from_f32!(uf8_from_f32, Uf8);
bench_from_f32!(uf8_e5m3_from_f32, Uf8E5M3);
bench_from_f32!(uf16_from_f32, Uf16);
bench_from_f32!(uf16_e6m10_from_f32, Uf16E6M10);
bench_from_f32!(uf32_from_f32, Uf32);
#[cfg(feature = "f128")]
bench_from_f32!(uf64_from_f32, Uf64);

#[bench]
fn uf32_from_f64(b: &mut Bencher) {
    b.iter(|| {
        let mut acc = 0_u32;
        for value in F64_INPUTS {
            acc ^= Uf32::from_f64(black_box(value)).to_bits();
        }
        black_box(acc)
    });
}

bench_to_f32!(uf8_to_f32, UF8_INPUTS);
bench_to_f32!(uf8_e5m3_to_f32, UF8_E5M3_INPUTS);
bench_to_f32!(uf16_to_f32, UF16_INPUTS);
bench_to_f32!(uf16_e6m10_to_f32, UF16_E6M10_INPUTS);
bench_to_f32!(uf32_to_f32, UF32_INPUTS);
#[cfg(feature = "f128")]
bench_to_f32!(uf64_to_f32, UF64_INPUTS);

#[bench]
fn uf32_to_f64(b: &mut Bencher) {
    b.iter(|| {
        let mut acc = 0.0_f64;
        for value in UF32_INPUTS {
            acc += black_box(value).to_f64();
        }
        black_box(acc)
    });
}

#[cfg(feature = "f128")]
#[bench]
fn uf64_from_f64(b: &mut Bencher) {
    b.iter(|| {
        let mut acc = 0_u64;
        for value in F64_INPUTS {
            acc ^= Uf64::from_f64(black_box(value)).to_bits();
        }
        black_box(acc)
    });
}

#[cfg(feature = "f128")]
#[bench]
fn uf64_to_f64(b: &mut Bencher) {
    b.iter(|| {
        let mut acc = 0.0_f64;
        for value in UF64_INPUTS {
            acc += black_box(value).to_f64();
        }
        black_box(acc)
    });
}

bench_binary_op!(uf8_add, UF8_INPUTS, +);
bench_binary_op!(uf8_sub, UF8_INPUTS, -);
bench_binary_op!(uf8_mul, UF8_INPUTS, *);
bench_binary_op!(uf8_div, UF8_INPUTS, /);
bench_compact_powuf!(uf8_powuf, UF8_INPUTS);
bench_compact_pow1muf!(uf8_pow1muf, UF8_INPUTS);

bench_binary_op!(uf8_e5m3_add, UF8_E5M3_INPUTS, +);
bench_binary_op!(uf8_e5m3_sub, UF8_E5M3_INPUTS, -);
bench_binary_op!(uf8_e5m3_mul, UF8_E5M3_INPUTS, *);
bench_binary_op!(uf8_e5m3_div, UF8_E5M3_INPUTS, /);
bench_compact_powuf!(uf8_e5m3_powuf, UF8_E5M3_INPUTS);
bench_compact_pow1muf!(uf8_e5m3_pow1muf, UF8_E5M3_INPUTS);

bench_binary_op!(uf16_add, UF16_INPUTS, +);
bench_binary_op!(uf16_sub, UF16_INPUTS, -);
bench_binary_op!(uf16_mul, UF16_INPUTS, *);
bench_binary_op!(uf16_div, UF16_INPUTS, /);

bench_binary_op!(uf16_e6m10_add, UF16_E6M10_INPUTS, +);
bench_binary_op!(uf16_e6m10_sub, UF16_E6M10_INPUTS, -);
bench_binary_op!(uf16_e6m10_mul, UF16_E6M10_INPUTS, *);
bench_binary_op!(uf16_e6m10_div, UF16_E6M10_INPUTS, /);

bench_binary_op!(uf32_add, UF32_INPUTS, +);
bench_binary_op!(uf32_sub, UF32_INPUTS, -);
bench_binary_op!(uf32_mul, UF32_INPUTS, *);
bench_binary_op!(uf32_div, UF32_INPUTS, /);

#[cfg(feature = "f128")]
bench_binary_op!(uf64_add, UF64_INPUTS, +);
#[cfg(feature = "f128")]
bench_binary_op!(uf64_sub, UF64_INPUTS, -);
#[cfg(feature = "f128")]
bench_binary_op!(uf64_mul, UF64_INPUTS, *);
#[cfg(feature = "f128")]
bench_binary_op!(uf64_div, UF64_INPUTS, /);

bench_ordering!(uf8_ordering, UF8_INPUTS);
bench_ordering!(uf8_e5m3_ordering, UF8_E5M3_INPUTS);
bench_ordering!(uf16_ordering, UF16_INPUTS);
bench_ordering!(uf16_e6m10_ordering, UF16_E6M10_INPUTS);
bench_ordering!(uf32_ordering, UF32_INPUTS);
#[cfg(feature = "f128")]
bench_ordering!(uf64_ordering, UF64_INPUTS);

bench_powuf_f32!(f32_powuf_uf8, UF8_INPUTS);
bench_powuf_f32!(f32_powuf_uf8_e5m3, UF8_E5M3_INPUTS);
bench_powuf_f32!(f32_powuf_uf16, UF16_INPUTS);
bench_powuf_f32!(f32_powuf_uf16_e6m10, UF16_E6M10_INPUTS);
bench_powuf_f32!(f32_powuf_uf32, UF32_INPUTS);
#[cfg(feature = "f128")]
bench_powuf_f32!(f32_powuf_uf64, UF64_INPUTS);

bench_pow1muf_f32!(f32_pow1muf_uf8, UF8_INPUTS);
bench_pow1muf_f32!(f32_pow1muf_uf8_e5m3, UF8_E5M3_INPUTS);
bench_pow1muf_f32!(f32_pow1muf_uf16, UF16_INPUTS);
bench_pow1muf_f32!(f32_pow1muf_uf16_e6m10, UF16_E6M10_INPUTS);
bench_pow1muf_f32!(f32_pow1muf_uf32, UF32_INPUTS);
#[cfg(feature = "f128")]
bench_pow1muf_f32!(f32_pow1muf_uf64, UF64_INPUTS);

bench_powuf_f64!(f64_powuf_uf8, UF8_INPUTS);
bench_powuf_f64!(f64_powuf_uf8_e5m3, UF8_E5M3_INPUTS);
bench_powuf_f64!(f64_powuf_uf16, UF16_INPUTS);
bench_powuf_f64!(f64_powuf_uf16_e6m10, UF16_E6M10_INPUTS);
bench_powuf_f64!(f64_powuf_uf32, UF32_INPUTS);
#[cfg(feature = "f128")]
bench_powuf_f64!(f64_powuf_uf64, UF64_INPUTS);

bench_pow1muf_f64!(f64_pow1muf_uf8, UF8_INPUTS);
bench_pow1muf_f64!(f64_pow1muf_uf8_e5m3, UF8_E5M3_INPUTS);
bench_pow1muf_f64!(f64_pow1muf_uf16, UF16_INPUTS);
bench_pow1muf_f64!(f64_pow1muf_uf16_e6m10, UF16_E6M10_INPUTS);
bench_pow1muf_f64!(f64_pow1muf_uf32, UF32_INPUTS);
#[cfg(feature = "f128")]
bench_pow1muf_f64!(f64_pow1muf_uf64, UF64_INPUTS);
