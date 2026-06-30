// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use pss::pss_catalog::catalog_id_path;
use pss::pss_transpose::{transpose_offset, DOMAIN_PSS_V1};

#[test]
fn transpose_offset_stable() {
    let id = catalog_id_path("carrier_a.bin");
    let o1 = transpose_offset(DOMAIN_PSS_V1, &id, 2, 7, 4096).unwrap();
    let o2 = transpose_offset(DOMAIN_PSS_V1, &id, 2, 7, 4096).unwrap();
    assert_eq!(o1, o2);
    assert!(o1 < 4095);
}
