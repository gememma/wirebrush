{
  description = "Wirebrush";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";

    cargo2nix.url = "github:cargo2nix/cargo2nix/unstable";
    cargo2nix.inputs.flake-utils.follows = "flake-utils";
    cargo2nix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , ...
    } @ inputs:
    let
      pkgsFor = system: import nixpkgs {
        inherit system;
        overlays = [
          inputs.cargo2nix.overlays.default
          inputs.fenix.overlays.default

          (final: prev: {
            rust-toolchain =
              let
                inherit (final.lib.strings) fileContents;

                stableFor = target: target.fromToolchainFile {
                  file = ./rust-toolchain.toml;
                  sha256 = "sha256-eMJethw5ZLrJHmoN2/l0bIyQjoTX1NsvalWSscTixpI=";
                };

                rustfmt = final.fenix.latest.rustfmt;
              in
              final.fenix.combine [
                rustfmt
                (stableFor final.fenix)
              ];
          })

          (final: prev: {
            cargo2nix = inputs.cargo2nix.packages.${system}.default;
          })
        ];
      };

      supportedSystems = with flake-utils.lib.system; [
        aarch64-darwin
        x86_64-darwin
        x86_64-linux
      ];

      inherit (flake-utils.lib) eachSystem;
    in
    eachSystem supportedSystems (system:
    let
      pkgs = pkgsFor system;

      rustPkgs = pkgs.rustBuilder.makePackageSet {
        packageFun = import ./Cargo.nix;
        rustToolchain = pkgs.rust-toolchain;
      };

      inherit (pkgs.lib) optionals;
    in
    rec
    {
      packages = rec {
        default = wirebrush;
        wirebrush = (rustPkgs.workspace.wirebrush { }).bin;

        content = pkgs.stdenv.mkDerivation {
          name = "wirebrush-content";
          src = ./content;

          phases = "installPhase";

          installPhase = ''
            mkdir -p $out
            cp -vrf $src/* $out
          '';
        };

        static = pkgs.stdenv.mkDerivation {
          name = "wirebrush-static";
          src = ./static;

          phases = "installPhase";

          installPhase = ''
            mkdir -p $out
            cp -vrf $src/* $out
          '';
        };
      };

      apps = rec {
        wirebrush = flake-utils.lib.mkApp {
          drv = packages.wirebrush;
        };
        default = wirebrush;
      };

      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          cargo2nix
          nixpkgs-fmt
          rust-toolchain

          libiconv
        ];
      };

      formatter = pkgs.nixpkgs-fmt;
    });
}
