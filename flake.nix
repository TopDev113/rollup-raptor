{
  description = "noir-client";
  nixConfig = {
    extra-substituters = [
      "https://crane.cachix.org"
      "https://nix-community.cachix.org"
    ];
    extra-trusted-public-keys = [
      "crane.cachix.org-1:8Scfpmn9w+hGdXH/Q9tTLiYAE/2dnJYRJP7kl80GuRk="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs = inputs@{self, flake-parts, ...}:
    flake-parts.lib.mkFlake { inherit inputs; }{
      systems = [ "aarch64-darwin" "x86_64-linux" ];
      perSystem = {config, self', inputs', system, pkgs, lib, ...}:
        let
          toolchain = inputs'.fenix.packages.${system}.fromToolchainFile {
            file = ./client/rust-toolchain.toml;
            sha256 = "";
          };
          rustToolchain = inputs'.fenix.packages.complete.toolchain;
          craneLib = inputs.crane.lib.${system}.overrideToolchain rustToolchain;

          rollupClientAttributes = {
            src = lib.cleanSourceWith {
              src = craneLib.path ./client;
              filter = path: type: craneLib.filterCargoSources path type;
            };
            nativeBuildInputs = with pkgs; [ pkg-config  ];
            buildInputs = with pkgs; [ rustup openssl.dev sqlite ] ++ (lib.optionals (system == "aarch64-darwin") [pkgs.libiconv pkgs.darwin.Security pkgs.darwin.apple_sdk.frameworks.SystemConfiguration]);
          };
        in
          {
            devShells.default = pkgs.mkShell {
              # Rust Analyzer needs to be able to find the path to default crate
              RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
              nativeBuildInputs = [
                rustToolchain
              ]  ++ rollupClientAttributes.nativeBuildInputs ++ rollupClientAttributes.buildInputs;
          };

          packages = {
            noir-client-deps = craneLib.buildDepsOnly (rollupClientAttributes // {
              pname = "noir-client-deps";
            });
            noir-client =
                let noir-client' =
                    craneLib.buildPackage (rollupClientAttributes // {
                    cargoArtifacts = self'.packages.noir-client-deps;
                    meta.mainProgram = "noir-client";
                    });
                in pkgs.runCommand "noir-client-wrapped" {
                    buildInputs = [ pkgs.makeWrapper ];
                    meta.mainProgram = "noir-client";
                }
                ''
                    mkdir -p $out/bin
                    makeWrapper ${noir-client'}/bin/noir-client $out/bin/noir_cli_rollup \
                    --set PATH ${pkgs.lib.makeBinPath []}
                '';

            default = self'.packages.noir-client;

            noir-client-docs = craneLib.cargoDoc (rollupClientAttributes // {
              cargoArtifacts = self'.packages.noir-client-deps;
            });
        };
    };
  };
}







