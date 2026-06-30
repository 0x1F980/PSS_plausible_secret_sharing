// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980
//
// PSS — Plausible Secret Sharing
// ITS via Shamir k-of-n over GF(256) with read-only transposed carriers.

#![no_std]

extern crate alloc;

pub mod pss_capacity;
pub mod pss_catalog;
pub mod pss_error;
pub mod pss_extract;
pub mod pss_path;
pub mod pss_field_gf256;
pub mod pss_lagrange;
pub mod pss_payload;
pub mod pss_poly;
pub mod pss_segment;
pub mod pss_setup;
pub mod pss_shamir;
pub mod pss_synthetic;
pub mod pss_tier;
pub mod pss_transpose;
pub mod pss_verify;

pub use pss_capacity::{max_secret_size, pool_utilization_pct, CapacityReport};
pub use pss_catalog::{assign_indices, catalog_id, catalog_id_file, catalog_id_path, CarrierMeta};
pub use pss_error::{PssError, PssResult};
pub use pss_field_gf256::Gf256;
pub use pss_lagrange::{lagrange_at_zero, lagrange_interpolate};
pub use pss_shamir::{reconstruct_byte, reconstruct_secret, split_byte, split_secret, Share, ShareBundle};
pub use pss_path::{
    combo_decode_byte, combo_decode_seed, decode_path_from_carriers, path_from_bytes, path_to_bytes,
    split_path_secret, PathRecipe, PathStep, PATH_CHAIN,
};
pub use pss_synthetic::{build_combo_pool, combo_demo_roundtrip, SyntheticPool};
pub use pss_transpose::{
    read_path_share_byte, read_share_byte, transpose_offset, DOMAIN_PSS_V1, DOMAIN_PSS_V1_PATH,
    DOMAIN_PAYLOAD_V1,
};
pub use pss_verify::verify_shares_consistency;
