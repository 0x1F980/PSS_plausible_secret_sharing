# PSS Formal Verification

## License: GNU GPLv3 Only

This document maps Rust implementation ↔ Lean sketches ↔ integration tests.

## Rust ↔ Lean map

| Rust | Lean | Status |
|------|------|--------|
| `pss_field_gf256.rs` | `PSS/Field/GF256.lean` | axioms + add=XOR |
| `pss_lagrange.rs` | `PSS/Lagrange/AtZero.lean` | reconstruction sketch |
| `pss_shamir.rs` | `PSS/Shamir/Threshold.lean` | degree k−1 uniqueness |
| ITS `< k` uniform | `PSS/ITS/ZeroInformation.lean` | statement + sketch |
| `pss_segment.rs` | `PSS/Segment/TieredMax.lean` | \|F*\|\_max = L\_{n−k+1} |

## Build Lean

```bash
cd mathematics
lake build
```

## Test alignment

| Property | Rust test | Lean file |
|----------|-----------|-----------|
| GF(256) mul inverse | `gf256_mul_inverse` | `GF256.lean` |
| Lagrange roundtrip | `lagrange_known_gf256` | `AtZero.lean` |
| Shamir ITS | `shamir_its_property.rs` | `ZeroInformation.lean` |
| Capacity 2,3,4,5,6 → 4 | `segment_capacity.rs` | `TieredMax.lean` |

## Out of formal scope (documented only)

- Corpus search / combinatorial camouflage
- SHA-256 plumbing (public, not ITS)
- SSS_CHAIN chaining
- File impose / rewrite

## Reviewer note

Lean files are **proof sketches** aligned with Rust tests, not a complete Mathlib formalization of GF(256). Cryptographic ITS claim rests on standard Shamir over finite fields, implemented and tested in Rust.
