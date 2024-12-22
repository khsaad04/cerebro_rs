{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  packages = [
    pkgs.cargo
    pkgs.clippy
    pkgs.rustc
    pkgs.rustfmt
    pkgs.rust-analyzer
    pkgs.openssl
    pkgs.pkg-config
  ];
}
