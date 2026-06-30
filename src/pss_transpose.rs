// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use crate::pss_error::{PssError, PssResult};
use sha2::{Digest, Sha256};

pub const DOMAIN_PSS_V1: &[u8] = b"PSS-v1";
pub const DOMAIN_PSS_V1_PATH: &[u8] = b"PSS-v1-path";
pub const DOMAIN_PAYLOAD_V1: &[u8] = b"PSS-v1-payload";

pub fn transpose_offset(
    domain: &[u8],
    catalog_id: &[u8; 32],
    index: u8,
    byte_pos: u64,
    file_len: u64,
) -> PssResult<u64> {
    if file_len < 2 {
        return Err(PssError::FileTooShort);
    }
    let mut h = Sha256::new();
    h.update(domain);
    h.update(catalog_id);
    h.update([index]);
    h.update(byte_pos.to_le_bytes());
    let digest = h.finalize();
    let val = u64::from_le_bytes(digest[0..8].try_into().unwrap());
    Ok(val % (file_len - 1))
}

pub fn read_share_byte(
    file: &[u8],
    catalog_id: &[u8; 32],
    index: u8,
    byte_pos: u64,
) -> PssResult<u8> {
    let off = transpose_offset(DOMAIN_PSS_V1, catalog_id, index, byte_pos, file.len() as u64)?;
    file.get(off as usize)
        .copied()
        .ok_or(PssError::FileTooShort)
}

pub fn read_path_share_byte(
    file: &[u8],
    catalog_id: &[u8; 32],
    index: u8,
    byte_pos: u64,
) -> PssResult<u8> {
    let off = transpose_offset(DOMAIN_PSS_V1_PATH, catalog_id, index, byte_pos, file.len() as u64)?;
    file.get(off as usize)
        .copied()
        .ok_or(PssError::FileTooShort)
}

pub fn read_payload_byte(
    file: &[u8],
    catalog_id: &[u8; 32],
    index: u8,
    block: u64,
    j: u64,
) -> PssResult<u8> {
    let mut h = Sha256::new();
    h.update(DOMAIN_PAYLOAD_V1);
    h.update(catalog_id);
    h.update([index]);
    h.update(block.to_le_bytes());
    h.update(j.to_le_bytes());
    let digest = h.finalize();
    let val = u64::from_le_bytes(digest[0..8].try_into().unwrap());
    if file.len() < 2 {
        return Err(PssError::FileTooShort);
    }
    let off = val % (file.len() as u64 - 1);
    file.get(off as usize)
        .copied()
        .ok_or(PssError::FileTooShort)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pss_catalog::catalog_id_path;

    #[test]
    fn offset_deterministic() {
        let _data = b"hello world";
        let id = catalog_id_path("hello.bin");
        let o1 = transpose_offset(DOMAIN_PSS_V1, &id, 1, 0, 100).unwrap();
        let o2 = transpose_offset(DOMAIN_PSS_V1, &id, 1, 0, 100).unwrap();
        assert_eq!(o1, o2);
    }
}
