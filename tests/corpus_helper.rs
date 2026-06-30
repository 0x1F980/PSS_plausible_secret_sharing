// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

//! Build synthetic carriers whose transposed bytes match Shamir shares.

use std::collections::BTreeSet;
use std::collections::HashMap;

use pss::pss_catalog::{assign_indices, catalog_id_path, CarrierMeta};
use pss::pss_path::{combo_decode_seed, decode_path_from_carriers, path_from_bytes, PathRecipe, PathStep, PATH_CHAIN};
use pss::pss_setup::decode_from_carriers;
use pss::pss_shamir::split_secret;
use pss::pss_transpose::{transpose_offset, DOMAIN_PSS_V1, DOMAIN_PSS_V1_PATH};

#[derive(Clone, Debug)]
pub struct SyntheticCarrier {
    pub path: String,
    pub data: Vec<u8>,
    pub meta: CarrierMeta,
}

#[derive(Clone, Debug)]
pub struct ComboCorpus {
    pub carriers: Vec<SyntheticCarrier>,
    pub recipe: PathRecipe,
}

fn embed_domain_shares(
    files: &mut [Vec<u8>],
    metas: &[CarrierMeta],
    per_share: &[Vec<(usize, u8)>],
    domain: &[u8],
) -> bool {
    for (share_idx, meta) in metas.iter().enumerate() {
        let mut used = BTreeSet::new();
        for &(pos, byte) in &per_share[share_idx] {
            let Ok(off) = transpose_offset(
                domain,
                &meta.catalog_id,
                meta.index,
                pos as u64,
                files[share_idx].len() as u64,
            ) else {
                return false;
            };
            if !used.insert(off) {
                return false;
            }
            files[share_idx][off as usize] = byte;
        }
    }
    true
}

pub fn default_combo_recipe() -> PathRecipe {
    PathRecipe {
        steps: vec![
            PathStep { left: 1, right: 2 },
            PathStep {
                left: PATH_CHAIN,
                right: 1,
            },
        ],
    }
}

pub fn build_matching_corpus(
    secret: &[u8],
    k: usize,
    n: usize,
    rng: &mut impl FnMut() -> u8,
) -> Vec<SyntheticCarrier> {
    let paths: Vec<String> = (0..n).map(|i| format!("carrier_{i}.bin")).collect();
    build_matching_corpus_inner(secret, None, k, k, n, false, paths, rng).carriers
}

fn carrier_paths_on_disk(dir: &std::path::Path, n: usize) -> Vec<String> {
    std::fs::create_dir_all(dir).ok();
    for i in 0..n {
        std::fs::write(dir.join(format!("carrier_{i}.bin")), []).expect("touch carrier");
    }
    let mut paths: Vec<String> = std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
        .map(|e| e.path().to_string_lossy().into_owned())
        .collect();
    paths.sort();
    assert_eq!(paths.len(), n, "expected {n} carrier files in {}", dir.display());
    paths
}

/// Embed shares at offsets for exact on-disk paths (CLI tests: catalog_id = path hash).
pub fn build_matching_corpus_in_dir(
    dir: &std::path::Path,
    secret: &[u8],
    k: usize,
    n: usize,
    rng: &mut impl FnMut() -> u8,
) -> Vec<SyntheticCarrier> {
    let paths = carrier_paths_on_disk(dir, n);
    build_matching_corpus_inner(secret, None, k, k, n, false, paths, rng).carriers
}

pub fn build_matching_corpus_with_path(
    secret: &[u8],
    path_recipe: Option<PathRecipe>,
    k_seed: usize,
    k_path: usize,
    n: usize,
    rng: &mut impl FnMut() -> u8,
) -> ComboCorpus {
    let paths: Vec<String> = (0..n).map(|i| format!("carrier_{i}.bin")).collect();
    build_matching_corpus_inner(secret, path_recipe, k_seed, k_path, n, true, paths, rng)
}

pub fn build_matching_corpus_with_path_in_dir(
    dir: &std::path::Path,
    secret: &[u8],
    path_recipe: Option<PathRecipe>,
    k_seed: usize,
    k_path: usize,
    n: usize,
    rng: &mut impl FnMut() -> u8,
) -> ComboCorpus {
    let paths = carrier_paths_on_disk(dir, n);
    build_matching_corpus_inner(secret, path_recipe, k_seed, k_path, n, true, paths, rng)
}

fn build_matching_corpus_inner(
    secret: &[u8],
    path_recipe: Option<PathRecipe>,
    k_seed: usize,
    k_path: usize,
    n: usize,
    require_path: bool,
    paths: Vec<String>,
    rng: &mut impl FnMut() -> u8,
) -> ComboCorpus {
    let recipe = path_recipe.unwrap_or_else(default_combo_recipe);
    let path_bytes = if require_path {
        Some(pss::pss_path::path_to_bytes(&recipe))
    } else {
        None
    };
    let indices: Vec<u8> = (1..=n as u8).collect();
    let per_seed = split_secret(secret, k_seed, &indices, rng).expect("seed split");
    let per_path = if let Some(ref pb) = path_bytes {
        let mut path_rng = || rng();
        Some(split_secret(pb, k_path, &indices, &mut path_rng).expect("path split"))
    } else {
        None
    };
    let paths: Vec<String> = if paths.len() == n {
        paths
    } else {
        (0..n).map(|i| format!("carrier_{i}.bin")).collect()
    };

    let mut len = secret.len().max(path_bytes.as_ref().map(|p| p.len()).unwrap_or(0)).max(512);
    for _attempt in 0..128 {
        let mut files: Vec<Vec<u8>> = paths
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let mut v = vec![0u8; len];
                let pb = p.as_bytes();
                for (j, slot) in v.iter_mut().enumerate() {
                    *slot = pb.get(j).copied().unwrap_or((i as u8).wrapping_add(j as u8));
                }
                v
            })
            .collect();

        let mut metas: Vec<CarrierMeta> = paths
            .iter()
            .zip(files.iter())
            .map(|(path, data)| CarrierMeta {
                path: path.clone(),
                catalog_id: catalog_id_path(path),
                size: data.len() as u64,
                index: 0,
            })
            .collect();
        metas = assign_indices(metas);

        let mut files_by_path: HashMap<String, Vec<u8>> =
            paths.iter().cloned().zip(files).collect();
        files = metas
            .iter()
            .map(|m| files_by_path.remove(&m.path).expect("carrier file"))
            .collect();

        if !embed_domain_shares(&mut files, &metas, &per_seed, DOMAIN_PSS_V1) {
            len = len.saturating_add(256);
            continue;
        }
        if let Some(ref pp) = per_path {
            if !embed_domain_shares(&mut files, &metas, pp, DOMAIN_PSS_V1_PATH) {
                len = len.saturating_add(256);
                continue;
            }
        }

        let carriers: Vec<(CarrierMeta, Vec<u8>)> = metas
            .iter()
            .zip(files.iter())
            .map(|(m, d)| (m.clone(), d.clone()))
            .collect();

        let k_seed_indices: Vec<u8> = (1..=k_seed as u8).collect();
        let k_path_indices: Vec<u8> = (1..=k_path as u8).collect();
        let seed_ok = decode_from_carriers(&carriers, &k_seed_indices, secret.len())
            .map(|s| s[..secret.len()] == *secret)
            .unwrap_or(false);

        let path_ok = if require_path {
            let pb = path_bytes.as_ref().unwrap();
            decode_path_from_carriers(&carriers, &k_path_indices, pb.len())
                .ok()
                .and_then(|p| path_from_bytes(&p).ok())
                .map(|r| r == recipe)
                .unwrap_or(false)
        } else {
            true
        };

        let combo_ok = if require_path {
            combo_decode_seed(&carriers, &recipe, secret.len())
                .map(|s| s[..secret.len()] == *secret)
                .unwrap_or(false)
        } else {
            true
        };

        if seed_ok && path_ok && combo_ok {
            let carriers_out = metas
                .into_iter()
                .zip(files)
                .map(|(meta, data)| SyntheticCarrier {
                    path: meta.path.clone(),
                    data,
                    meta,
                })
                .collect();
            return ComboCorpus {
                carriers: carriers_out,
                recipe,
            };
        }

        len = len.saturating_add(256);
    }

    panic!("failed to build matching corpus with path after retries");
}

#[cfg(test)]
mod tests {
    use super::*;
    use pss::pss_transpose::read_share_byte;

    #[test]
    fn synthetic_roundtrip() {
        let secret = [42u8; 32];
        let mut c = 1u8;
        let corpus = build_matching_corpus(&secret, 3, 5, &mut || {
            c = c.wrapping_add(19);
            c
        });

        let carriers: Vec<(CarrierMeta, Vec<u8>)> = corpus
            .iter()
            .map(|c| (c.meta.clone(), c.data.clone()))
            .collect();

        let seed = decode_from_carriers(&carriers, &[1, 2, 3], 32).unwrap();
        assert_eq!(seed, secret);
    }

    #[test]
    fn read_share_matches_embedded() {
        let secret = [7u8; 16];
        let mut c = 2u8;
        let corpus = build_matching_corpus(&secret, 2, 3, &mut || {
            c = c.wrapping_add(11);
            c
        });
        let c1 = corpus
            .iter()
            .find(|c| c.meta.index == 1)
            .expect("carrier with index 1");
        let off = transpose_offset(
            DOMAIN_PSS_V1,
            &c1.meta.catalog_id,
            c1.meta.index,
            0,
            c1.data.len() as u64,
        )
        .unwrap();
        let b0 = read_share_byte(&c1.data, &c1.meta.catalog_id, c1.meta.index, 0).unwrap();
        assert_eq!(b0, c1.data[off as usize]);
    }
}
