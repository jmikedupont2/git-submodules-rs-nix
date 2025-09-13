{
  description = "A flake for testing git submodules";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk"; # Add naersk as an input
    naersk.inputs.nixpkgs.follows = "nixpkgs"; # Ensure naersk uses our nixpkgs
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, naersk }@inputs:
    let
      forAllSystems = flake-utils.lib.eachDefaultSystem (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          # Fetch the entire repository including submodules from remote
          repo = pkgs.fetchgit {
            url = "https://github.com/jmikedupont2/git-submodules-rs-nix.git";
            rev = "b3da08e8e7fe2bb97b97bce5451d6750081256f8"; # Use the specific revision
            deepClone = true; # Ensure submodules are fetched
            sha256 = "clsEx25K+hTb8vXbcSzP39l6bRwjS5X3OBYNaeDm10c=";
          };
        in
        {
          checks = {
            gitoxide = pkgs.runCommand "gitoxide-tests" {
              src = repo; # Pass the specific submodule as source
              buildInputs = [ toolchain ];
              CARGO_HOME = "$TMPDIR/.cargo"; # Set CARGO_HOME to a writable directory
            } ''
              mkdir -p $CARGO_HOME # Create the directory
              cd gitoxide && cargo test
              touch $out
            '';
            submod = pkgs.runCommand "submod-tests" {
              src = repo; # Pass the specific submodule as source
              buildInputs = [ toolchain ];
              CARGO_HOME = "$TMPDIR/.cargo"; # Set CARGO_HOME to a writable directory
            } ''
              mkdir -p $CARGO_HOME # Create the directory
              cd submod && cargo test
              touch $out
            '';
            magoo = pkgs.runCommand "magoo-tests" {
              src = repo; # Pass the specific submodule as source
              buildInputs = [ toolchain ];
              CARGO_HOME = "$TMPDIR/.cargo"; # Set CARGO_HOME to a writable directory
            } ''
              mkdir -p $CARGO_HOME # Create the directory
              cd magoo && cargo test
              touch $out
            '';
          };

          packages = {
            lock-generator = pkgs.stdenv.mkDerivation {
              pname = "lock-generator";
              version = "0.0.0";
              src = ./.; # Source is the entire project root
              buildInputs = [ toolchain pkgs.cargo ]; # Ensure toolchain and cargo are available
              shellHook = ''
                echo "Generating Cargo.lock..."
                cargo generate-lockfile --manifest-path Cargo.toml
                echo "Cargo.lock generated."
              '';
              installPhase = ''
                mkdir -p $out
                cp Cargo.lock $out/Cargo.lock
              '';
            };

            

            submodules-project = naersk.lib.${system}.buildPackage {
              pname = "submodules";
              version = "0.1.0"; # Get this from Cargo.toml
              src = ./.; # Source is the entire project root
              # naersk handles toolchain and Cargo.lock internally
            };

            git-config-parser = pkgs.runCommand "git-config-parser-wrapper" {
              buildInputs = [ pkgs.coreutils ]; # For cp
              submodulesProject = self.packages.${system}.submodules-project;
            } ''
              mkdir -p $out/bin
              cp $submodulesProject/bin/git-config-parser $out/bin/
            '';

                      submodules-managed = pkgs.runCommand "submodules-managed" {
              src = repo; # Use the fetched repository as source
              buildInputs = [ pkgs.coreutils ]; # Ensure cp is available
            } ''
              # Create target directories
              mkdir -p $out/gitoxide
              mkdir -p $out/submod
              mkdir -p $out/magoo
              mkdir -p $out/git-submodule-tools

              # Manually copy submodules
              cp -r $src/gitoxide $out/gitoxide
              cp -r $src/submod $out/submod
              cp -r $src/magoo $out/magoo
              cp -r $src/git-submodule-tools $out/git-submodule-tools
            '';

            submodule-collector = pkgs.stdenv.mkDerivation {
              pname = "submodule-collector";
              version = "0.1.0"; # Matches Cargo.toml
              src = ./.; # Source is the entire project root (workspace root)
              buildInputs = [ toolchain pkgs.cargo pkgs.pkg-config pkgs.openssl ]; # Add pkg-config and openssl
              buildPhase = ''
                export CARGO_HOME=$TMPDIR/.cargo
                echo "Building submodule-collector..."
                cargo build --release --package submodule-collector
              '';
              installPhase = ''
                mkdir -p $out/bin
                cp target/release/submodule-collector $out/bin/
              '';
            };
          };

          devShell = pkgs.mkShell {
            buildInputs = [
              toolchain
              pkgs.git
              pkgs.pkg-config
              pkgs.openssl
              pkgs.cargo # Explicitly add cargo
              pkgs.jq # Add jq here
              pkgs.valgrind # Add valgrind for profiling
              pkgs.emacsPackages.magit
              pkgs.emacsPackages.rustic
              pkgs.emacsPackages.cargo-mode
              pkgs.emacsPackages.rust-mode
              pkgs.emacsPackages.lsp-mode
              pkgs.emacsPackages.company
              pkgs.emacsPackages.flycheck
              pkgs.emacsPackages.lsp-ui
              pkgs.emacsPackages.dap-mode
              pkgs.emacsPackages.tuareg
              #pkgs.emacsPackages.merlin-mode
              #pkgs.emacsPackages.dune-mode
              pkgs.emacsPackages.utop
              pkgs.shellcheck
              pkgs.shfmt
              pkgs.nixpkgs-fmt
            ];
            # Optionally, add environment variables needed for development
            # shellHook = ''
            #   export MY_CUSTOM_VAR="hello"
            # '';
          };
        });
    in
    forAllSystems;
}
