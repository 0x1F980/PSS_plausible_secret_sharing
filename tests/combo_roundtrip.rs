// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

mod corpus_helper;

use pss::pss_path::{combo_decode_seed, decode_path_from_carriers, path_from_bytes};
use pss::pss_setup::decode_from_carriers;

#[test]
fn sum_and_combo_same_seed() {
    let secret = [99u8; 32];
    let mut c = 3u8;
    let combo = corpus_helper::build_matching_corpus_with_path(
        &secret,
        None,
        2,
        2,
        3,
        &mut || {
            c = c.wrapping_add(13);
            c
        },
    );

    let carriers: Vec<_> = combo
        .carriers
        .iter()
        .map(|c| (c.meta.clone(), c.data.clone()))
        .collect();

    let sum_seed = decode_from_carriers(&carriers, &[1, 2], 32).unwrap();
    assert_eq!(sum_seed[..32], secret);

    let path_bytes = decode_path_from_carriers(&carriers, &[1, 2], 5).unwrap();
    let recipe = path_from_bytes(&path_bytes).unwrap();
    assert_eq!(recipe, combo.recipe);

    let combo_seed = combo_decode_seed(&carriers, &combo.recipe, 32).unwrap();
    assert_eq!(combo_seed[..32], secret);
}
