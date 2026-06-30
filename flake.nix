{
  description = "PSS — Plausible Secret Sharing";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: let
    systems = [ "x86_64-linux" "aarch64-linux" ];
    forAll = nixpkgs.lib.genAttrs systems;
  in {
    devShells = forAll (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      default = pkgs.mkShell {
        name = "pss-build-env";
        buildInputs = with pkgs; [ rustc cargo rustfmt clippy musl lean4 ];
        shellHook = ''
          export RUSTFLAGS="-C target-feature=+aes,+ssse3 -C link-arg=-s"
        '';
      };
    });

    packages = forAll (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      default = pkgs.rustPlatform.buildRustPackage {
        pname = "pss";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
        doCheck = true;
      };
    });
  };
}
