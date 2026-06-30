// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use pss::pss_tier::{tier_decode_root, tier_encode_root, TierConfig};

#[test]
fn tier_roundtrip_integration() {
    let cfg = TierConfig { k1: 2, n1: 3, k2: 2, n2: 3 };
    let mut c = 11u8;
    let (branches, leaves) = tier_encode_root(99, &cfg, &mut || {
        c = c.wrapping_add(5);
        c
    })
    .unwrap();
    let root = tier_decode_root(&branches[..2], &leaves, &cfg).unwrap();
    assert_eq!(root, 99);
}
