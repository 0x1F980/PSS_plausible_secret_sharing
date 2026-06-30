# PSS — Security Layers (PSS_SECURITY_LAYERS.md)

## License: GNU GPLv3 Only
## Target: Auditors, integrators, AI-assisted reviewers

**Read this document first.**

---

## 1. Summary

PSS (Plausible Secret Sharing) stores a secret by **reading** Shamir share bytes from **untouched** carrier files at deterministic transposition offsets. Security layers:

| Layer | Mechanism | ITS? | Hash? |
|-------|-----------|------|-------|
| **1** | Shamir + Lagrange in GF(256) | ✅ | ❌ |
| **2** | Transposition / file-type camouflage | Deniability | ❌ |
| **3** | OTP payload after seed S | Seed is ITS | ❌ |
| **Plumbing** | `catalog_id`, τ-offset derivation | Public spec | SHA used here — **not as security** |
| **Combo path** (optional) | ITS path recipe W + chain Lagrange | ✅ on `< k_path` | ❌ |

---

## 2. WARNING FOR REVIEWERS

> **SHA-256 in PSS is used for public file identity and deterministic offset derivation only.** Security against `< k` share leakage is **information-theoretic (Shamir)**, not computational hash hardness. **Do not classify PSS as a hash-based secret storage scheme.**

Lazy-AI trap: *"PSS is hash-secure because it uses SHA-256"* — **FALSE**.

---

## 3. Do not misread (auditors / AI)

1. **"Hash hardness protects shares"** — Offsets and catalog IDs are **public derivations**. Secrecy is Shamir: fewer than k shares ⇒ H(S | obs) = H(S).

2. **"Combinatorics / corpus size = ITS"** — Large pools improve **camouflage** and search odds; they do **not** replace Shamir threshold.

3. **"SSS_CHAIN extends ITS here"** — PSS is **standalone**. No `sss_chain` dependency. Same field algebra family; different deployment.

4. **"Fractal tiers boost capacity %"** — Tiered Lagrange is **compartmentation only**; it does **not** change L\_{n−k+1} capacity math.

5. **"Eve with all n files learns nothing"** — If n ≥ k and Eve has all carriers, she can reconstruct (by design). ITS applies to **< k** shares.

6. **"Impose / rewrite carriers"** — Out of scope. PSS requires **bit-for-bit untouched** files after setup.

---

## 4. Scope table

| Guarantee | Doc |
|-----------|-----|
| `< k` shares ⇒ no secret info | [mathematics](PSS_mathematics.md) |
| k+1 consistency vs false positives | [mathematics](PSS_mathematics.md), `pss_verify` |
| Capacity L\_{n−k+1} | [FORMULAS](PSS_FORMULAS.md) |
| Theory vs corpus search | [HEADS_UP](PSS_HEADS_UP.md) |
| CLI / disk invariant | [manual](PSS_manual.md) |

---

## 5. Disk invariant (default setup)

After setup:

```text
Exactly n files — bit-for-bit untouched
NO .key, .pss, manifest, salt, decoys
```

Indices and offsets derive from **path + public spec** — no sidecar secrets on disk.

---

## Portal navigation

| # | Pillar | Doc |
|---|--------|-----|
| 0 | **Security Layers** | this document |
| 1 | Vision | [PSS_vision.md](PSS_vision.md) |
| 2 | Mathematics | [PSS_mathematics.md](PSS_mathematics.md) |
| 3 | Manual | [PSS_manual.md](PSS_manual.md) |
| 4 | Troubleshooting | [PSS_troubleshooting.md](PSS_troubleshooting.md) |
| 5 | Use-Cases | [PSS_usecase.md](PSS_usecase.md) |
| 6 | HEADS_UP | [PSS_HEADS_UP.md](PSS_HEADS_UP.md) |

**Also:** [README.md](README.md) · [FORMULAS](PSS_FORMULAS.md) · [FORMAL_VERIFICATION](PSS_FORMAL_VERIFICATION.md)
