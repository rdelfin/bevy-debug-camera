{
  description = "";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem(system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = pkgs.mkShell rec {
          nativeBuildInputs = [
            pkg-config
            clang
            rust-analyzer
          ];

          buildInputs = [
            (rust-bin.nightly.latest.default.override {
              extensions = [ "rust-src" ];
            })
            systemd alsa-lib vulkan-loader
            # for `x11` feature in bevy
            xorg.libXcursor xorg.libXrandr xorg.libXi
            # for `wayland` feature in bevy
            # libxkbcommon wayland
          ];

          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

          RUST_BACKTRACE = 1;
        };
      }
    );
}