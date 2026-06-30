// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use crate::pss_error::{PssError, PssResult};
use crate::pss_segment::max_secret_size_from_sorted;

#[derive(Clone, Debug, PartialEq)]
pub struct CapacityReport {
    pub n: usize,
    pub k: usize,
    pub total_pool_bytes: u64,
    pub max_secret_bytes: u64,
    pub utilization_pct: f64,
    pub l_n_minus_k_plus_1: u64,
}

/// Compute max secret size L_{n-k+1} from file sizes (selects top n largest if more provided).
pub fn max_secret_size(sizes: &[u64], k: usize, n: usize, select_top: Option<usize>) -> PssResult<u64> {
    let report = capacity_report(sizes, k, n, select_top)?;
    Ok(report.max_secret_bytes)
}

pub fn pool_utilization_pct(max_secret: u64, total_pool: u64) -> f64 {
    if total_pool == 0 {
        return 0.0;
    }
    (max_secret as f64 / total_pool as f64) * 100.0
}

pub fn capacity_report(sizes: &[u64], k: usize, n: usize, select_top: Option<usize>) -> PssResult<CapacityReport> {
    if k == 0 {
        return Err(PssError::InvalidK);
    }
    if n < k {
        return Err(PssError::InvalidN);
    }
    let mut selected: alloc::vec::Vec<u64> = sizes.to_vec();
    selected.sort_unstable();
    selected.reverse();
    if let Some(top) = select_top {
        if top < n {
            return Err(PssError::Other("select-top must be >= n"));
        }
        selected.truncate(top.min(selected.len()));
    }
    if selected.len() < n {
        return Err(PssError::InvalidN);
    }
    selected.truncate(n);
    selected.sort_unstable();
    let total: u64 = selected.iter().sum();
    let max_secret = max_secret_size_from_sorted(&selected, k, n)?;
    Ok(CapacityReport {
        n,
        k,
        total_pool_bytes: total,
        max_secret_bytes: max_secret,
        utilization_pct: pool_utilization_pct(max_secret, total),
        l_n_minus_k_plus_1: max_secret,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capacity_23456() {
        let sizes = [2u64, 3, 4, 5, 6];
        let r = capacity_report(&sizes, 3, 5, None).unwrap();
        assert_eq!(r.max_secret_bytes, 4);
        assert_eq!(r.total_pool_bytes, 20);
        assert!((r.utilization_pct - 20.0).abs() < 0.01);
    }
}
