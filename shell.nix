{ pkgs ? import <nixpkgs> {  } }:

with pkgs;
mkShell {
  name = "days-dev";
  buildInputs = with pkgs; [
    (rust-bin.nightly.latest.default.override {
      extensions = [ "rust-src" ];
    })
    rust-analyzer
  ];
}
