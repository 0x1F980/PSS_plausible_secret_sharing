// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

//! In-memory synthetic carriers for demos and integration-style tests.

use alloc::collections::BTreeSet;

use crate::pss_catalog::{assign_indices, catalog_id_path, CarrierMeta};
use crate::pss_path::{
    combo_decode_seed, decode_path_from_carriers, path_from_bytes, path_to_bytes, PathRecipe,
    PathStep, PATH_CHAIN,
};
use crate::pss_setup::decode_from_carriers;
use crate::pss_shamir::split_secret;
use crate::pss_transpose::{transpose_offset, DOMAIN_PSS_V1, DOMAIN_PSS_V1_PATH};
use crate::PssResult;

#[derive(Clone, Debug)]
pub struct SyntheticPool {
    pub carriers: alloc::vec::Vec<(CarrierMeta, alloc::vec::Vec<u8>)>,
    pub recipe: PathRecipe,
}

fn embed_domain(
    files: &mut [alloc::vec::Vec<u8>],
    metas: &[CarrierMeta],
    per_share: &[alloc::vec::Vec<(usize, u8)>],
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

pub fn default_recipe() -> PathRecipe {
    PathRecipe {
        steps: alloc::vec![
            PathStep { left: 1, right: 2 },
            PathStep {
                left: PATH_CHAIN,
                right: 1,
            },
        ],
    }
}

pub fn build_combo_pool(
    secret: &[u8],
    k_seed: usize,
    k_path: usize,
    n: usize,
    rng: &mut impl FnMut() -> u8,
) -> PssResult<SyntheticPool> {
    let recipe = default_recipe();
    let path_bytes = path_to_bytes(&recipe);
    let indices: alloc::vec::Vec<u8> = (1..=n as u8).collect();
    let per_seed = split_secret(secret, k_seed, &indices, rng)?;
    let mut path_rng = || rng();
    let per_path = split_secret(&path_bytes, k_path, &indices, &mut path_rng)?;
    let paths: alloc::vec::Vec<alloc::string::String> =
        (0..n).map(|i| alloc::format!("carrier_{i}.bin")).collect();

    let mut len = secret.len().max(path_bytes.len()).max(512);
    for _ in 0..128 {
        let mut files: alloc::vec::Vec<alloc::vec::Vec<u8>> = paths
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let mut v = alloc::vec![0u8; len];
                let pb = p.as_bytes();
                for (j, slot) in v.iter_mut().enumerate() {
                    *slot = pb.get(j).copied().unwrap_or((i as u8).wrapping_add(j as u8));
                }
                v
            })
            .collect();

        let mut metas: alloc::vec::Vec<CarrierMeta> = paths
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

        if !embed_domain(&mut files, &metas, &per_seed, DOMAIN_PSS_V1) {
            len = len.saturating_add(256);
            continue;
        }
        if !embed_domain(&mut files, &metas, &per_path, DOMAIN_PSS_V1_PATH) {
            len = len.saturating_add(256);
            continue;
        }

        let carriers: alloc::vec::Vec<_> = metas
            .iter()
            .zip(files.iter())
            .map(|(m, d)| (m.clone(), d.clone()))
            .collect();

        let k_seed_i: alloc::vec::Vec<u8> = (1..=k_seed as u8).collect();
        let k_path_i: alloc::vec::Vec<u8> = (1..=k_path as u8).collect();
        let seed_ok = decode_from_carriers(&carriers, &k_seed_i, secret.len())
            .map(|s| s[..secret.len()] == *secret)
            .unwrap_or(false);
        let path_ok = decode_path_from_carriers(&carriers, &k_path_i, path_bytes.len())
            .ok()
            .and_then(|p| path_from_bytes(&p).ok())
            .map(|r| r == recipe)
            .unwrap_or(false);
        let combo_ok = combo_decode_seed(&carriers, &recipe, secret.len())
            .map(|s| s[..secret.len()] == *secret)
            .unwrap_or(false);

        if seed_ok && path_ok && combo_ok {
            return Ok(SyntheticPool { carriers, recipe });
        }
        len = len.saturating_add(256);
    }
    Err(crate::PssError::CorpusSearchFailed)
}

pub fn combo_demo_roundtrip(rng: &mut impl FnMut() -> u8) -> PssResult<()> {
    let secret = [42u8; 32];
    build_combo_pool(&secret, 2, 2, 3, rng)?;
    Ok(())
}
