// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use pss::pss_extract::extract_transposed;

#[test]
fn extract_same_pool_same_output() {
    let a = (b"alpha file content".as_slice(), "a.bin".to_string());
    let b = (b"beta file content here".as_slice(), "b.bin".to_string());
    let pool1 = [a.clone(), b.clone()];
    let e1 = extract_transposed(&pool1, 8).unwrap();
    let e2 = extract_transposed(&[a, b], 8).unwrap();
    assert_eq!(e1, e2);
}
