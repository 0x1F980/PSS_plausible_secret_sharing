// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use pss::pss_shamir::{reconstruct_byte, split_byte};

#[test]
fn shamir_its_less_than_k_uniform_posterior() {
    let indices = [1u8, 2, 3, 4, 5];
    let mut counts = [0u32; 256];

    for secret in 0u8..=255 {
        let shares = split_byte(secret, 3, &indices, &[17, 33]).unwrap();
        let subset = [shares[0], shares[1]];
        let recovered = reconstruct_byte(&subset).unwrap().to_u8();
        counts[recovered as usize] += 1;
    }

    let expected = 256u32 / 256;
    for &c in &counts {
        assert!((c as i32 - expected as i32).abs() <= 1);
    }
}
