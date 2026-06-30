# PSS Vision

## License: GNU GPLv3 Only
## Target: Architects, cryptographers, operators

**Read first:** [PSS_SECURITY_LAYERS.md](PSS_SECURITY_LAYERS.md) · [README.md](README.md)

---

## Purpose

**PSS (Plausible Secret Sharing)** hides Shamir k-of-n shares inside **untouched** carrier files. Shares are read at deterministic transposition offsets; setup selects n files from a corpus whose existing bytes already match the required share values.

An adversary with fewer than k carriers learns **nothing** about the secret (information-theoretic). Recovery uses Lagrange interpolation at x = 0 in GF(256).

---

## Threat model (Eve)

| Eve holds | Result |
|-----------|--------|
| < k carriers | ITS: H(S \| obs) = H(S) |
| k carriers | Reconstruct seed S via Lagrange |
| all n carriers | Reconstruct (n ≥ k) |
| full corpus, unknown n-set | Search + k+1 verify rejects false matches |

Eve knows the public spec, algorithms, paths, and transpose function τ. She does **not** get ITS protection if she holds ≥ k shares.

---

## vs SSS / SSS_CHAIN deployment

| | SSS / SSS_CHAIN | PSS |
|---|-----------------|-----|
| Carrier content | Synthetic noise / link bytes | **Bit-identical** ordinary files |
| Algebra | Shamir / chaining | Shamir GF(256) |
| Sidecar on disk | varies by consumer | **none** (exactly n files) |
| Plausible deniability | lower | higher |
| Dependency | ecosystem crate | **standalone** (no `sss_chain`) |

Same mathematics family; **opposite deployment** — not opposite algebra.

---

## Design principles

1. **ITS core** — Shamir threshold; Lagrange at x = 0; add = XOR.
2. **Camouflage** — read-only transposition; files never modified at setup.
3. **Keyless disk** — indices from sorted catalog; no `.key`, `.pss`, manifest.
4. **Public plumbing** — SHA-256 for IDs and τ; not a security boundary.
5. **Honest capacity** — \|F*\|\_max = L\_{n−k+1}; large chosen F* without impose is impractical ([HEADS_UP](PSS_HEADS_UP.md)).

---

## Out of scope

- Rewriting / imposing bytes on carriers
- SSS_CHAIN as dependency or ITS substitute
- Hash hardness as security boundary
- Fractal tiers boosting capacity %

---

## Portal navigation

| # | Pillar | Doc |
|---|--------|-----|
| 0 | Security Layers | [PSS_SECURITY_LAYERS.md](PSS_SECURITY_LAYERS.md) |
| 1 | **Vision** | this document |
| 2 | Mathematics | [PSS_mathematics.md](PSS_mathematics.md) |
| 3 | Manual | [PSS_manual.md](PSS_manual.md) |
| 4 | Troubleshooting | [PSS_troubleshooting.md](PSS_troubleshooting.md) |
| 5 | Use-Cases | [PSS_usecase.md](PSS_usecase.md) |
| 6 | HEADS_UP | [PSS_HEADS_UP.md](PSS_HEADS_UP.md) |

**Also:** [FORMULAS](PSS_FORMULAS.md) · [FORMAL_VERIFICATION](PSS_FORMAL_VERIFICATION.md)
