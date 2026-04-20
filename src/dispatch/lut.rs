const UF8_E4M4_ADD: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e4m4_add.bin"));
const UF8_E4M4_SUB: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e4m4_sub.bin"));
const UF8_E4M4_MUL: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e4m4_mul.bin"));
const UF8_E4M4_DIV: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e4m4_div.bin"));

const UF8_E5M3_ADD: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e5m3_add.bin"));
const UF8_E5M3_SUB: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e5m3_sub.bin"));
const UF8_E5M3_MUL: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e5m3_mul.bin"));
const UF8_E5M3_DIV: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_e5m3_div.bin"));

#[inline(always)]
pub(crate) fn add_uf8(a: u8, b: u8) -> u8 {
    UF8_E4M4_ADD[table_index(a, b)]
}

#[inline(always)]
pub(crate) fn sub_uf8(a: u8, b: u8) -> u8 {
    UF8_E4M4_SUB[table_index(a, b)]
}

#[inline(always)]
pub(crate) fn mul_uf8(a: u8, b: u8) -> u8 {
    UF8_E4M4_MUL[table_index(a, b)]
}

#[inline(always)]
pub(crate) fn div_uf8(a: u8, b: u8) -> u8 {
    UF8_E4M4_DIV[table_index(a, b)]
}

#[inline(always)]
pub(crate) fn add_uf8_e5m3(a: u8, b: u8) -> u8 {
    UF8_E5M3_ADD[table_index(a, b)]
}

#[inline(always)]
pub(crate) fn sub_uf8_e5m3(a: u8, b: u8) -> u8 {
    UF8_E5M3_SUB[table_index(a, b)]
}

#[inline(always)]
pub(crate) fn mul_uf8_e5m3(a: u8, b: u8) -> u8 {
    UF8_E5M3_MUL[table_index(a, b)]
}

#[inline(always)]
pub(crate) fn div_uf8_e5m3(a: u8, b: u8) -> u8 {
    UF8_E5M3_DIV[table_index(a, b)]
}

#[inline(always)]
const fn table_index(a: u8, b: u8) -> usize {
    ((a as usize) << 8) | b as usize
}
