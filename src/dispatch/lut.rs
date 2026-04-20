const UF8_ADD: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_add.bin"));
const UF8_SUB: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_sub.bin"));
const UF8_MUL: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_mul.bin"));
const UF8_DIV: &[u8; 65536] = include_bytes!(concat!(env!("OUT_DIR"), "/uf8_div.bin"));

#[inline(always)]
pub(crate) fn add_uf8(a: u8, b: u8) -> u8 {
    UF8_ADD[table_index(a, b)]
}

#[inline(always)]
pub(crate) fn sub_uf8(a: u8, b: u8) -> u8 {
    UF8_SUB[table_index(a, b)]
}

#[inline(always)]
pub(crate) fn mul_uf8(a: u8, b: u8) -> u8 {
    UF8_MUL[table_index(a, b)]
}

#[inline(always)]
pub(crate) fn div_uf8(a: u8, b: u8) -> u8 {
    UF8_DIV[table_index(a, b)]
}

#[inline(always)]
const fn table_index(a: u8, b: u8) -> usize {
    ((a as usize) << 8) | b as usize
}
