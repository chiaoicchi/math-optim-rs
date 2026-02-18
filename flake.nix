{
  description = "math-optim-rs environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        rustToolchain =
          (fenix.packages.${system}.toolchainOf {
            channel = "1.89.0";
            sha256 = "sha256-+9FmLhAOezBZCOziO0Qct1NOrfpjNsXxc/8I0c7BdKE=";
          }).withComponents
            [
              "cargo"
              "clippy"
              "rust-src"
              "rust-analyzer"
              "rustc"
              "rustfmt"
            ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
          ];

          RUST_BACKTRACE = "1";

          shellHook = ''
            echo "math-optim-rs Environment (fenix)"
            echo "  Rust $(rustc --version)"
          '';
        };
      }
    );
}
