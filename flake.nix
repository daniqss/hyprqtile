{
  description = "Qtile-like workspaces and monitors management for Hyprland";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
      in {
        devShell = with pkgs;
          mkShell {
            buildInputs = [
              cargo
              cargo-expand
              rust-analyzer
              rustc
              rustfmt
              clippy
            ];

            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "hyprqtile";
          version = "0.1.7";

          src = self;
          cargoLock.lockFile = ./Cargo.lock;

          meta = with pkgs.lib; {
            mainProgram = "hyprqtile";
            homepage = "https://github.com/daniqss/hyprqtile";
            description = "Qtile-like workspaces and monitors management for Hyprland";
            license = licenses.gpl3Only;
            platforms = platforms.linux;
          };
        };
      }
    );
}
