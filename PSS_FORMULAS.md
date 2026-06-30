# PSS Formulas

## License: GNU GPLv3 Only
## Target: Reviewers, capacity planners

**Read first:** [PSS_SECURITY_LAYERS.md](PSS_SECURITY_LAYERS.md) · [PSS_mathematics.md](PSS_mathematics.md)

---

## Shamir split (one byte)

```text
f(x) = a_0 + a_1·x + … + a_{k−1}·x^{k−1}   (coefficients in GF(256))
a_0 = secret byte
share_i = f(i)
```

## Lagrange at zero

```text
f(0) = Σ_{j=1}^{k} y_j · Π_{m≠j} x_m / (x_m − x_j)
```

Division and product in GF(256); outer sum uses XOR.

## Transpose offset

```text
τ(P, b) = H("PSS-v1" || catalog_id || index || le64(b)) mod (|P| − 1)
```

Payload domain: `"PSS-v1-payload"` with block index.

## Capacity

Sort selected sizes ascending: L₁ ≤ … ≤ Lₙ.

```text
|F*|_max     = L_{n−k+1}
eligible(b)  = { i : L_i > b }
require      = |eligible(b)| ≥ k  for all b < |F*|
%_pool       ≈ |F*|_max / Σ_{i=1}^{n} L_i
```

CLI: `pss capacity --pool DIR --k K --n N [--select-top N]`

### Reference example (GB units)

| L₁ | L₂ | L₃ | L₄ | L₅ | k | n | \|F*\|_max |
|----|----|----|----|----|---|---|------------|
| 2  | 3  | 4  | 5  | 6  | 3 | 5 | **4**      |

%\_pool ≈ 4/20 = 20%.

## Catalog index assignment

Paths sorted lexicographically; Shamir indices 1..n assigned in order (`assign_indices`).

## OTP payload

```text
C = P ⊕ H("PSS-OTP-v1" || seed || domain || counter)
```

## Verify (k+1)

Given k shares defining f, check share\_{k+1} via Lagrange predict — rejects universe false positives.

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
| 6 | HEADS_UP | [PSS_HEADS_UP.md](PSS_HEADS_UP.md) |

**Also:** [README.md](README.md) · [FORMAL_VERIFICATION](PSS_FORMAL_VERIFICATION.md)
