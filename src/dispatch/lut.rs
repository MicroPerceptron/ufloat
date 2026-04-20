#[cfg(any(not(feature = "f16"), feature = "soft-float"))]
const UF8_E4M4_ADD: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e4m4_add.bin"));
#[cfg(any(not(feature = "f16"), feature = "soft-float"))]
const UF8_E4M4_SUB: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e4m4_sub.bin"));
#[cfg(any(not(feature = "f16"), feature = "soft-float"))]
const UF8_E4M4_MUL: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e4m4_mul.bin"));
#[cfg(any(not(feature = "f16"), feature = "soft-float"))]
const UF8_E4M4_DIV: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e4m4_div.bin"));

#[cfg(any(not(feature = "f16"), feature = "soft-float"))]
const UF8_E5M3_ADD: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e5m3_add.bin"));
#[cfg(any(not(feature = "f16"), feature = "soft-float"))]
const UF8_E5M3_SUB: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e5m3_sub.bin"));
#[cfg(any(not(feature = "f16"), feature = "soft-float"))]
const UF8_E5M3_MUL: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e5m3_mul.bin"));
#[cfg(any(not(feature = "f16"), feature = "soft-float"))]
const UF8_E5M3_DIV: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e5m3_div.bin"));

const UF8_E4M4_POW: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e4m4_pow.bin"));
const UF8_E4M4_POW1M: &[u8; 65536] =
    include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e4m4_pow1m.bin"));
const UF8_E5M3_POW: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e5m3_pow.bin"));
const UF8_E5M3_POW1M: &[u8; 65536] =
    include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e5m3_pow1m.bin"));

#[cfg(any(not(feature = "f16"), feature = "soft-float"))]
#[inline(always)]
pub(crate) fn add_uf8(a: u8, b: u8) -> u8 {
    UF8_E4M4_ADD[table_index(a, b)]
}

#[cfg(any(not(feature = "f16"), feature = "soft-float"))]
#[inline(always)]
pub(crate) fn sub_uf8(a: u8, b: u8) -> u8 {
    UF8_E4M4_SUB[table_index(a, b)]
}

#[cfg(any(not(feature = "f16"), feature = "soft-float"))]
#[inline(always)]
pub(crate) fn mul_uf8(a: u8, b: u8) -> u8 {
    UF8_E4M4_MUL[table_index(a, b)]
}

#[cfg(any(not(feature = "f16"), feature = "soft-float"))]
#[inline(always)]
pub(crate) fn div_uf8(a: u8, b: u8) -> u8 {
    UF8_E4M4_DIV[table_index(a, b)]
}

#[cfg(any(not(feature = "f16"), feature = "soft-float"))]
#[inline(always)]
pub(crate) fn add_uf8_e5m3(a: u8, b: u8) -> u8 {
    UF8_E5M3_ADD[table_index(a, b)]
}

#[cfg(any(not(feature = "f16"), feature = "soft-float"))]
#[inline(always)]
pub(crate) fn sub_uf8_e5m3(a: u8, b: u8) -> u8 {
    UF8_E5M3_SUB[table_index(a, b)]
}

#[cfg(any(not(feature = "f16"), feature = "soft-float"))]
#[inline(always)]
pub(crate) fn mul_uf8_e5m3(a: u8, b: u8) -> u8 {
    UF8_E5M3_MUL[table_index(a, b)]
}

#[cfg(any(not(feature = "f16"), feature = "soft-float"))]
#[inline(always)]
pub(crate) fn div_uf8_e5m3(a: u8, b: u8) -> u8 {
    UF8_E5M3_DIV[table_index(a, b)]
}

#[inline(always)]
pub(crate) fn pow_uf8(a: u8, b: u8) -> u8 {
    UF8_E4M4_POW[table_index(a, b)]
}

#[inline(always)]
pub(crate) fn pow1m_uf8(a: u8, b: u8) -> u8 {
    UF8_E4M4_POW1M[table_index(a, b)]
}

#[inline(always)]
pub(crate) fn pow_uf8_e5m3(a: u8, b: u8) -> u8 {
    UF8_E5M3_POW[table_index(a, b)]
}

#[inline(always)]
pub(crate) fn pow1m_uf8_e5m3(a: u8, b: u8) -> u8 {
    UF8_E5M3_POW1M[table_index(a, b)]
}

#[inline(always)]
const fn table_index(a: u8, b: u8) -> usize {
    ((a as usize) << 8) | b as usize
}
