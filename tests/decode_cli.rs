// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

mod corpus_helper;

use std::path::Path;
use std::process::Command;

use pss::pss_payload::{otp_encrypt, OTP_DOMAIN};
use pss::pss_path::path_to_bytes;
use pss::pss_setup::decode_from_carriers;

fn write_pool(corpus: &[corpus_helper::SyntheticCarrier]) {
    for car in corpus {
        if let Some(parent) = Path::new(&car.path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::write(&car.path, &car.data).unwrap();
    }
}

#[test]
fn decode_otp_roundtrip_cli() {
    let tmp = std::env::temp_dir().join("pss_decode_otp_cli");
    let pool = tmp.join("carriers");
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&pool).unwrap();

    let secret = [42u8; 32];
    let mut c = 3u8;
    let corpus = corpus_helper::build_matching_corpus_in_dir(&pool, &secret, 3, 5, &mut || {
        c = c.wrapping_add(19);
        c
    });
    write_pool(&corpus);

    let carriers: Vec<_> = corpus
        .iter()
        .map(|c| (c.meta.clone(), c.data.clone()))
        .collect();
    let seed = decode_from_carriers(&carriers, &[1, 2, 3], 32).unwrap();
    let plain = b"hello otp payload";
    let cipher = otp_encrypt(plain, &seed, OTP_DOMAIN);
    let cipher_path = tmp.join("cipher.bin");
    std::fs::write(&cipher_path, &cipher).unwrap();

    let out = Command::new(env!("CARGO_BIN_EXE_pss"))
        .args([
            "decode",
            "--pool",
            pool.to_str().unwrap(),
            "1",
            "2",
            "3",
            "--payload",
            cipher_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.contains("payload=hello otp payload"),
        "stdout={text:?} stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn decode_otp_payload_len_cli() {
    let tmp = std::env::temp_dir().join("pss_decode_otp_len_cli");
    let pool = tmp.join("carriers");
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&pool).unwrap();

    let secret = [7u8; 32];
    let mut c = 5u8;
    let corpus = corpus_helper::build_matching_corpus_in_dir(&pool, &secret, 2, 3, &mut || {
        c = c.wrapping_add(11);
        c
    });
    write_pool(&corpus);

    let carriers: Vec<_> = corpus
        .iter()
        .map(|c| (c.meta.clone(), c.data.clone()))
        .collect();
    let seed = decode_from_carriers(&carriers, &[1, 2], 32).unwrap();
    let plain = b"0123456789abcdef";
    let cipher = otp_encrypt(plain, &seed, OTP_DOMAIN);
    let cipher_path = tmp.join("cipher.bin");
    std::fs::write(&cipher_path, &cipher).unwrap();

    let out = Command::new(env!("CARGO_BIN_EXE_pss"))
        .args([
            "decode",
            "--pool",
            pool.to_str().unwrap(),
            "1",
            "2",
            "--payload",
            cipher_path.to_str().unwrap(),
            "--payload-len",
            "8",
        ])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("payload=01234567"));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn decode_combo_cli() {
    let tmp = std::env::temp_dir().join("pss_decode_combo_cli");
    let pool = tmp.join("carriers");
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&pool).unwrap();

    let secret = [11u8; 32];
    let mut c = 8u8;
    let combo = corpus_helper::build_matching_corpus_with_path_in_dir(
        &pool,
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
    write_pool(&combo.carriers);

    let path_len = path_to_bytes(&combo.recipe).len();
    let path_len_str = path_len.to_string();

    let out = Command::new(env!("CARGO_BIN_EXE_pss"))
        .args([
            "decode",
            "--pool",
            pool.to_str().unwrap(),
            "--mode",
            "combo",
            "--path-k",
            "1",
            "2",
            "--path-len",
            &path_len_str,
            "--seed-k",
            "1",
            "2",
        ])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.starts_with("seed="));

    let _ = std::fs::remove_dir_all(&tmp);
}
