// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

mod corpus_helper;

use std::process::Command;

#[test]
fn cli_info_on_synthetic_pool() {
    let tmp = std::env::temp_dir().join("pss_cli_pool");
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();

    let secret = [9u8; 32];
    let mut c_state = 4u8;
    for c in corpus_helper::build_matching_corpus(&secret, 3, 5, &mut || {
        c_state = c_state.wrapping_add(13);
        c_state
    }) {
        std::fs::write(tmp.join(&c.path), &c.data).unwrap();
    }

    let out = Command::new(env!("CARGO_BIN_EXE_pss"))
        .args(["info", "--pool", tmp.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("index="));
    let _ = std::fs::remove_dir_all(&tmp);
}
