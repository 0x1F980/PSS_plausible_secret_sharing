// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use pss::pss_field_gf256::Gf256;
use pss::pss_lagrange::lagrange_at_zero;

#[test]
fn gf256_field_roundtrip() {
    let a = Gf256::from_u8(83);
    let b = Gf256::from_u8(149);
    let c = a * b;
    assert_eq!(c * b.inv(), a);
}

#[test]
fn lagrange_gf256_example() {
    let points = [
        (Gf256::from_u8(1), Gf256::from_u8(6)),
        (Gf256::from_u8(2), Gf256::from_u8(3)),
    ];
    assert_eq!(lagrange_at_zero(&points).to_u8(), 5);
}
