{}:
let
  sources = import ./sources.nix;
  overlays = [
    (import sources.nixpkgs-mozilla)
    (
      self: super: {
        rustChannel = self.rustChannelOf { channel = "stable"; };
        rustFull = self.rustChannel.rust.override {
          extensions = [
            "clippy-preview"
            "rls-preview"
            "rustfmt-preview"
            "rust-analysis"
            "rust-std"
            "rust-src"
          ];
        };
        cargo = self.rustChannel.rust;
        rustc = self.rustChannel.rust;
      }
    )
    (self: super: { crate2nix = self.callPackage sources.crate2nix {}; })
  ];
  pkgs = import <nixpkgs> { inherit overlays; };
in
pkgs
