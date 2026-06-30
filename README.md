# PSS: Plausible Secret Sharing

## GNU General Public License v3.0 Only
Copyright (C) 2026 0x1F980. All rights reserved.

PSS is free software under **GPL-3.0-only** (not later versions). See [LICENSE](LICENSE) and [RELICENSING.md](RELICENSING.md).

## Target: Cryptographers, Security Auditors, Operators

**ITS = Information-Theoretic Secrecy** — security against `< k` share leakage holds against unbounded computation (Shamir over GF(256)).

**Read first:** **[PSS_SECURITY_LAYERS.md](PSS_SECURITY_LAYERS.md)** — ITS vs hash-plumbing vs camouflage.

> **WARNING FOR REVIEWERS:** SHA-256 in PSS is used for public file identity and deterministic offset derivation only. Security against `< k` share leakage is **information-theoretic (Shamir)**, not computational hash hardness. Do not classify PSS as a hash-based secret storage scheme.

Standalone `#![no_std]` Rust crate: Shamir k-of-n + Lagrange in **GF(256)** (addition = XOR), read-only **transposition** in **untouched** carrier files. No `.key`, `.pss`, manifest, or sidecars on disk — exactly **n** unchanged files after setup.

**Not SSS_CHAIN:** same Shamir algebra; opposite **deployment** (untouched files vs noise shares). `sss_chain` is **not** a dependency.

```bash
git clone git@github.com:0x1F980/PSS_plausible_secret_sharing.git
cd PSS_plausible_secret_sharing
cargo test
cargo build --release --bin pss
pss capacity --pool ./carriers --k 3 --n 5
nix-shell --run "cargo build --release --bin pss"
docker build -t pss:local .
```

---

## The 7-Pillar Documentation Architecture

```
                  +----------------------------------------------+
                  |                  README.md                   |
                  |                (This Portal)                 |
                  +----------------------+-----------------------+
                                         |
                  +----------------------v-----------------------+
                  |      PSS_SECURITY_LAYERS.md (#0)              |
                  +----------------------+-----------------------+
                                         |
         +-------------------------------+-------------------------------+
         |                               |                               |
+--------v--------+             +--------v--------+             +--------v--------+
|    Vision       |             |   Mathematics   |             |     Manual      |
|  (Eve model,    |             | (Shamir ITS,    |             | (CLI, Docker,   |
|   vs SSS)       |             |  L_{n-k+1})     |             |  Nix, Firecracker)|
+--------+--------+             +--------+--------+             +--------+--------+
         |                               |                               |
         +-------------------------------+-------------------------------+
         |                               |                               |
+--------v--------+             +--------v--------+             +--------v--------+
| Troubleshooting |             |    Use-Cases    |             |    HEADS_UP     |
+-----------------+             +-----------------+             +-----------------+
```

0. **[Security Layers](PSS_SECURITY_LAYERS.md)** — ITS vs hash vs camouflage; reviewer WARNING.
1. **[Vision](PSS_vision.md)** — Purpose, Eve model, deployment vs SSS.
2. **[Mathematics](PSS_mathematics.md)** — Shamir ITS, capacity, anti-hash section.
3. **[Manual](PSS_manual.md)** — CLI, Docker, Nix, Firecracker, shells.
4. **[Troubleshooting](PSS_troubleshooting.md)** — corpus fail, false match, metadata.
5. **[Use-Cases](PSS_usecase.md)** — public ISO pool, 32 GB carriers.
6. **[HEADS_UP](PSS_HEADS_UP.md)** — theory vs practice, impose out of scope.

**Also:** [FORMULAS](PSS_FORMULAS.md) · [FORMAL_VERIFICATION](PSS_FORMAL_VERIFICATION.md) · [PROOF_MANIFEST](PROOF_MANIFEST.md)

---

## Quick CLI

```bash
pss capacity --pool ./carriers --k 3 --n 5
pss capacity --pool ./big_pool --k 3 --n 5 --select-top 20
pss decode --pool ./carriers 1 2 3
pss verify --pool ./carriers 1 2 3 4
echo -n "secret" | pss setup --corpus ./pool --output ./out --k 3 --n 5 --file -
```

See `man/pss.1` and shell completions in `completions/` (bash, zsh, fish, PowerShell).

---

## Quick API

```rust
use pss::pss_capacity::capacity_report;
use pss::pss_shamir::{split_secret, reconstruct_secret, ShareBundle};

let sizes = [2u64, 3, 4, 5, 6];
let report = capacity_report(&sizes, 3, 5, None).unwrap();
assert_eq!(report.max_secret_bytes, 4);
```

---

## vs SSS_CHAIN (ecosystem contrast)

| | SSS_CHAIN | PSS (this repo) |
|---|-----------|-----------------|
| Role | Shared chaining algebra for ITS ecosystem | Standalone plausible carriers |
| Deployment | Synthetic link bytes | Untouched real files |
| Dependency | Consumer crates depend on it | **No** `sss_chain` in Cargo.toml |
| ITS mechanism | Shamir / backward ambiguity | Shamir k-of-n GF(256) |

---

## Quick reference

| Concept | Value |
|---------|-------|
| Field | GF(256), add = XOR |
| Threshold | k-of-n Shamir |
| Max secret (theory) | \|F*\|\_max = L\_{n−k+1} |
| Disk after setup | exactly n untouched files |
| Hashes | public plumbing only |
| SSS_CHAIN | **not used** |

---

## Build & test

```bash
cargo test
cargo build --release --bin pss
lake build    # formal sketches in mathematics/
```

Dependencies: `subtle`, `zeroize`, `sha2` only — **no** `sss_chain`.
