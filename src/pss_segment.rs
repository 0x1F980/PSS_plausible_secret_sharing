// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use crate::pss_error::{PssError, PssResult};

/// For sorted sizes L1 <= ... <= Ln, max contiguous secret length is L_{n-k+1}.
pub fn max_secret_size_from_sorted(sorted_sizes: &[u64], k: usize, n: usize) -> PssResult<u64> {
    if k == 0 {
        return Err(PssError::InvalidK);
    }
    if n < k {
        return Err(PssError::InvalidN);
    }
    if sorted_sizes.len() < n {
        return Err(PssError::Other("need at least n file sizes"));
    }
    let idx = n - k;
    Ok(sorted_sizes[idx])
}

/// Eligible carrier indices at byte offset b (0-based): files with size > b.
pub fn eligible_indices_at_byte(sizes: &[u64], b: u64) -> alloc::vec::Vec<usize> {
    sizes
        .iter()
        .enumerate()
        .filter(|(_, &sz)| sz > b)
        .map(|(i, _)| i)
        .collect()
}

/// Minimum k eligible carriers required at each byte position.
pub fn can_encode_length(sizes: &[u64], k: usize, len: u64) -> bool {
    if len == 0 {
        return sizes.len() >= k;
    }
    (0..len).all(|b| eligible_indices_at_byte(sizes, b).len() >= k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiered_max_23456_gb() {
        let sizes = [2u64, 3, 4, 5, 6];
        assert_eq!(max_secret_size_from_sorted(&sizes, 3, 5).unwrap(), 4);
    }

    #[test]
    fn eligible_at_byte() {
        let sizes = [2u64, 3, 4, 5, 6];
        assert_eq!(eligible_indices_at_byte(&sizes, 0).len(), 5);
        assert_eq!(eligible_indices_at_byte(&sizes, 3).len(), 3);
    }
}
