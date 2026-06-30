// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use pss::pss_capacity::capacity_report;

#[test]
fn capacity_23456_gb_units() {
    let sizes = [2u64, 3, 4, 5, 6];
    let r = capacity_report(&sizes, 3, 5, None).unwrap();
    assert_eq!(r.max_secret_bytes, 4);
    assert_eq!(r.l_n_minus_k_plus_1, 4);
    assert_eq!(r.total_pool_bytes, 20);
    assert!((r.utilization_pct - 20.0).abs() < 0.01);
}
