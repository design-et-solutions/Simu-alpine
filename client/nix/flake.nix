{
  description = "simu alpine dioxus frontend";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    project-banner.url = "github:wallago/project-banner?dir=nix";
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, project-banner, ... }:
    let
    in flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config = { allowUnfree = true; };
        };
        commonNativeBuildInputs = with pkgs; [
          wasm-bindgen-cli
          pkg-config
          lld
          xdotool
          zlib
        ];
        dioxus = let
          stdenv = pkgs.stdenv;
          fetchurl = pkgs.fetchurl;
        in import ./dx.nix { inherit stdenv fetchurl; };
        commonBuildInputs = with pkgs; [
          binaryen
          tailwindcss
          xdotool
          zlib
          glib
          gobject-introspection
          atk
          gtk3
          libsoup_3
          webkitgtk_4_1
        ];
        rust = pkgs.rust-bin.nightly.latest.complete.override {
          targets = [ "wasm32-unknown-unknown" ];
        };
      in {
        defaultPackage = pkgs.rustPlatform.buildRustPackage {
          name = "simu-alpine-frontend";
          src = ./..;
          cargoLock = { lockFile = ../Cargo.lock; };
          nativeBuildInputs = commonNativeBuildInputs;
          buildInputs = commonBuildInputs;
          buildPhase = let dx = pkgs.lib.getExe dioxus;
          in ''
            ${dx} build --package web
          '';

          installPhase = ''
            mkdir -p $out
            cp -r target/dx/web/debug/web/public/* $out/
          '';
        };
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = commonNativeBuildInputs;
          buildInputs = with pkgs;
            [ watchman wabt cargo-udeps ] ++ [ rust banner dioxus ]
            ++ commonBuildInputs;
          shellHook = ''
            ${project-banner.packages.${system}.default}/bin/project-banner \
              --owner "design & solutions" \
              --logo " 󰖌 " \
              --part "frontend" \
              --product "simu alpine" \
              --code "DS25-SMAP-FT01" \
              --tips "dx serve"
          '';
        };
      });
}
