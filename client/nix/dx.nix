{ stdenv, fetchurl }:
stdenv.mkDerivation {
  pname = "dioxus-cli";
  version = "0.7.0-rc.0";
  src = fetchurl {
    url =
      "https://github.com/DioxusLabs/dioxus/releases/download/v0.7.0-rc.0/dx-x86_64-unknown-linux-gnu.tar.gz";
    sha256 = "sha256-eo7QSKg6f/jD+FPjAyumaebLrgio3DskCJnZavODoD0=";
  };

  unpackPhase = ''
    mkdir source
    cd source
    tar -xvf $src
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp dx $out/bin/
  '';
}
