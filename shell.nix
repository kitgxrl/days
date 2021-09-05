{ pkgs ? import <nixpkgs> {  } }:

with pkgs;
mkShell {
  name = "days-dev";
  buildInputs = with pkgs; [
    rust-bin.nightly.latest.default
    rust-analyzer
  ];
}
