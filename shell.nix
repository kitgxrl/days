let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };

in
pkgs.mkShell {
  buildInputs = with pkgs; [
    latest.rustChannels.nightly.rust
    latest.rustChannels.nightly.cargo
    latest.rustChannels.nightly.rust-src
    rustfmt
    rust-analyzer
  ];
}
