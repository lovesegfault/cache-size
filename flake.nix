{
  description = " A Rust library to quickly get the size and line size of your CPU caches";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, rust, utils, ... }: utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; overlays = [ rust.overlays.default ]; };

      rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

      src = ./.;

      cargoArtifacts = craneLib.buildDepsOnly {
        inherit src;
      };
      fmt = craneLib.cargoFmt { inherit cargoArtifacts src; };
      clippy = craneLib.cargoClippy {
        inherit src;
        cargoArtifacts = fmt;
        cargoClippyExtraArgs = "-- --deny warnings";
      };
      crate = craneLib.buildPackage {
        inherit src;
        cargoArtifacts = clippy;
      };
    in
    {
      checks = { inherit fmt clippy; };
      packages.default = crate;
      devShells.default = with pkgs; mkShell {
        name = "cache-size";
        nativeBuildInputs = [
          rustToolchain
          cargo-edit
          cargo-udeps
          cargo-watch
          rust-analyzer

          nixpkgs-fmt
          nil
          statix

          pre-commit
        ];
        shellHook = ''
          pre-commit install --install-hooks
        '';
      };
    });
}
