{
  description = "Rust Dev Shell using Fenix for WebAssembly";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    fenix,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        toolchain = with fenix.packages.${system};
          combine [
            default.toolchain
            targets.wasm32-unknown-unknown.latest.rust-std
          ];
      in {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            toolchain
            trunk
          ];
        };
      }
    );
}
