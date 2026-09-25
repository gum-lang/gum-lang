{
  description = "Gum Markup Language — development environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        gum-nix = pkgs.rustPlatform.buildRustPackage {
          pname = "gum-nix";
          version = "0.1.0";
          src = ./nix-generator;
          cargoLock.lockFile = ./nix-generator/Cargo.lock;
        };
      in
      {
        packages.gum-nix = gum-nix;
        apps.gum-nix = {
          type = "app";
          program = "${gum-nix}/bin/gum-nix";
        };

        devShells.default = pkgs.mkShell {
          name = "gum-lang";

          packages = with pkgs; [
            # Task runner
            just

            # Rust toolchain
            rustc
            cargo
            rustfmt
            clippy
            wasm-pack

            # Node.js (TypeScript parser, VSCode extension, website)
            nodejs_22

            # Python (parser-python)
            python312
            python312Packages.pip
            python312Packages.black
            python312Packages.ruff
            python312Packages.pytest
            maturin

            # Go (parser-go)
            go

            # Tree-sitter CLI (tree-sitter-lang grammar development)
            tree-sitter

            # Nix prefetch tools (for gum-nix development)
            nix-prefetch-git
          ];

          shellHook = ''
            echo "Gum Language Ecosystem — dev shell"
            echo "  rustc:    $(rustc --version 2>/dev/null || echo 'not found')"
            echo "  cargo:    $(cargo --version 2>/dev/null || echo 'not found')"
            echo "  node:     $(node --version 2>/dev/null || echo 'not found')"
            echo "  python:   $(python3 --version 2>/dev/null || echo 'not found')"
            echo "  go:       $(go version 2>/dev/null || echo 'not found')"
            echo "  just:     $(just --version 2>/dev/null || echo 'not found')"
            echo ""
            echo "Run 'just' to see available commands."
          '';
        };
      }
    );
}
