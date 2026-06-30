# PSS Manual

## License: GNU GPLv3 Only
## Target: Operators, integrators, build engineers

**Read first:** [PSS_SECURITY_LAYERS.md](PSS_SECURITY_LAYERS.md) · [README.md](README.md)

---

## Build

```bash
cargo build --release --bin pss
cargo test
cd mathematics && lake build
```

---

## CLI

Path `-` means stdin/stdout where supported.

| Command | Description |
|---------|-------------|
| `pss setup` | `--corpus DIR --output DIR --k K --n N [--min-size BYTES] --file PATH` |
| `pss decode` | `--pool DIR [--mode sum\|combo] [--seed-k K IDX...] [--path-k K IDX...] [--path-len N] [--seed-len N] [--payload FILE] [--payload-len N]` |
| `pss combo-demo` | Synthetic ITS path + sum chain roundtrip |
| `pss verify` | `--pool DIR INDEX...` — k+1 consistency (≥3 indices) |
| `pss capacity` | `--pool DIR --k K --n N [--select-top N]` — \|F*\|\_max, L\_{n−k+1}, % |
| `pss extract` | `--pool DIR [--max-bytes N]` — deterministic pool walk |
| `pss info` | `--pool DIR` — debug offsets/indices (no writes) |
| `pss tier-setup` | `--secret BYTE` — fractal encode demo |
| `pss tier-decode` | alias of tier demo decode path |
| `pss tier-demo` | same as tier-setup/decode demo |

### `--select-top N`

When the pool contains more than n files, consider only the **N largest** files before selecting the top n for capacity math. Requires N ≥ n. Matches `pss_capacity.rs`.

### Examples

```bash
# Capacity on entire pool (uses n largest files)
pss capacity --pool ./iso_pool --k 3 --n 5

# Capacity restricted to 20 largest candidates
pss capacity --pool ./iso_pool --k 3 --n 5 --select-top 20

# Decode with share indices 1,2,3
pss decode --pool ./carriers 1 2 3

# k+1 verify (indices 1,2,3,4 for k=3)
pss verify --pool ./carriers 1 2 3 4

# Extract transposed bytes (extract mode)
pss extract --pool ./carriers --max-bytes 64 > extracted.bin

# Setup: select n untouched files from corpus
pss setup --corpus ./big_pool --output ./selected --k 3 --n 5 --file secret.bin

# OTP payload decrypt after sum decode (seed from indices 1,2,3)
pss decode --pool ./carriers 1 2 3 --payload cipher.bin

# Truncate decrypted payload to 32 bytes
pss decode --pool ./carriers 1 2 3 --payload cipher.bin --payload-len 32

# Combo decode (optional --seed-k cross-checks path vs sum seed)
pss decode --pool ./carriers --mode combo --path-k 1 2 --path-len 5 --seed-k 1 2 1 2 3

# Synthetic combo roundtrip self-test
pss combo-demo
```

---

## Rust library

```rust
use pss::pss_capacity::capacity_report;
use pss::pss_shamir::{split_secret, reconstruct_secret, ShareBundle};
use pss::pss_setup::{setup_from_corpus, decode_from_carriers, SetupConfig};

let sizes = [2u64, 3, 4, 5, 6];
let report = capacity_report(&sizes, 3, 5, None).unwrap();
assert_eq!(report.max_secret_bytes, 4);
```

Crate is `#![no_std]` + `alloc`.

---

## Docker

```bash
docker build -t pss:local .
docker run --rm pss:local help
```

Static musl binary in scratch image.

---

## Nix

```bash
nix-shell --run "cargo build --release --bin pss"
nix build
```

---

## Firecracker

Hermetic decode VM (`firecracker/`):

```bash
cd firecracker
./build-rootfs.sh
# Config: pss-vm.json
```

---

## Man page & completions

```bash
man ./man/pss.1
# Install completions from completions/ (bash, fish, zsh, ps1)
```

---

## Dependencies

`Cargo.toml`: `subtle`, `zeroize`, `sha2` — **no** `sss_chain`.

---

## Portal navigation

| # | Pillar | Doc |
|---|--------|-----|
| 0 | Security Layers | [PSS_SECURITY_LAYERS.md](PSS_SECURITY_LAYERS.md) |
| 1 | Vision | [PSS_vision.md](PSS_vision.md) |
| 2 | Mathematics | [PSS_mathematics.md](PSS_mathematics.md) |
| 3 | **Manual** | this document |
| 4 | Troubleshooting | [PSS_troubleshooting.md](PSS_troubleshooting.md) |
| 5 | Use-Cases | [PSS_usecase.md](PSS_usecase.md) |
| 6 | HEADS_UP | [PSS_HEADS_UP.md](PSS_HEADS_UP.md) |

**Also:** [FORMULAS](PSS_FORMULAS.md) · [FORMAL_VERIFICATION](PSS_FORMAL_VERIFICATION.md)
