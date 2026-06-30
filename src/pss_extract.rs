// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use crate::pss_catalog::{assign_indices, catalog_id_path, CarrierMeta};
use crate::pss_error::PssResult;
use crate::pss_transpose::{read_share_byte, DOMAIN_PSS_V1};

/// Deterministic extract: read transposed bytes from all carriers in pool order.
pub fn extract_transposed(pool: &[(&[u8], alloc::string::String)], max_bytes: usize) -> PssResult<alloc::vec::Vec<u8>> {
    let mut metas: alloc::vec::Vec<CarrierMeta> = pool
        .iter()
        .map(|(data, path)| CarrierMeta {
            path: path.clone(),
            catalog_id: catalog_id_path(path),
            size: data.len() as u64,
            index: 0,
        })
        .collect();
    metas = assign_indices(metas);

    let mut out = alloc::vec::Vec::with_capacity(max_bytes);
    for b in 0..max_bytes as u64 {
        let mut row = alloc::vec::Vec::new();
        for (i, meta) in metas.iter().enumerate() {
            let data = pool[i].0;
            if meta.size <= b {
                continue;
            }
            let byte = read_share_byte(data, &meta.catalog_id, meta.index, b)?;
            row.push(byte);
        }
        if row.is_empty() {
            break;
        }
        out.push(row[0]);
    }
    Ok(out)
}

pub fn extract_domain_label() -> &'static [u8] {
    DOMAIN_PSS_V1
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn extract_deterministic() {
        let a = (b"file a content".as_slice(), "a.bin".to_string());
        let b = (b"file b longer content".as_slice(), "b.bin".to_string());
        let e1 = extract_transposed(&[a.clone(), b.clone()], 5).unwrap();
        let e2 = extract_transposed(&[a, b], 5).unwrap();
        assert_eq!(e1, e2);
    }
}
