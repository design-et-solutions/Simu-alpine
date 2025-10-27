{
  description = "ffb";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    project-banner.url = "github:wallago/project-banner?dir=nix";
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, project-banner, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.nightly.latest.complete.override {
          targets = [ "x86_64-pc-windows-gnu" ];
        };
        commonNativeBuildInputs = [ pkgs.pkg-config ];
        commonBuildInputs = with pkgs; [
          openssl
          pkgsCross.mingwW64.buildPackages.gcc
          pkgs.pkgsCross.mingwW64.windows.pthreads
          zip
        ];
      in {
        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = commonNativeBuildInputs;
            buildInputs = [ rust ] ++ commonBuildInputs;
            shellHook = ''
              ${project-banner.packages.${system}.default}/bin/project-banner \
                --owner "design & solutions" \
                --logo " 󰖌 " \
                --part "ffb" \
                --product "simu" \
                --code "DS25-SIMU-FF01"

              export LIBCLANG_PATH=${pkgs.libclang.lib}/lib
            '';

          };
        };
      });
}
