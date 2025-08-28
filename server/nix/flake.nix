{
  description = "actix backend";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.nightly.latest.complete;
        commonNativeBuildInputs = [ pkgs.pkg-config ];
        commonBuildInputs = [ pkgs.openssl ];
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "picture-perfect-backend";
          src = ./..;
          cargoLock = { lockFile = ../Cargo.lock; };
          nativeBuildInputs = commonNativeBuildInputs;
          buildInputs = commonBuildInputs;
        };
        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = commonNativeBuildInputs;
            buildInputs = with pkgs;
              [ postgresql sqlx-cli sops yq ] ++ [ rust ] ++ commonBuildInputs;
            shellHook = ''
              export PATH=$PATH:$(pwd)/nix/shell
              project_banner.sh
              echo "
              🐚 Rust dev shell ready!
              Run: cargo build / cargo test / etc.
              Available commands:
              - run_db.sh
              - load_db_migration.sh
              - cargo sqlx prepare --check
              "
            '';
          };
        };
      });
}
