// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use std::fs;
use std::path::Path;

mod corpus_helper;

use pss::pss_payload::seed_from_file;
use pss::pss_setup::{setup_from_corpus, SetupConfig};

fn count_files(dir: &Path) -> usize {
    fs::read_dir(dir)
        .unwrap()
        .filter(|e| {
            let e = e.as_ref().unwrap();
            e.file_type().map(|t| t.is_file()).unwrap_or(false)
        })
        .count()
}

fn has_sidecar(dir: &Path) -> bool {
    fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .any(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.ends_with(".key")
                || name.ends_with(".pss")
                || name.contains("manifest")
                || name.ends_with(".salt")
        })
}

#[test]
fn keyless_output_exactly_n_files() {
    let corpus_dir = std::env::temp_dir().join("pss_keyless_corpus");
    let output_dir = std::env::temp_dir().join("pss_keyless_out");
    let _ = fs::remove_dir_all(&corpus_dir);
    let _ = fs::remove_dir_all(&output_dir);
    fs::create_dir_all(&corpus_dir).unwrap();

    let n = 5usize;
    let k = 3usize;
    for i in 0..n {
        fs::write(corpus_dir.join(format!("c{i}.bin")), vec![i as u8 + 1; 256]).unwrap();
    }

    let secret = [42u8; 32];
    let seed = seed_from_file(&secret);

    // Embed shares of setup's derived seed (SHA256(secret)), not raw secret bytes.
    let mut c = 1u8;
    let mut embed_rng = || {
        c = c.wrapping_add(17);
        c
    };
    let synthetic = corpus_helper::build_matching_corpus(&seed, k, n, &mut embed_rng);

    let synth_corpus: Vec<(String, Vec<u8>)> = synthetic
        .iter()
        .map(|s| (s.path.clone(), s.data.clone()))
        .collect();

    // Restart RNG so setup_from_corpus generates the same Shamir shares we embedded.
    let mut c = 1u8;
    let mut setup_rng = || {
        c = c.wrapping_add(17);
        c
    };
    let cfg = SetupConfig { k, n, min_size: 64 };
    let result = setup_from_corpus(&secret, &synth_corpus, &cfg, &mut setup_rng).expect("setup");

    fs::create_dir_all(&output_dir).unwrap();
    for (i, meta) in result.carriers.iter().enumerate() {
        let data = &synthetic[i].data;
        let fname = Path::new(&meta.path)
            .file_name()
            .unwrap()
            .to_string_lossy();
        fs::write(output_dir.join(fname.as_ref()), data).unwrap();
    }

    assert_eq!(count_files(&output_dir), n);
    assert!(!has_sidecar(&output_dir));

    let _ = fs::remove_dir_all(&corpus_dir);
    let _ = fs::remove_dir_all(&output_dir);
}
