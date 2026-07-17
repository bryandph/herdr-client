{
  description = "herdr-client — typed Rust client for herdr's socket API";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    # herdr provides `herdr api schema`, the source of the pinned schema. This
    # input IS the herdr-version pin: `nix flake update herdr` + `.#refresh-schema`
    # + scripts/regen.sh is how the client tracks a new herdr release. herdr keeps
    # its own nixpkgs (don't follow — its build is tested against it).
    herdr.url = "github:ogulcancelik/herdr";
  };

  outputs = {
    self,
    nixpkgs,
    herdr,
  }: let
    inherit (nixpkgs) lib;
    systems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    forAll = f: lib.genAttrs systems (system: f (import nixpkgs {inherit system;}));
  in {
    # devShell carries the exact toolchain regen depends on: cargo-typify pins
    # the codegen so the checked-in src/generated/*.rs stay reproducible, and
    # python3 runs scripts/flatten.py. (herdr is deliberately NOT here — building
    # it is heavy and only the schema refresh needs it; use `.#refresh-schema`.)
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

    # `nix run .#refresh-schema` dumps the flake-pinned herdr's socket schema to
    # schema/herdr-api.schema.json. With scripts/regen.sh this regenerates the
    # client for whatever herdr the flake input points at — so flake.lock is the
    # single source of the herdr-version pin, and a bump surfaces as a reviewable
    # schema + generated-code diff.
    apps = forAll (pkgs: {
      refresh-schema = {
        type = "app";
        program = lib.getExe (pkgs.writeShellApplication {
          name = "refresh-schema";
          runtimeInputs = [herdr.packages.${pkgs.system}.default];
          text = ''
            out="''${1:-schema/herdr-api.schema.json}"
            herdr api schema --output "$out"
            echo "refreshed $out from herdr $(herdr --version)"
          '';
        });
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

    # The herdr version this client is currently generated against — drives
    # release tagging (`nix eval --raw .#herdrVersion.<system>`). Eval-only, no
    # herdr build required.
    herdrVersion = forAll (pkgs: herdr.packages.${pkgs.system}.default.version);

    formatter = forAll (pkgs: pkgs.alejandra or pkgs.nixpkgs-fmt);
  };
}
