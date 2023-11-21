{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    cargo
    rustc
    rust-analyzer
  ];
  shellHook = with pkgs; with pkgs.xorg; ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${libGL}/lib"
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${libX11}/lib"
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${libXcursor}/lib"
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${libXrandr}/lib"
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${libXi}/lib"
  '';
}
