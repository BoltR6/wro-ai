{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = with pkgs; [ 
        rustc
        cargo
        rustfmt
        rust-analyzer
        clippy
        git
    ];
}