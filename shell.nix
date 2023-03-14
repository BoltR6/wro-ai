{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell rec {
  name = "coggers";
  # nativeBuildInputs is usually what you want -- tools you need to run
  nativeBuildInputs = with pkgs; [ 
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    git
    libxkbcommon
    libGL
    cmake
    pkg-config
    fontconfig

    # WINIT_UNIX_BACKEND=wayland
    wayland

    # WINIT_UNIX_BACKEND=x11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xorg.libX11
  ];
  LD_LIBRARY_PATH = "${ pkgs.lib.makeLibraryPath nativeBuildInputs }";
}