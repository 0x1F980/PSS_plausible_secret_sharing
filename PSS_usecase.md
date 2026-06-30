# PSS Use Cases

## License: GNU GPLv3 Only
## Target: Operators planning deployments

**Read first:** [PSS_SECURITY_LAYERS.md](PSS_SECURITY_LAYERS.md) · [PSS_vision.md](PSS_vision.md)

---

## Public ISO carrier pool (32 GB)

Scenario: large directory of public ISO images; hide a 32-byte seed with 3-of-5 Shamir.

```bash
# 1. Check capacity bound
pss capacity --pool /mnt/isos --k 3 --n 5 --select-top 50

# 2. Select n untouched carriers (corpus search)
pss setup --corpus /mnt/isos --output /vault/carriers --k 3 --n 5 --file seed.bin

# 3. Verify disk: exactly 5 files, no sidecars
ls /vault/carriers   # no .key, .pss, manifest

# 4. Recover with any 3 share indices
pss verify --pool /vault/carriers 1 2 3 4
pss decode --pool /vault/carriers 1 4 5
```

Output directory contains **bit-identical** copies of selected ISOs.

---

## Plausible deniability

Carriers look like ordinary files (ISOs, media, logs). No PSS marker on disk. Adversary must identify which n files from a huge corpus were chosen.

---

## Extract mode

When chosen-secret corpus search fails or is not desired:

```bash
pss extract --pool ./carriers --max-bytes 64 > extracted.bin
```

Deterministic read walk — **not** the same as recovering a chosen F* of size L\_{n−k+1}.

---

## Tiered compartmentation

```bash
pss tier-setup --secret 42
pss tier-decode --secret 42
```

Hierarchical k-of-n for organizational separation. **Does not** increase capacity % — see [HEADS_UP](PSS_HEADS_UP.md).

---

## Not suitable for

| Scenario | Why |
|----------|-----|
| Imposing bytes into carriers | Out of scope |
| Hash-only storage | ITS is Shamir, not SHA |
| SSS_CHAIN wire protocols | Separate product; not a dependency |
| Hiding from Eve with all n files | n ≥ k ⇒ reconstruction |

---

## Portal navigation

| # | Pillar | Doc |
|---|--------|-----|
| 0 | Security Layers | [PSS_SECURITY_LAYERS.md](PSS_SECURITY_LAYERS.md) |
| 1 | Vision | [PSS_vision.md](PSS_vision.md) |
| 2 | Mathematics | [PSS_mathematics.md](PSS_mathematics.md) |
| 3 | Manual | [PSS_manual.md](PSS_manual.md) |
| 4 | Troubleshooting | [PSS_troubleshooting.md](PSS_troubleshooting.md) |
| 5 | **Use-Cases** | this document |
| 6 | HEADS_UP | [PSS_HEADS_UP.md](PSS_HEADS_UP.md) |

**Also:** [README.md](README.md) · [FORMULAS](PSS_FORMULAS.md)
