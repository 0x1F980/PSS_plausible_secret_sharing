// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use sha2::{Digest, Sha256};

pub fn seed_from_file(data: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(data);
    h.finalize().into()
}

pub fn otp_keystream(seed: &[u8; 32], domain: &[u8], len: usize) -> alloc::vec::Vec<u8> {
    let mut out = alloc::vec::Vec::with_capacity(len);
    let mut counter = 0u64;
    while out.len() < len {
        let mut h = Sha256::new();
        h.update(domain);
        h.update(seed);
        h.update(counter.to_le_bytes());
        let block = h.finalize();
        out.extend_from_slice(&block);
        counter += 1;
    }
    out.truncate(len);
    out
}

pub fn otp_encrypt(plaintext: &[u8], seed: &[u8; 32], domain: &[u8]) -> alloc::vec::Vec<u8> {
    let stream = otp_keystream(seed, domain, plaintext.len());
    plaintext
        .iter()
        .zip(stream.iter())
        .map(|(p, k)| p ^ k)
        .collect()
}

pub fn otp_decrypt(ciphertext: &[u8], seed: &[u8; 32], domain: &[u8]) -> alloc::vec::Vec<u8> {
    otp_encrypt(ciphertext, seed, domain)
}

pub const OTP_DOMAIN: &[u8] = b"PSS-v1-payload";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn otp_roundtrip() {
        let seed = [7u8; 32];
        let plain = b"secret payload";
        let enc = otp_encrypt(plain, &seed, OTP_DOMAIN);
        let dec = otp_decrypt(&enc, &seed, OTP_DOMAIN);
        assert_eq!(&dec, plain);
    }
}
