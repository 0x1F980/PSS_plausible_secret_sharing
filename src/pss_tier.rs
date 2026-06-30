// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use crate::pss_error::{PssError, PssResult};
use crate::pss_lagrange::lagrange_at_zero;
use crate::pss_field_gf256::Gf256;
use crate::pss_shamir::{split_byte, Share};

#[derive(Clone, Debug)]
pub struct TierConfig {
    pub k1: usize,
    pub n1: usize,
    pub k2: usize,
    pub n2: usize,
}

/// Hierarchical Lagrange: root secret from k1 branch values; each branch from k2 leaf shares.
pub fn tier_encode_root(
    secret: u8,
    cfg: &TierConfig,
    rng: &mut impl FnMut() -> u8,
) -> PssResult<(alloc::vec::Vec<Share>, alloc::vec::Vec<alloc::vec::Vec<Share>>)> {
    if cfg.k1 == 0 || cfg.k2 == 0 || cfg.n1 < cfg.k1 || cfg.n2 < cfg.k2 {
        return Err(PssError::InvalidN);
    }

    let branch_indices: alloc::vec::Vec<u8> = (1..=cfg.n1 as u8).collect();
    let branch_random: alloc::vec::Vec<u8> = (0..cfg.k1 - 1).map(|_| rng()).collect();
    let branch_shares = split_byte(secret, cfg.k1, &branch_indices, &branch_random)?;

    let mut leaves = alloc::vec::Vec::new();
    for (_, branch_val) in branch_shares.iter().take(cfg.n1) {
        let leaf_indices: alloc::vec::Vec<u8> = (1..=cfg.n2 as u8).collect();
        let leaf_random: alloc::vec::Vec<u8> = (0..cfg.k2 - 1).map(|_| rng()).collect();
        let leaf_shares = split_byte(branch_val.to_u8(), cfg.k2, &leaf_indices, &leaf_random)?;
        leaves.push(leaf_shares);
    }

    Ok((branch_shares, leaves))
}

pub fn tier_decode_root(
    branch_indices: &[Share],
    leaf_groups: &[alloc::vec::Vec<Share>],
    cfg: &TierConfig,
) -> PssResult<u8> {
    if cfg.k1 == 0 || cfg.k2 == 0 {
        return Err(PssError::InvalidN);
    }

    let mut branch_values = alloc::vec::Vec::new();
    for g in 0..cfg.n1.min(leaf_groups.len()) {
        let leaves = &leaf_groups[g];
        if leaves.len() < cfg.k2 {
            continue;
        }
        let points: alloc::vec::Vec<(Gf256, Gf256)> = leaves[..cfg.k2]
            .iter()
            .map(|&(i, v)| (Gf256::from_u8(i), v))
            .collect();
        let branch_byte = lagrange_at_zero(&points).to_u8();
        let branch_index = branch_indices.get(g).map(|s| s.0).unwrap_or((g + 1) as u8);
        branch_values.push((branch_index, Gf256::from_u8(branch_byte)));
    }

    if branch_values.len() < cfg.k1 {
        return Err(PssError::InsufficientShares);
    }
    let root_points: alloc::vec::Vec<(Gf256, Gf256)> = branch_values[..cfg.k1]
        .iter()
        .map(|&(i, v)| (Gf256::from_u8(i), v))
        .collect();
    Ok(lagrange_at_zero(&root_points).to_u8())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier_roundtrip() {
        let cfg = TierConfig { k1: 2, n1: 3, k2: 2, n2: 3 };
        let mut c = 11u8;
        let (branches, leaves) = tier_encode_root(42, &cfg, &mut || {
            c = c.wrapping_add(5);
            c
        }).unwrap();
        let root = tier_decode_root(&branches, &leaves, &cfg).unwrap();
        assert_eq!(root, 42);
    }
}
