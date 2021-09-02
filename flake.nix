{
  description = "Rust Chip-8 emulator";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, utils, ... }:
    utils.lib.eachDefaultSystem (
      system: let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };
      in
        {
          devShell = pkgs.mkShell {
            name = "chippy";
            buildInputs = with pkgs; [
              rust
              rust-analyzer
              cargo-watch
              cargo-edit
            ];

            RUST_BACKTRACE = 1;
          };
        }
    );
}
