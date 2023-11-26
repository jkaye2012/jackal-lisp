
{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-23.05.tar.gz") {}}:

let
  fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") {};
  python-packages = ps: with ps; [
    mkdocs-material
  ];
in
pkgs.mkShell {
  packages = [
    fenix.stable.toolchain
    pkgs.cargo-tarpaulin
    (pkgs.python3.withPackages python-packages)
  ];
}
