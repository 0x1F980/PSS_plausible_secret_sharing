# PSS Mathematics — Specification, Postulates & Proofs

## License: GNU GPLv3 Only
## Target: Mathematicians, cryptographers & independent reviewers

*(Repository: PSS_plausible_secret_sharing. Rust identifiers appear in Appendix A.)*

**Read first:** [PSS_SECURITY_LAYERS.md](PSS_SECURITY_LAYERS.md) · [README.md](README.md)

---

## Purpose

PSS stores a secret by **reading** Shamir share bytes from **untouched** carrier files at public, deterministic offsets. Security has **independent** parts:

1. **Shamir ITS (information-theoretic):** Fewer than k shares ⇒ H(S | obs) = H(S) against unbounded computation.
2. **Camouflage (operational):** Carriers look like ordinary files; no sidecar keys on disk.
3. **Corpus search (operational):** Setup finds n files whose existing bytes match shares — not ITS.

**Not claimed:** hash one-wayness, collision resistance, or combinatorial ITS from pool size alone.

> **WARNING FOR REVIEWERS:** SHA-256 derives **public** catalog IDs and offsets only. ITS is Shamir, not hash hardness.

**Reviewer task:** Read **§0.1** (worked example), postulates, capacity sketch, then **Appendix A**.

---

## 0. Notation

| Symbol | Meaning |
|--------|---------|
| GF(256) | Field with 256 elements; add = XOR |
| k | Shamir threshold |
| n | Total share count |
| s | Secret byte (or byte vector) |
| f | Degree-(k−1) polynomial, f(0) = s |
| (i, f(i)) | Share at index i |
| L_i | Carrier file size in bytes |
| L₁ ≤ … ≤ L_n | Sorted selected carrier sizes |
| \|F*\|\_max | Max contiguous secret = L\_{n−k+1} |
| τ(P,b) | Transpose offset for byte position b in carrier P |
| catalog_id | Public file identity (path hash) |

---

## 0.1 Worked example (read this first)

**Parameters:** k = 2, n = 3, indices {1, 2, 3}, secret byte s = 0xAB.

### Shamir split in GF(256)

Choose random a₁ = 0x11. Polynomial:

```text
f(x) = 0xAB + 0x11·x   (coefficients in GF(256), add = XOR)
```

| Index i | f(i) |
|---------|------|
| 1 | 0xBA |
| 2 | 0x89 |
| 3 | 0xF8 |

### Reconstruction from k = 2 shares

Use shares (1, 0xBA) and (2, 0x89). Lagrange at x = 0 yields f(0) = 0xAB.

*(Matches `tests/lagrange_roundtrip.rs` and `pss_field_gf256::lagrange_known_gf256`.)*

### ITS with t = 1 share

Eve sees only (1, 0xBA). For any candidate secret s′, choose a₁′ = f(1) − s′; a consistent degree-1 polynomial exists. Posterior on s remains **uniform** — exactly 256 equally likely secrets.

*(Tested: `tests/shamir_its_property.rs`.)*

### Capacity example (GB units)

Selected sizes (sorted): L = [2, 3, 4, 5, 6], k = 3, n = 5.

```text
|F*|_max = L_{n−k+1} = L_3 = 4 GB
%_pool ≈ 4 / (2+3+4+5+6) = 20%
```

Per byte b: eligible carriers {i : L_i > b}; require |eligible| ≥ k.

*(Tested: `tests/segment_capacity.rs`, `tests/capacity_cli.rs`.)*

### k+1 false-positive rejection

Eve finds k random bytes matching k shares by luck. Share k+1 fails Lagrange consistency unless all k+1 lie on the same degree-(k−1) polynomial.

*(Implementation: `pss_verify.rs`.)*

---

## Postulates

| ID | Postulate |
|----|-----------|
| **P0** | Arithmetic in GF(256); addition is XOR. |
| **P1** | Eve knows algorithm, public spec, catalog paths, and any shares she holds. Unbounded computation. |
| **P2** | Shamir ITS: t < k shares ⇒ uniform f(0). |
| **P3** | Transposition offsets τ are **public** derivations (SHA-256 plumbing). |
| **P4** | Setup copies **untouched** files; no impose / rewrite. |
| **P5** | Fusion (Lagrange) **combines** existing bytes; it does not create them. |
| **P6** | Combinatorics / pool size improves camouflage, not ITS. |
| **P7** | SSS_CHAIN is **not** part of PSS security. |
| **P8** | Fractal tiers change compartmentation only, not L\_{n−k+1} %. |

---

## 1. Shamir ITS sketch

**Theorem:** For random degree-(k−1) polynomial over GF(256), any t < k share values give no information about f(0).

*Sketch:* t points constrain a t−1 degree subspace; f(0) varies uniformly as the remaining coefficients range over GF(256).

**Corollary:** H(S | obs) = H(S) for byte secrets split independently per position (standard Shamir).

---

## 2. Capacity: |F*|\_max = L\_{n−k+1}

**Sketch:** At byte offset b (0-based), only carriers with L_i > b can hold that byte. For a secret of length |F*|, we need at least k eligible carriers for every b ∈ {0, …, |F*|−1}. With sorted sizes, the binding constraint is the (n−k+1)-th smallest among the selected n files.

See [PSS_FORMULAS.md](PSS_FORMULAS.md) for formulas.

---

## 3. What SHA-256 is NOT

SHA-256 in PSS:

- Derives **public** `catalog_id` from path
- Derives **public** transpose offsets τ(P,b)

SHA-256 does **not**:

- Provide ITS against < k share leakage
- Act as one-way hiding for the secret
- Replace Shamir threshold security

Do not classify PSS as CDS-style hash storage or combinatorial secret splitting.

---

## 4. Universe false positives

Large corpus ⇒ more accidental byte matches. Mitigation: **k+1 Shamir consistency** (`pss verify`) — Lagrange predict on extra share. Not SSS_CHAIN.

---

## 5. Indistinguishability ≠ ITS

File types and pool composition aid **deniability**; they do not replace Shamir ITS for share leakage.

---

## 6. Confirm / reject checklist (reviewers)

| Claim | Confirm if | Reject if |
|-------|------------|-----------|
| ITS for < k shares | Shamir over GF(256), random coeffs | Hash hardness cited as ITS |
| Disk has no sidecars | setup outputs n untouched files | `.key` / manifest required |
| Max secret size | L\_{n−k+1} from sorted sizes | min(L_i) used instead |
| SHA role | public plumbing only | SHA called security boundary |
| SSS_CHAIN | absent from Cargo.toml | sss_chain dependency present |
| Large F* practical | corpus search / extract mode | impose assumed in scope |

---

## 7. Combo path (optional ITS)

Recipe **W** = sequence of `(left, right)` pairs; index `0` = chained accumulator (genvalg). Serialized and Shamir-split on domain `PSS-v1-path`.

- `< k_path` path shares ⇒ ITS on W (same Shamir argument as seed).
- `combo_decode_seed`: per byte, chain Lagrange steps reading seed-domain share bytes.
- **Default decode** uses sum mode only; combo is optional ([HEADS_UP](PSS_HEADS_UP.md) §9).

---

## Appendix A — Rust correspondence

| Math | Rust module |
|------|-------------|
| GF(256) | `pss_field_gf256.rs` |
| Polynomial / eval | `pss_poly.rs` |
| Lagrange at 0 | `pss_lagrange.rs` |
| Shamir split/reconstruct | `pss_shamir.rs` |
| L\_{n−k+1} | `pss_segment.rs`, `pss_capacity.rs` |
| τ offsets | `pss_transpose.rs` |
| k+1 verify | `pss_verify.rs` |
| Setup / decode | `pss_setup.rs` |
| Combo path W | `pss_path.rs` |

Formal sketches: [PSS_FORMAL_VERIFICATION.md](PSS_FORMAL_VERIFICATION.md) · `mathematics/PSS/`

---

## Portal navigation

| # | Pillar | Doc |
|---|--------|-----|
| 0 | Security Layers | [PSS_SECURITY_LAYERS.md](PSS_SECURITY_LAYERS.md) |
| 1 | Vision | [PSS_vision.md](PSS_vision.md) |
| 2 | **Mathematics** | this document |
| 3 | Manual | [PSS_manual.md](PSS_manual.md) |
| 4 | Troubleshooting | [PSS_troubleshooting.md](PSS_troubleshooting.md) |
| 5 | Use-Cases | [PSS_usecase.md](PSS_usecase.md) |
| 6 | HEADS_UP | [PSS_HEADS_UP.md](PSS_HEADS_UP.md) |

**Also:** [FORMULAS](PSS_FORMULAS.md) · [FORMAL_VERIFICATION](PSS_FORMAL_VERIFICATION.md) · [PROOF_MANIFEST](PROOF_MANIFEST.md)
