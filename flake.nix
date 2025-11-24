{
  description =
    "An ad-hoc macroquad development flake, forked from my bevy development flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config

            libGL

            # X11 dependencies
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr

            # Wayland dependencies
            libxkbcommon
            wayland

            # Development tools
            cargo-watch
          ];

          shellHook = ''
            export LD_LIBRARY_PATH=${
              with pkgs;
              pkgs.lib.makeLibraryPath [
                xorg.libX11
                libxkbcommon
                xorg.libXcursor
                xorg.libXi
                xorg.libXrandr
                libGL
              ]
            }:$LD_LIBRARY_PATH
          '';
        };
      });
}
