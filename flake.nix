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
          };
      }
    );
}
