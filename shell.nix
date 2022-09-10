{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    openssl
    openssl.dev
    binutils-unwrapped-all-targets
    pkg-config
    rustup
  ];
}
