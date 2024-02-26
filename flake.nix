{
  description = "cerebro devShell";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = {
    nixpkgs,
    utils,
    fenix,
    ...
  }: let
    overlays = [fenix.overlays.default];

    pkgsForSystem = system:
      import nixpkgs {
        inherit overlays system;
        config.allowUnfree = true;
      };

    toolchain = "stable";
  in
    utils.lib.eachSystem ["x86_64-linux"] (system: let
      legacyPackages = pkgsForSystem system;
      rustPkg = legacyPackages.fenix."${toolchain}".withComponents [
        "cargo"
        "clippy"
        "rust-src"
        "rustc"
        "rustfmt"
      ];
    in rec {
      inherit legacyPackages;

      devShell = with legacyPackages;
        mkShell {
          buildInputs = [
            gdb
            rustPkg
          ];
        };
    });
}
