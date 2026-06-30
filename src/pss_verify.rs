// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use crate::pss_error::{PssError, PssResult};
use crate::pss_shamir::{verify_share_consistency, Share};

/// Verify k shares plus one extra share lie on the same Shamir polynomial (anti false match).
pub fn verify_shares_consistency(k_shares: &[Share], extra: Share) -> PssResult<()> {
    if k_shares.len() < 2 {
        return Err(PssError::InsufficientShares);
    }
    if !verify_share_consistency(k_shares, extra)? {
        return Err(PssError::InconsistentShares);
    }
    Ok(())
}

/// Verify full share set: every (k+1) tuple must be consistent when possible.
pub fn verify_all_shares(shares: &[Share]) -> PssResult<()> {
    if shares.len() < 3 {
        return Ok(());
    }
    let k = shares.len() - 1;
    let subset = &shares[..k];
    verify_shares_consistency(subset, shares[k])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pss_field_gf256::Gf256;
    use crate::pss_shamir::split_byte;

    #[test]
    fn reject_false_share() {
        let shares = split_byte(99, 2, &[1, 2, 3], &[42]).unwrap();
        let bad = (3u8, Gf256(0));
        assert!(verify_shares_consistency(&shares[..2], bad).is_err());
    }
}
