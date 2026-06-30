// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use sha2::{Digest, Sha256};

pub fn catalog_id(data: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(data);
    h.finalize().into()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CarrierMeta {
    pub path: alloc::string::String,
    pub catalog_id: [u8; 32],
    pub size: u64,
    pub index: u8,
}

/// SHA-256 identity of carrier path (stable for untouched files under transpose).
pub fn catalog_id_path(path: &str) -> [u8; 32] {
    catalog_id(path.as_bytes())
}

/// SHA-256 of raw file bytes (informational / verification only).
pub fn catalog_id_file(data: &[u8]) -> [u8; 32] {
    catalog_id(data)
}

/// Assign Shamir indices 1..n by stable sort of catalog_id among n carriers.
pub fn assign_indices(mut carriers: alloc::vec::Vec<CarrierMeta>) -> alloc::vec::Vec<CarrierMeta> {
    carriers.sort_by(|a, b| a.catalog_id.cmp(&b.catalog_id));
    for (i, c) in carriers.iter_mut().enumerate() {
        c.index = (i + 1) as u8;
    }
    carriers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_index_order() {
        let c1 = CarrierMeta {
            path: "a".into(),
            catalog_id: [2u8; 32],
            size: 100,
            index: 0,
        };
        let c2 = CarrierMeta {
            path: "b".into(),
            catalog_id: [1u8; 32],
            size: 200,
            index: 0,
        };
        let out = assign_indices(alloc::vec![c1, c2]);
        assert_eq!(out[0].index, 1);
        assert_eq!(out[1].index, 2);
    }
}
