{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      perSystem = { config, self', pkgs, lib, system, ... }:
        let
         
          runtimeDeps = [ ];
          buildDeps = with pkgs; [ openssl pkg-config tailwindcss ];
          devDeps = with pkgs; [ 
            gdb 
            clang
            rust-analyzer 
            openssl
            sea-orm-cli
            mold-wrapped
          ];

          cargoToml = builtins.fromTOML (builtins.readFile ./backend/Cargo.toml);
          msrv = cargoToml.package.rust-version;

          rustPackage = features:
            (pkgs.makeRustPlatform {
              cargo = pkgs.rust-bin.stable.latest.minimal;
              rustc = pkgs.rust-bin.stable.latest.minimal;
            }).buildRustPackage {
              inherit (cargoToml.package) name version;
              src = ./.;
              cargoLock = {
                lockFile = ./Cargo.lock;
                allowBuiltinFetchGit = true;
              };
              buildFeatures = features;
              buildInputs = runtimeDeps;
              nativeBuildInputs = buildDeps;
              # Uncomment if your cargo tests require networking or otherwise
              # don't play nicely with the Nix build sandbox:
              doCheck = false;
            };

          mkDevShell = rustc:
            pkgs.mkShell {
              shellHook = ''
                export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
              '';
              buildInputs = runtimeDeps;
              nativeBuildInputs = buildDeps ++ devDeps ++ [ rustc ];
            };
        in {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import inputs.rust-overlay) ];
          };

          devShells.default = self'.devShells.stable;
          devShells.nightly = (mkDevShell (pkgs.rust-bin.selectLatestNightlyWith
            (toolchain: toolchain.default)));
          devShells.stable = (mkDevShell (pkgs.rust-bin.stable.latest.default));
          devShells.msrv = (mkDevShell pkgs.rust-bin.stable.${msrv}.default);
        };
    };
}

