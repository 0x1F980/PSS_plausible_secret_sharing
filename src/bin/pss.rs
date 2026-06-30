// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use pss::pss_capacity::capacity_report;
use pss::pss_catalog::{assign_indices, catalog_id_path, CarrierMeta};
use pss::pss_extract::extract_transposed;
use pss::pss_path::{combo_decode_seed, decode_path_from_carriers, path_from_bytes};
use pss::pss_payload::{otp_decrypt, OTP_DOMAIN};
use pss::pss_setup::{decode_from_carriers, setup_from_corpus, SetupConfig};
use pss::pss_synthetic::combo_demo_roundtrip;
use pss::pss_tier::{tier_decode_root, tier_encode_root, TierConfig};
use pss::pss_transpose::read_share_byte;
use pss::pss_verify::verify_all_shares;
use pss::pss_shamir::Share;

fn read_bytes(path: &str) -> io::Result<Vec<u8>> {
    if path == "-" {
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf)?;
        Ok(buf)
    } else {
        fs::read(path)
    }
}

fn write_bytes(path: &str, data: &[u8]) -> io::Result<()> {
    if path == "-" {
        io::stdout().write_all(data)?;
        io::stdout().flush()
    } else {
        fs::write(path, data)
    }
}

fn usage() -> &'static str {
    "pss — Plausible Secret Sharing (Shamir ITS + transposed carriers)\n\
\n\
Usage:\n\
  pss setup --corpus DIR --output DIR --k K --n N [--min-size BYTES] --file PATH\n\
  pss decode --pool DIR [--mode sum|combo] [--seed-k K IDX...] [--path-k K IDX...] [--path-len N] [--seed-len N] [--payload FILE] [--payload-len N] INDEX...\n\
  pss verify --pool DIR --k K INDEX...\n\
  pss capacity --pool DIR --k K --n N [--select-top N]\n\
  pss extract --pool DIR [--max-bytes N]\n\
  pss info --pool DIR\n\
  pss combo-demo\n\
  pss tier-setup [--secret BYTE]\n\
  pss tier-decode [--secret BYTE]\n\
  pss tier-demo [--secret BYTE]\n"
}

fn load_pool(dir: &Path) -> io::Result<Vec<(CarrierMeta, Vec<u8>)>> {
    use std::collections::HashMap;

    let mut by_path: HashMap<String, Vec<u8>> = HashMap::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let path = entry.path();
        let data = fs::read(&path)?;
        let path_str = path.to_string_lossy().to_string();
        by_path.insert(path_str, data);
    }

    let metas: Vec<CarrierMeta> = by_path
        .iter()
        .map(|(path_str, data)| CarrierMeta {
            path: path_str.clone(),
            catalog_id: catalog_id_path(path_str),
            size: data.len() as u64,
            index: 0,
        })
        .collect();
    let assigned = assign_indices(metas);
    Ok(assigned
        .into_iter()
        .map(|meta| {
            let data = by_path
                .remove(&meta.path)
                .expect("carrier data missing after index assignment");
            (meta, data)
        })
        .collect())
}

fn cmd_setup(args: &[String]) -> i32 {
    let mut corpus = PathBuf::from(".");
    let mut output = PathBuf::from(".");
    let mut k = 3usize;
    let mut n = 5usize;
    let mut min_size = 0u64;
    let mut file = String::from("-");

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--corpus" => {
                i += 1;
                corpus = PathBuf::from(&args[i]);
            }
            "--output" => {
                i += 1;
                output = PathBuf::from(&args[i]);
            }
            "--k" => {
                i += 1;
                k = args[i].parse().unwrap_or(3);
            }
            "--n" => {
                i += 1;
                n = args[i].parse().unwrap_or(5);
            }
            "--min-size" => {
                i += 1;
                min_size = args[i].parse().unwrap_or(0);
            }
            "--file" => {
                i += 1;
                file = args[i].clone();
            }
            _ => {}
        }
        i += 1;
    }

    let secret = match read_bytes(&file) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("setup read error: {e}");
            return 1;
        }
    };

    let mut corpus_files = Vec::new();
    if let Ok(entries) = fs::read_dir(&corpus) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                let p = entry.path();
                if let Ok(data) = fs::read(&p) {
                    corpus_files.push((p.to_string_lossy().to_string(), data));
                }
            }
        }
    }

    let mut rng_state = 9u8;
    let cfg = SetupConfig { k, n, min_size };
    let result = setup_from_corpus(&secret, &corpus_files, &cfg, &mut || {
        rng_state = rng_state.wrapping_add(23);
        rng_state
    });

    match result {
        Ok(res) => {
            let _ = fs::create_dir_all(&output);
            for meta in &res.carriers {
                let src = corpus.join(Path::new(&meta.path).file_name().unwrap_or_default());
                let dst = output.join(Path::new(&meta.path).file_name().unwrap_or_default());
                if let Ok(data) = fs::read(&src) {
                    let _ = fs::write(&dst, data);
                }
            }
            let count = fs::read_dir(&output)
                .map(|d| d.filter(|e| e.as_ref().map(|x| x.file_type().map(|t| t.is_file()).unwrap_or(false)).unwrap_or(false)).count())
                .unwrap_or(0);
            println!("setup ok: {} carriers written (target n={})", count, n);
            println!("seed sha256: {}", hex_encode(&res.seed));
            0
        }
        Err(e) => {
            eprintln!("setup failed: {e}");
            1
        }
    }
}

fn cmd_capacity(args: &[String]) -> i32 {
    let mut pool = PathBuf::from(".");
    let mut k = 3usize;
    let mut n = 5usize;
    let mut select_top: Option<usize> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--pool" => {
                i += 1;
                pool = PathBuf::from(&args[i]);
            }
            "--k" => {
                i += 1;
                k = args[i].parse().unwrap_or(3);
            }
            "--n" => {
                i += 1;
                n = args[i].parse().unwrap_or(5);
            }
            "--select-top" => {
                i += 1;
                select_top = args[i].parse().ok();
            }
            _ => {}
        }
        i += 1;
    }

    let sizes: Vec<u64> = match fs::read_dir(&pool) {
        Ok(entries) => entries
            .flatten()
            .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
            .filter_map(|e| fs::metadata(e.path()).ok().map(|m| m.len()))
            .collect(),
        Err(e) => {
            eprintln!("capacity error: {e}");
            return 1;
        }
    };

    match capacity_report(&sizes, k, n, select_top) {
        Ok(r) => {
            println!("k={} n={}", r.k, r.n);
            if let Some(top) = select_top {
                println!("select_top={top}");
            }
            println!("total_pool_bytes={}", r.total_pool_bytes);
            println!("max_secret_bytes={} (L_{{n-k+1}}={})", r.max_secret_bytes, r.l_n_minus_k_plus_1);
            println!("utilization_pct={:.4}", r.utilization_pct);
            0
        }
        Err(e) => {
            eprintln!("capacity error: {e}");
            1
        }
    }
}

fn cmd_extract(args: &[String]) -> i32 {
    let mut pool = PathBuf::from(".");
    let mut max_bytes = 32usize;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--pool" => {
                i += 1;
                pool = PathBuf::from(&args[i]);
            }
            "--max-bytes" => {
                i += 1;
                max_bytes = args[i].parse().unwrap_or(32);
            }
            _ => {}
        }
        i += 1;
    }

    let carriers = match load_pool(&pool) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("extract error: {e}");
            return 1;
        }
    };
    let pool_refs: Vec<(&[u8], String)> = carriers
        .iter()
        .map(|(m, d)| (d.as_slice(), m.path.clone()))
        .collect();
    match extract_transposed(&pool_refs, max_bytes) {
        Ok(data) => {
            let _ = write_bytes("-", &data);
            0
        }
        Err(e) => {
            eprintln!("extract error: {e}");
            1
        }
    }
}

fn cmd_info(args: &[String]) -> i32 {
    let mut pool = PathBuf::from(".");
    if args.len() >= 2 && args[0] == "--pool" {
        pool = PathBuf::from(&args[1]);
    }
    match load_pool(&pool) {
        Ok(carriers) => {
            for (m, _) in &carriers {
                println!("path={} index={} size={} catalog_id={}", m.path, m.index, m.size, hex_encode(&m.catalog_id));
            }
            0
        }
        Err(e) => {
            eprintln!("info error: {e}");
            1
        }
    }
}

fn cmd_decode(args: &[String]) -> i32 {
    let mut pool = PathBuf::from(".");
    let mut mode = String::from("sum");
    let mut seed_indices = Vec::new();
    let mut path_indices = Vec::new();
    let mut path_len = 0usize;
    let mut seed_len = 32usize;
    let mut payload_file: Option<String> = None;
    let mut payload_len = 0usize;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--pool" => {
                i += 1;
                pool = PathBuf::from(&args[i]);
            }
            "--mode" => {
                i += 1;
                mode = args[i].clone();
            }
            "--seed-k" => {
                i += 1;
                while i < args.len() && !args[i].starts_with('-') {
                    if let Ok(v) = args[i].parse::<u8>() {
                        seed_indices.push(v);
                    }
                    i += 1;
                }
                continue;
            }
            "--path-k" => {
                i += 1;
                while i < args.len() && !args[i].starts_with('-') {
                    if let Ok(v) = args[i].parse::<u8>() {
                        path_indices.push(v);
                    }
                    i += 1;
                }
                continue;
            }
            "--path-len" => {
                i += 1;
                path_len = args[i].parse().unwrap_or(0);
            }
            "--seed-len" => {
                i += 1;
                seed_len = args[i].parse().unwrap_or(32);
            }
            "--payload" => {
                i += 1;
                payload_file = Some(args[i].clone());
            }
            "--payload-len" => {
                i += 1;
                payload_len = args[i].parse().unwrap_or(0);
            }
            "--k" => {
                i += 1;
                while i < args.len() && !args[i].starts_with('-') {
                    if let Ok(v) = args[i].parse::<u8>() {
                        seed_indices.push(v);
                    }
                    i += 1;
                }
                continue;
            }
            s if !s.starts_with('-') => {
                if let Ok(v) = s.parse::<u8>() {
                    if mode == "combo" && path_indices.is_empty() && seed_indices.is_empty() {
                        path_indices.push(v);
                    } else {
                        seed_indices.push(v);
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }

    let carriers = match load_pool(&pool) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("decode error: {e}");
            return 1;
        }
    };

    let seed = if mode == "combo" {
        if path_indices.len() < 2 {
            eprintln!("combo mode requires --path-k with at least 2 indices");
            return 1;
        }
        let plen = if path_len > 0 {
            path_len
        } else {
            5usize
        };
        let path_bytes = match decode_path_from_carriers(&carriers, &path_indices, plen) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("path decode error: {e}");
                return 1;
            }
        };
        let recipe = match path_from_bytes(&path_bytes) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("path recipe error: {e}");
                return 1;
            }
        };
        let combo_seed = match combo_decode_seed(&carriers, &recipe, seed_len) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("combo decode error: {e}");
                return 1;
            }
        };
        if seed_indices.len() >= 2 {
            let sum_seed = match decode_from_carriers(&carriers, &seed_indices, seed_len) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("combo verify (sum decode) error: {e}");
                    return 1;
                }
            };
            if combo_seed[..seed_len] != sum_seed[..seed_len] {
                eprintln!("combo verify: seed mismatch (path recipe vs sum decode)");
                return 1;
            }
        }
        combo_seed
    } else {
        if seed_indices.len() < 2 {
            eprintln!("sum mode requires at least 2 seed indices");
            return 1;
        }
        match decode_from_carriers(&carriers, &seed_indices, seed_len) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("decode error: {e}");
                return 1;
            }
        }
    };

    if let Some(pf) = payload_file {
        let cipher = match read_bytes(&pf) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("payload read error: {e}");
                return 1;
            }
        };
        if payload_len > cipher.len() {
            eprintln!(
                "payload-len {payload_len} exceeds ciphertext length {}",
                cipher.len()
            );
            return 1;
        }
        let mut plain = otp_decrypt(&cipher, &seed, OTP_DOMAIN);
        if payload_len > 0 {
            plain.truncate(payload_len);
        }
        if pf == "-" {
            if write_bytes("-", &plain).is_err() {
                return 1;
            }
        } else {
            println!("payload={}", String::from_utf8_lossy(&plain));
        }
    } else {
        println!("seed={}", hex_encode(&seed));
    }
    0
}

fn cmd_combo_demo(_args: &[String]) -> i32 {
    let mut c = 9u8;
    match combo_demo_roundtrip(&mut || {
        c = c.wrapping_add(23);
        c
    }) {
        Ok(()) => {
            println!("combo-demo ok (ITS path + sum chain roundtrip)");
            0
        }
        Err(e) => {
            eprintln!("combo-demo failed: {e}");
            1
        }
    }
}

fn cmd_verify(args: &[String]) -> i32 {
    let mut pool = PathBuf::from(".");
    let mut k_indices = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--pool" => {
                i += 1;
                pool = PathBuf::from(&args[i]);
            }
            s if s.parse::<u8>().is_ok() => k_indices.push(s.parse().unwrap()),
            _ => {}
        }
        i += 1;
    }

    let carriers = match load_pool(&pool) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("verify error: {e}");
            return 1;
        }
    };

    if k_indices.len() < 3 {
        eprintln!("verify needs at least 3 share indices for k+1 check");
        return 1;
    }

    let mut shares: Vec<Share> = Vec::new();
    for &idx in &k_indices {
        let (meta, data) = match carriers.iter().find(|(m, _)| m.index == idx) {
            Some(x) => x,
            None => {
                eprintln!("missing index {idx}");
                return 1;
            }
        };
        let val = match read_share_byte(data, &meta.catalog_id, meta.index, 0) {
            Ok(b) => pss::Gf256::from_u8(b),
            Err(e) => {
                eprintln!("read error: {e}");
                return 1;
            }
        };
        shares.push((idx, val));
    }

    match verify_all_shares(&shares) {
        Ok(()) => {
            println!("verify ok");
            0
        }
        Err(e) => {
            eprintln!("verify failed: {e}");
            1
        }
    }
}

fn cmd_tier_demo(args: &[String]) -> i32 {
    let secret = args
        .iter()
        .position(|a| a == "--secret")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(42u8);

    let cfg = TierConfig { k1: 2, n1: 3, k2: 2, n2: 3 };
    let mut c = 5u8;
    match tier_encode_root(secret, &cfg, &mut || {
        c = c.wrapping_add(7);
        c
    }) {
        Ok((branches, leaves)) => match tier_decode_root(&branches[..2], &leaves, &cfg) {
            Ok(root) => {
                println!("tier root={root}");
                0
            }
            Err(e) => {
                eprintln!("tier decode: {e}");
                1
            }
        },
        Err(e) => {
            eprintln!("tier encode: {e}");
            1
        }
    }
}

fn hex_encode(data: &[u8]) -> String {
    data.iter().map(|b| format!("{b:02x}")).collect()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        print!("{}", usage());
        std::process::exit(0);
    }
    let code = match args[0].as_str() {
        "setup" => cmd_setup(&args[1..]),
        "decode" => cmd_decode(&args[1..]),
        "combo-demo" => cmd_combo_demo(&args[1..]),
        "verify" => cmd_verify(&args[1..]),
        "capacity" => cmd_capacity(&args[1..]),
        "extract" => cmd_extract(&args[1..]),
        "info" => cmd_info(&args[1..]),
        "tier-setup" | "tier-decode" | "tier-demo" => cmd_tier_demo(&args[1..]),
        "help" | "--help" | "-h" => {
            print!("{}", usage());
            0
        }
        _ => {
            eprintln!("unknown command: {}", args[0]);
            print!("{}", usage());
            1
        }
    };
    std::process::exit(code);
}
