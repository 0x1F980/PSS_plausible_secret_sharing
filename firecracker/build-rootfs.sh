#!/usr/bin/env bash
# Build minimal rootfs with static pss binary for Firecracker decode VM
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${ROOT}/firecracker"
ROOTFS="${OUT}/rootfs.ext4"
MNT="${OUT}/mnt"

cargo build --release --bin pss --target x86_64-unknown-linux-musl 2>/dev/null || \
  cargo build --release --bin pss

mkdir -p "${MNT}/usr/local/bin"
cp "${ROOT}/target/release/pss" "${MNT}/usr/local/bin/pss" 2>/dev/null || \
  cp "${ROOT}/target/x86_64-unknown-linux-musl/release/pss" "${MNT}/usr/local/bin/pss"

# Create sparse ext4 if tools available
if command -v mkfs.ext4 >/dev/null && command -v dd >/dev/null; then
  rm -f "${ROOTFS}"
  dd if=/dev/zero of="${ROOTFS}" bs=1M count=64 status=none
  mkfs.ext4 -F "${ROOTFS}" >/dev/null
  mkdir -p "${MNT}"
  if mount -o loop "${ROOTFS}" "${MNT}" 2>/dev/null; then
    mkdir -p "${MNT}/usr/local/bin"
    cp "${MNT}/usr/local/bin/pss" "${MNT}/usr/local/bin/pss" 2>/dev/null || true
    umount "${MNT}" || true
  fi
fi

echo "Firecracker assets in ${OUT}"
echo "Run: firecracker --config-file ${OUT}/pss-vm.json"
