# shell.nix - Hermetic Dev Shell for PSS
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "pss-build-env";
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    clippy
    musl
    lean4
  ];

  shellHook = ''
    export RUSTFLAGS="-C target-feature=+aes,+ssse3 -C link-arg=-s"
    echo "Hermetic PSS Nix Environment Loaded!"
  '';
}
