{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
        {
          packages.default = pkgs.buildEnv {
            name = "rivulet-fd";
            buildInputs = [
              pkgs.fd
            ];
            paths = [];
          };
        }
    );
}
