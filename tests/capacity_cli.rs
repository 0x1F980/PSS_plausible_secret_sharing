// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use std::process::Command;

#[test]
fn capacity_cli_matches_library() {
    let tmp = std::env::temp_dir().join("pss_capacity_cli");
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();
    for (name, size) in [("a.bin", 2u64), ("b.bin", 3), ("c.bin", 4), ("d.bin", 5), ("e.bin", 6)] {
        std::fs::write(tmp.join(name), vec![0u8; (size * 1024) as usize]).unwrap();
    }

    let out = Command::new(env!("CARGO_BIN_EXE_pss"))
        .args(["capacity", "--pool", tmp.to_str().unwrap(), "--k", "3", "--n", "5"])
        .output()
        .unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("max_secret_bytes=4096"));
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn cli_help_ok() {
    let out = Command::new(env!("CARGO_BIN_EXE_pss"))
        .arg("help")
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("Plausible Secret Sharing"));
}

#[test]
fn combo_demo_ok() {
    let out = Command::new(env!("CARGO_BIN_EXE_pss"))
        .arg("combo-demo")
        .output()
        .unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    assert!(String::from_utf8_lossy(&out.stdout).contains("combo-demo ok"));
}

#[test]
fn tier_demo_ok() {
    let out = Command::new(env!("CARGO_BIN_EXE_pss"))
        .args(["tier-demo", "--secret", "42"])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("tier root=42"));
}
