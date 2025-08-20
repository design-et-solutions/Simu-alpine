{
  description = "client";

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
        rust = pkgs.rust-bin.nightly.latest.default;
        commonNativeBuildInputs = with pkgs; [ pkg-config ];
        commonBuildInputs = with pkgs; [ openssl ];
      in {
        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = commonNativeBuildInputs;
            buildInputs = with pkgs;
              [ rust-analyzer ] ++ [ rust ] ++ commonBuildInputs;
            shellHook = ''
              echo "
              🐚 Rust dev shell ready!
              Run: cargo build / cargo test / etc."
            '';
          };
        };
      });
}
