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

  outputs =
    {
      fenix,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        toolchain =
          with fenix.packages.${system};
          combine [
            default.toolchain
            targets.wasm32-unknown-unknown.latest.rust-std
          ];
        nativeBuildInputs = [
          toolchain
          pkgs.trunk
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs;
        };
        packages.githubPagesWasmBundle = pkgs.rustPlatform.buildRustPackage {
          inherit nativeBuildInputs;
          pname = "beerio";
          version = "0.0.1";
          src = pkgs.lib.cleanSource ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          buildPhase = "trunk build --release --public-url /handpicked";
          installPhase = ''
            cp dist/index.html dist/404.html
            cp -r dist $out
          '';
        };

      }
    );
}
