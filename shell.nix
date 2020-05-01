let
  pkgs = import ./nix {};
in
pkgs.mkShell {
  name = "cache-size";
  nativeBuildInputs = with pkgs; [
    cargo

    rust-analyzer
    cargo-edit

    niv
    nixpkgs-fmt
  ];
}
