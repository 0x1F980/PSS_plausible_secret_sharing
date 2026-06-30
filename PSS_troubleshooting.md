# PSS Troubleshooting

## License: GNU GPLv3 Only
## Target: Operators debugging setup/decode failures

**Read first:** [PSS_SECURITY_LAYERS.md](PSS_SECURITY_LAYERS.md) · [PSS_manual.md](PSS_manual.md)

---

## setup: CorpusSearchFailed

**Cause:** No combination of n files matches Shamir shares for the secret.

**Mitigations:**

- Increase corpus size
- Lower `--min-size`
- Use smaller secret (32-byte seed is practical)
- See [HEADS_UP](PSS_HEADS_UP.md): large F* without impose is a lottery

---

## decode: InsufficientShares

**Cause:** Fewer than k indices, or index missing from pool.

**Fix:** Run `pss info --pool DIR`; pass k distinct indices.

---

## verify failed / false match

**Cause:** k random bytes matched by luck; k+1 fails Lagrange consistency.

**Fix:** Always `pss verify` with k+1 indices before trusting decode.

---

## capacity mismatch

**Cause:** Used min(L_i) instead of L\_{n−k+1}.

**Fix:**

```bash
pss capacity --pool ./carriers --k 3 --n 5
# optional: --select-top N to restrict candidate pool
```

Must match `pss_capacity.rs` / `tests/capacity_cli.rs`.

---

## Wrong offsets after rename

**Cause:** `catalog_id_path` — offsets depend on **path**, not content.

**Fix:** Preserve paths or re-run setup with consistent names.

---

## metadata / timestamps

PSS does not hide filesystem metadata. Content remains untouched; mtime may change on copy.

---

## tier decode errors

Check `TierConfig` k1/n1/k2/n2. Tiers do not fix corpus search or capacity.

---

## Combo decode / path errors

**`path decode error` / `InsufficientShares`**

Cause: Fewer than `k_path` path share indices, or path shares missing from pool.

Fix: Run `pss info --pool DIR`; pass at least 2 distinct path indices via `--path-k`. Path bytes use domain `PSS-v1-path` (separate from seed shares).

**`path recipe error`**

Cause: Wrong `--path-len`, corrupted path shares, or invalid recipe serialization.

Fix: Match `--path-len` to embedded path byte count (default combo recipe is 5 bytes). Use `pss combo-demo` to verify library roundtrip.

**`combo verify: seed mismatch`**

Cause: Path recipe W reconstructs but chain-Lagrange seed differs from sum decode on `--seed-k` indices — wrong path, wrong mode, or mismatched indices.

Fix: Confirm combo recipe and path share set. Sum mode alone may still work if seed shares are valid.

**All n carrier files but combo fails**

Having every file in the pool is **not** enough for combo mode: you still need **ITS path shares** (`< k_path` path shares reveal nothing about W). See [HEADS_UP §9](PSS_HEADS_UP.md).

---

## SSS_CHAIN confusion

PSS does **not** use SSS_CHAIN. Do not add `sss_chain` to `Cargo.toml`.

---

## Portal navigation

| # | Pillar | Doc |
|---|--------|-----|
| 0 | Security Layers | [PSS_SECURITY_LAYERS.md](PSS_SECURITY_LAYERS.md) |
| 1 | Vision | [PSS_vision.md](PSS_vision.md) |
| 2 | Mathematics | [PSS_mathematics.md](PSS_mathematics.md) |
| 3 | Manual | [PSS_manual.md](PSS_manual.md) |
| 4 | **Troubleshooting** | this document |
| 5 | Use-Cases | [PSS_usecase.md](PSS_usecase.md) |
| 6 | HEADS_UP | [PSS_HEADS_UP.md](PSS_HEADS_UP.md) |

**Also:** [README.md](README.md) · [FORMULAS](PSS_FORMULAS.md)
