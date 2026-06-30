// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use crate::pss_error::{PssError, PssResult};
use crate::pss_field_gf256::Gf256;
use crate::pss_lagrange::{lagrange_at_zero, lagrange_interpolate};
use crate::pss_poly::Polynomial;

pub type Share = (u8, Gf256);

/// Split one secret byte into n shares with threshold k.
pub fn split_byte(secret: u8, k: usize, indices: &[u8], random_coeffs: &[u8]) -> PssResult<alloc::vec::Vec<Share>> {
    if k == 0 || indices.len() < k {
        return Err(PssError::InvalidK);
    }
    if random_coeffs.len() != k - 1 {
        return Err(PssError::Other("random coefficient count must be k-1"));
    }

    let mut coeffs = alloc::vec![Gf256::from_u8(secret)];
    for &r in random_coeffs {
        coeffs.push(Gf256::from_u8(r));
    }
    let poly = Polynomial::new(coeffs);

    Ok(indices
        .iter()
        .map(|&idx| (idx, poly.evaluate(Gf256::from_u8(idx))))
        .collect())
}

/// Reconstruct secret byte from at least k shares.
pub fn reconstruct_byte(shares: &[Share]) -> PssResult<Gf256> {
    if shares.is_empty() {
        return Err(PssError::InsufficientShares);
    }
    let points: alloc::vec::Vec<(Gf256, Gf256)> = shares
        .iter()
        .map(|&(i, v)| (Gf256::from_u8(i), v))
        .collect();
    Ok(lagrange_at_zero(&points))
}

/// Verify share (k+1)th point lies on same polynomial as first k shares.
pub fn verify_share_consistency(shares: &[Share], extra: Share) -> PssResult<bool> {
    if shares.len() < 2 {
        return Err(PssError::InsufficientShares);
    }
    let points: alloc::vec::Vec<(Gf256, Gf256)> = shares
        .iter()
        .map(|&(i, v)| (Gf256::from_u8(i), v))
        .collect();
    let expected = lagrange_interpolate(&points, Gf256::from_u8(extra.0));
    Ok(expected == extra.1)
}

/// Split a byte slice into share maps per index.
pub fn split_secret(
    secret: &[u8],
    k: usize,
    indices: &[u8],
    rng: &mut impl FnMut() -> u8,
) -> PssResult<alloc::vec::Vec<alloc::vec::Vec<(usize, u8)>>> {
    if k == 0 || indices.len() < k {
        return Err(PssError::InvalidK);
    }
    let n = indices.len();
    let mut per_share: alloc::vec::Vec<alloc::vec::Vec<(usize, u8)>> =
        alloc::vec::Vec::from_iter((0..n).map(|_| alloc::vec::Vec::with_capacity(secret.len())));

    for (pos, &byte) in secret.iter().enumerate() {
        let random: alloc::vec::Vec<u8> = (0..k - 1).map(|_| rng()).collect();
        let shares = split_byte(byte, k, indices, &random)?;
        for (i, (_, val)) in shares.iter().enumerate() {
            per_share[i].push((pos, val.to_u8()));
        }
    }
    Ok(per_share)
}

/// Reconstruct secret bytes from k share bundles (index, Vec of (pos, byte)).
pub fn reconstruct_secret(
    share_bundles: &[ShareBundle],
    len: usize,
) -> PssResult<alloc::vec::Vec<u8>> {
    if share_bundles.is_empty() {
        return Err(PssError::InsufficientShares);
    }
    let mut out = alloc::vec![0u8; len];
    for pos in 0..len {
        let mut points = alloc::vec::Vec::with_capacity(share_bundles.len());
        for bundle in share_bundles {
            let val = bundle
                .bytes
                .iter()
                .find(|(p, _)| *p == pos)
                .map(|(_, b)| *b)
                .ok_or(PssError::InsufficientShares)?;
            points.push((bundle.index, Gf256::from_u8(val)));
        }
        out[pos] = reconstruct_byte(&points)?.to_u8();
    }
    Ok(out)
}

#[derive(Clone, Debug)]
pub struct ShareBundle {
    pub index: u8,
    pub bytes: alloc::vec::Vec<(usize, u8)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_rng(counter: &mut u8) -> impl FnMut() -> u8 + '_ {
        move || {
            *counter = counter.wrapping_add(17);
            *counter
        }
    }

    #[test]
    fn shamir_roundtrip() {
        let secret = b"PSS test secret!!";
        let indices = [1u8, 2, 3, 4, 5];
        let mut counter = 3u8;
        let per_share = split_secret(secret, 3, &indices, &mut test_rng(&mut counter)).unwrap();

        let bundles: alloc::vec::Vec<ShareBundle> = [1usize, 2, 3]
            .iter()
            .map(|&i| ShareBundle {
                index: indices[i],
                bytes: per_share[i].clone(),
            })
            .collect();

        let recovered = reconstruct_secret(&bundles, secret.len()).unwrap();
        assert_eq!(&recovered, secret);
    }

    #[test]
    fn verify_consistency() {
        let indices = [1u8, 2, 3];
        let shares = split_byte(42, 2, &indices, &[17]).unwrap();
        let subset: alloc::vec::Vec<Share> = shares[..2].to_vec();
        assert!(verify_share_consistency(&subset, shares[2]).unwrap());
    }
}
