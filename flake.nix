{
  description = "Rust Dev Shell using Fenix for WebAssembly";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/24.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, fenix, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          toolchain = with fenix.packages.${system}; combine [
            default.toolchain
            targets.wasm32-unknown-unknown.latest.rust-std
            rust-analyzer
          ];
        in
        {
          devShells.default =
            pkgs.mkShell {
              packages = with pkgs; [
                toolchain
                trunk
              ];

            };
        }
      );
}

