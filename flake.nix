{
  description = "A CLI tool to show your GitHub contributions";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      craneLib = crane.mkLib pkgs;
      commonArgs = {
        src = craneLib.cleanCargoSource ./.;
        strictDeps = true;

        nativeBuildInputs = [pkgs.pkg-config];
        buildInputs = [pkgs.openssl];
      };

      dono = craneLib.buildPackage (commonArgs
        // {
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        });
    in {
      checks = {
        inherit dono;
      };

      packages = {
        default = dono;
      };

      apps.default = flake-utils.lib.mkApp {
        drv = dono;
      };

      devShells.default = craneLib.devShell {
        RUST_LOG = "info";
        packages = [];
      };
    });
}
