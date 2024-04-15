{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs =
    { nixpkgs
    , ...
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system
          ;
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs;[
          nil
          cargo
          rustc
          rust-analyzer
          rustfmt
        ];
      };
      formatter.${system} = pkgs.nixpkgs-fmt;
    };
}
