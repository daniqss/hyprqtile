{
  description = "Qtile-like workspaces and monitors management for Hyprland";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
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
        rustPlatform = pkgs.rustPlatform;
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            cargo-expand
            rust-analyzer
            rustc
            rustfmt
            clippy
          ];
        };

        packages.default = rustPlatform.buildRustPackage {
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
