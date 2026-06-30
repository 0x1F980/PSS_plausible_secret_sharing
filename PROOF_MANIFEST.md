# PSS — Proof manifest

## License: GNU GPLv3 Only

| Concern | Rust module | Test / Lean |
|---------|-------------|-------------|
| GF(256) field axioms | `pss_field_gf256.rs` | unit + `gf256_field.rs` |
| Polynomial eval | `pss_poly.rs` | unit |
| Lagrange at x=0 | `pss_lagrange.rs` | `lagrange_roundtrip.rs` |
| Shamir split/reconstruct | `pss_shamir.rs` | `shamir_its_property.rs` |
| k+1 verify | `pss_verify.rs` | unit |
| Tiered max \|F*\| | `pss_segment.rs`, `pss_capacity.rs` | `segment_capacity.rs` |
| Transpose determinism | `pss_transpose.rs` | `transpose_determinism.rs` |
| Keyless disk | `pss_setup.rs` | `keyless_disk.rs` |
| CLI capacity parity | `src/bin/pss.rs` | `capacity_cli.rs` |
| Combo path ITS + chain | `pss_path.rs`, `pss_synthetic.rs` | `combo_roundtrip.rs`, `path_its_property.rs`, `combo_its_gate.rs` |
| CLI combo-demo | `src/bin/pss.rs` | `capacity_cli.rs` |
| OTP decode CLI | `src/bin/pss.rs` | `decode_cli.rs` |

**Lean (in-repo):** `mathematics/PSS/` — see [PSS_FORMAL_VERIFICATION.md](PSS_FORMAL_VERIFICATION.md)

**Tests:** `cargo test`  
**Lean:** `lake build` in `mathematics/`

**Forbidden dependency:** `sss_chain` — not in `Cargo.toml`, not in proof scope.
