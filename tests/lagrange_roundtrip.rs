// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use pss::pss_field_gf256::Gf256;
use pss::pss_lagrange::lagrange_at_zero;
use pss::pss_shamir::{reconstruct_byte, split_byte};

#[test]
fn lagrange_reconstructs_secret_byte() {
    let indices = [1u8, 2, 3];
    let shares = split_byte(0xAB, 2, &indices, &[0x11]).unwrap();
    let recovered = reconstruct_byte(&shares[..2]).unwrap();
    assert_eq!(recovered.to_u8(), 0xAB);
}

#[test]
fn lagrange_three_point_example() {
    let points = [
        (Gf256::from_u8(1), Gf256::from_u8(6)),
        (Gf256::from_u8(2), Gf256::from_u8(3)),
        (Gf256::from_u8(3), Gf256::from_u8(7)),
    ];
    assert_eq!(lagrange_at_zero(&points[..2]).to_u8(), 5);
}
