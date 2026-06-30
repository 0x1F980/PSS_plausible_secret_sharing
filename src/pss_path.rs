// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use crate::pss_catalog::CarrierMeta;
use crate::pss_error::{PssError, PssResult};
use crate::pss_field_gf256::Gf256;
use crate::pss_lagrange::lagrange_at_zero;
use crate::pss_shamir::{reconstruct_secret, split_secret, ShareBundle};
use crate::pss_transpose::{read_path_share_byte, read_share_byte};

pub use crate::pss_transpose::DOMAIN_PSS_V1_PATH;

/// Chain index 0 = use accumulated value at x=0 (genvalg / ice-cream repeat).
pub const PATH_CHAIN: u8 = 0;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathStep {
    pub left: u8,
    pub right: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathRecipe {
    pub steps: alloc::vec::Vec<PathStep>,
}

pub fn path_to_bytes(recipe: &PathRecipe) -> alloc::vec::Vec<u8> {
    let mut out = alloc::vec::Vec::with_capacity(1 + recipe.steps.len() * 2);
    out.push(recipe.steps.len() as u8);
    for s in &recipe.steps {
        out.push(s.left);
        out.push(s.right);
    }
    out
}

pub fn path_from_bytes(bytes: &[u8]) -> PssResult<PathRecipe> {
    if bytes.is_empty() {
        return Err(PssError::Other("empty path recipe"));
    }
    let n = bytes[0] as usize;
    if bytes.len() != 1 + n * 2 {
        return Err(PssError::Other("invalid path recipe length"));
    }
    let mut steps = alloc::vec::Vec::with_capacity(n);
    for i in 0..n {
        steps.push(PathStep {
            left: bytes[1 + i * 2],
            right: bytes[1 + i * 2 + 1],
        });
    }
    Ok(PathRecipe { steps })
}

pub fn split_path_secret(
    path_bytes: &[u8],
    k: usize,
    indices: &[u8],
    rng: &mut impl FnMut() -> u8,
) -> PssResult<alloc::vec::Vec<alloc::vec::Vec<(usize, u8)>>> {
    split_secret(path_bytes, k, indices, rng)
}

pub fn decode_path_from_carriers(
    carriers: &[(CarrierMeta, alloc::vec::Vec<u8>)],
    k_indices: &[u8],
    path_len: usize,
) -> PssResult<alloc::vec::Vec<u8>> {
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
        for b in 0..path_len {
            let val = read_path_share_byte(data, &meta.catalog_id, meta.index, b as u64)?;
            bytes.push((b, val));
        }
        bundles.push(ShareBundle {
            index: meta.index,
            bytes,
        });
    }

    reconstruct_secret(&bundles, path_len)
}

fn carrier_point(
    carriers: &[(CarrierMeta, alloc::vec::Vec<u8>)],
    index: u8,
    byte_pos: u64,
    acc: Option<Gf256>,
) -> PssResult<(Gf256, Gf256)> {
    if index == PATH_CHAIN {
        return acc
            .map(|v| (Gf256::ZERO, v))
            .ok_or(PssError::Other("chain step before accumulator"));
    }
    let (meta, data) = carriers
        .iter()
        .find(|(m, _)| m.index == index)
        .ok_or(PssError::InsufficientShares)?;
    let val = read_share_byte(data, &meta.catalog_id, meta.index, byte_pos)?;
    Ok((Gf256::from_u8(index), Gf256::from_u8(val)))
}

/// Chain Lagrange steps per recipe; reads seed-domain share bytes at `byte_pos`.
pub fn combo_decode_byte(
    carriers: &[(CarrierMeta, alloc::vec::Vec<u8>)],
    recipe: &PathRecipe,
    byte_pos: u64,
) -> PssResult<u8> {
    if recipe.steps.is_empty() {
        return Err(PssError::Other("empty path recipe"));
    }
    let mut acc: Option<Gf256> = None;
    for step in &recipe.steps {
        let left = carrier_point(carriers, step.left, byte_pos, acc)?;
        let right = carrier_point(carriers, step.right, byte_pos, acc)?;
        acc = Some(lagrange_at_zero(&[left, right]));
    }
    Ok(acc.unwrap().to_u8())
}

pub fn combo_decode_seed(
    carriers: &[(CarrierMeta, alloc::vec::Vec<u8>)],
    recipe: &PathRecipe,
    seed_len: usize,
) -> PssResult<[u8; 32]> {
    let mut out = alloc::vec::Vec::with_capacity(seed_len);
    for b in 0..seed_len {
        out.push(combo_decode_byte(carriers, recipe, b as u64)?);
    }
    let mut seed = [0u8; 32];
    let n = out.len().min(32);
    seed[..n].copy_from_slice(&out[..n]);
    Ok(seed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_roundtrip_bytes() {
        let recipe = PathRecipe {
            steps: alloc::vec![
                PathStep { left: 1, right: 2 },
                PathStep { left: PATH_CHAIN, right: 1 },
            ],
        };
        let bytes = path_to_bytes(&recipe);
        let back = path_from_bytes(&bytes).unwrap();
        assert_eq!(back, recipe);
    }
}
