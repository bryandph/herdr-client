{
  description = "herdr-client — typed Rust client for herdr's socket API";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = {
    self,
    nixpkgs,
  }: let
    inherit (nixpkgs) lib;
    systems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    forAll = f: lib.genAttrs systems (system: f (import nixpkgs {inherit system;}));
  in {
    # devShell carries the exact toolchain regen depends on: cargo-typify pins
    # the codegen so the checked-in src/generated/*.rs stay reproducible, and
    # python3 runs scripts/flatten.py.
    devShells = forAll (pkgs: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          cargo
          rustc
          clippy
          rustfmt
          cargo-typify
          python3
        ];
      };
    });

    # The crate as a package appears once Cargo.lock is committed (kept out of
    # eval until then so the flake always evaluates on a fresh checkout).
    packages = forAll (pkgs:
      lib.optionalAttrs (builtins.pathExists ./Cargo.lock) {
        default = pkgs.rustPlatform.buildRustPackage {
          pname = "herdr-client";
          version = "0.1.0";
          src = self;
          cargoLock.lockFile = ./Cargo.lock;
        };
      });

    formatter = forAll (pkgs: pkgs.alejandra or pkgs.nixpkgs-fmt);
  };
}
