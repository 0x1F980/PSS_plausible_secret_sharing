// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

mod corpus_helper;

use pss::pss_path::{combo_decode_seed, decode_path_from_carriers};
use pss::pss_setup::decode_from_carriers;

#[test]
fn combo_needs_path_shares_with_all_carriers() {
    let secret = [11u8; 32];
    let mut c = 5u8;
    let combo = corpus_helper::build_matching_corpus_with_path(
        &secret,
        None,
        2,
        2,
        3,
        &mut || {
            c = c.wrapping_add(7);
            c
        },
    );

    let carriers: Vec<_> = combo
        .carriers
        .iter()
        .map(|c| (c.meta.clone(), c.data.clone()))
        .collect();

    // sum mode still works with k seed shares
    let sum_seed = decode_from_carriers(&carriers, &[1, 2], 32).unwrap();
    assert_eq!(sum_seed[..32], secret);

    // combo with full recipe but only 1 path share cannot reconstruct path
    assert!(decode_path_from_carriers(&carriers, &[1], 5).is_err());

    // wrong recipe (single step only) fails combo decode
    let wrong = pss::pss_path::PathRecipe {
        steps: vec![pss::pss_path::PathStep { left: 1, right: 3 }],
    };
    let wrong_seed = combo_decode_seed(&carriers, &wrong, 32).unwrap();
    assert_ne!(wrong_seed[..32], secret);
}
