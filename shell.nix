let
  moz_overlay = import (builtins.fetchTarball "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in with nixpkgs;
stdenv.mkDerivation {
  name = "rust-env";
  buildInputs = [ nixpkgs.latest.rustChannels.stable.rust nodejs ];
  # TODO
  buildPhase = ''
    cd test-wasm
    export HOME=$TMP
    npm install
    npm run asbuild
    cd ..
    cargo test
    mkdir $out
  '';
  dontInstall = true;
  dontFixup = true;
}
