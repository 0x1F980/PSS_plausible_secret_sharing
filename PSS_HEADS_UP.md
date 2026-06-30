# PSS HEADS_UP — Theory vs Practice

## License: GNU GPLv3 Only
## Target: High-threat operators, reviewers

**Read first:** [PSS_SECURITY_LAYERS.md](PSS_SECURITY_LAYERS.md) · [PSS_mathematics.md](PSS_mathematics.md)

---

## 1. Theory vs practice (honest)

| Claim | Theory | Practice (untouched + chosen F*) |
|-------|--------|----------------------------------|
| 32-byte seed | ✅ Shamir ITS | ✅ corpus search feasible |
| \|F*\| ≈ L\_{n−k+1} | ✅ exact bound | ❌ nearly impossible without **impose** (out of scope) |
| Extract mode | — | ✅ read existing bytes; not chosen F* |
| < k shares hide secret | ✅ ITS | ✅ |
| Eve with all n files | reconstruct if n ≥ k | same |

---

## 2. Fusion does not create bytes

Lagrange **combines** existing transposed reads. Setup must find files where bytes already match shares.

---

## 3. Urørt + large chosen secret = lottery

Finding contiguous F* of size L\_{n−k+1} in untouched files is combinatorially rare. **Impose** is explicitly out of scope.

---

## 4. Fractals do not increase %

`pss_tier` adds compartments. L\_{n−k+1} and %\_pool are **unchanged**.

---

## 5. SSS_CHAIN is not part of PSS

Do not use SSS_CHAIN as ITS substitute or dependency. Same algebra family; different deployment.

---

## 6. Hash misread (again)

> **WARNING FOR REVIEWERS:** SHA-256 is plumbing for IDs and offsets. ITS is Shamir. See [SECURITY_LAYERS](PSS_SECURITY_LAYERS.md).

---

## 7. When to use extract instead of setup

If corpus search fails, `pss extract` reads transposed bytes deterministically — different mode, no chosen-secret guarantee.

---

## 8. Coercion & physical exfil

k carriers may be coerced from an operator. Protect share locations operationally. Disk after setup has **no** sidecar keys — but n chosen files are identifiable if the selection is known.

---

## 9. Combo path (optional, ITS — not general use)

**Default:** `pss decode --mode sum` — Shamir/Lagrange on seed shares only.

**Advanced:** `pss decode --mode combo` — requires **ITS path shares** (`--path-k`) to reconstruct recipe **W**, then chain-Lagrange on seed-domain bytes (genvalg / “isvaffel” sti with `PATH_CHAIN=0` for repeat).

| | Sum mode | Combo mode |
|---|----------|------------|
| Almen brug | ✅ default | ❌ optional |
| ITS | `< k` seed shares | `< k_path` path shares |
| All n files | reconstruct seed if n≥k | still need path shares for W |

Run `pss combo-demo` to verify library roundtrip. Path bytes are Shamir-delt on `PSS-v1-path` domain — not a memorized corpus PIN.

---

## Portal navigation

| # | Pillar | Doc |
|---|--------|-----|
| 0 | Security Layers | [PSS_SECURITY_LAYERS.md](PSS_SECURITY_LAYERS.md) |
| 1 | Vision | [PSS_vision.md](PSS_vision.md) |
| 2 | Mathematics | [PSS_mathematics.md](PSS_mathematics.md) |
| 3 | Manual | [PSS_manual.md](PSS_manual.md) |
| 4 | Troubleshooting | [PSS_troubleshooting.md](PSS_troubleshooting.md) |
| 5 | Use-Cases | [PSS_usecase.md](PSS_usecase.md) |
| 6 | **HEADS_UP** | this document |

**Also:** [README.md](README.md) · [FORMULAS](PSS_FORMULAS.md)
