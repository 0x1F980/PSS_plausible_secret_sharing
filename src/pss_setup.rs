// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use crate::pss_catalog::{assign_indices, catalog_id_path, CarrierMeta};
use crate::pss_error::{PssError, PssResult};
use crate::pss_payload::{otp_encrypt, seed_from_file, OTP_DOMAIN};
use crate::pss_shamir::{split_secret, ShareBundle};
use crate::pss_transpose::read_share_byte;

#[derive(Clone, Debug)]
pub struct SetupConfig {
    pub k: usize,
    pub n: usize,
    pub min_size: u64,
}

#[derive(Clone, Debug)]
pub struct SetupResult {
    pub carriers: alloc::vec::Vec<CarrierMeta>,
    pub seed: [u8; 32],
}

/// Search corpus for n untouched files whose transposed bytes match Shamir shares.
/// For testing/small secrets: uses in-memory corpus entries (path, data).
pub fn setup_from_corpus(
    secret_file: &[u8],
    corpus: &[(alloc::string::String, alloc::vec::Vec<u8>)],
    cfg: &SetupConfig,
    rng: &mut impl FnMut() -> u8,
) -> PssResult<SetupResult> {
    if cfg.k == 0 || cfg.n < cfg.k {
        return Err(PssError::InvalidN);
    }

    let seed = seed_from_file(secret_file);
    let indices: alloc::vec::Vec<u8> = (1..=cfg.n as u8).collect();
    let per_share = split_secret(&seed, cfg.k, &indices, rng)?;

    let candidates: alloc::vec::Vec<_> = corpus
        .iter()
        .filter(|(_, d)| d.len() as u64 >= cfg.min_size)
        .collect();

    if candidates.len() < cfg.n {
        return Err(PssError::CorpusSearchFailed);
    }

    let mut selected = alloc::vec::Vec::new();
    'search: for combo in combinations(candidates.len(), cfg.n) {
        let mut metas = alloc::vec::Vec::new();
        let mut ok = true;
        for &ci in &combo {
            let (path, data) = &candidates[ci];
            metas.push(CarrierMeta {
                path: path.clone(),
                catalog_id: catalog_id_path(path),
                size: data.len() as u64,
                index: 0,
            });
        }
        metas = assign_indices(metas);

        for (share_idx, meta) in metas.iter().enumerate() {
            let data = &candidates[combo[share_idx]].1;
            for &(pos, expected) in &per_share[share_idx] {
                let got = read_share_byte(data, &meta.catalog_id, meta.index, pos as u64)?;
                if got != expected {
                    ok = false;
                    break;
                }
            }
            if !ok {
                break;
            }
        }
        if ok {
            selected = combo.iter().map(|&i| candidates[i].0.clone()).collect();
            break 'search;
        }
    }

    if selected.is_empty() {
        return Err(PssError::CorpusSearchFailed);
    }

    let mut carriers: alloc::vec::Vec<CarrierMeta> = selected
        .iter()
        .map(|path| {
            let data = corpus.iter().find(|(p, _)| p == path).unwrap().1.as_slice();
            CarrierMeta {
                path: path.clone(),
                catalog_id: catalog_id_path(path),
                size: data.len() as u64,
                index: 0,
            }
        })
        .collect();
    carriers = assign_indices(carriers);

    Ok(SetupResult { carriers, seed })
}

pub fn decode_from_carriers(
    carriers: &[(CarrierMeta, alloc::vec::Vec<u8>)],
    k_indices: &[u8],
    seed_len: usize,
) -> PssResult<[u8; 32]> {
    if k_indices.len() < 2 {
        return Err(PssError::InsufficientShares);
    }

    let mut bundles = alloc::vec::Vec::new();
    for &idx in k_indices {
        let (meta, data) = carriers
            .iter()
            .find(|(m, _)| m.index == idx)
            .ok_or(PssError::InsufficientShares)?;
        let mut bytes = alloc::vec::Vec::new();
        for b in 0..seed_len {
            let val = read_share_byte(data, &meta.catalog_id, meta.index, b as u64)?;
            bytes.push((b, val));
        }
        bundles.push(ShareBundle {
            index: meta.index,
            bytes,
        });
    }

    let recovered = crate::pss_shamir::reconstruct_secret(&bundles, seed_len)?;
    let mut seed = [0u8; 32];
    let n = recovered.len().min(32);
    seed[..n].copy_from_slice(&recovered[..n]);
    Ok(seed)
}

pub fn payload_ciphertext(secret_file: &[u8], seed: &[u8; 32]) -> alloc::vec::Vec<u8> {
    otp_encrypt(secret_file, seed, OTP_DOMAIN)
}

fn combinations(n: usize, k: usize) -> alloc::vec::Vec<alloc::vec::Vec<usize>> {
    let mut out = alloc::vec::Vec::new();
    let mut combo = alloc::vec::Vec::new();
    fn rec(n: usize, k: usize, start: usize, combo: &mut alloc::vec::Vec<usize>, out: &mut alloc::vec::Vec<alloc::vec::Vec<usize>>) {
        if combo.len() == k {
            out.push(combo.clone());
            return;
        }
        for i in start..n {
            combo.push(i);
            rec(n, k, i + 1, combo, out);
            combo.pop();
        }
    }
    rec(n, k, 0, &mut combo, &mut out);
    out
}
