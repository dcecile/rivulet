{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    curl.url = "path:./curl";
    fd.url = "path:./fd";
    overmind.url = "path:./overmind";
    task.url = "path:./task";
    watchman.url = "path:./watchman";
  };

  outputs = { nixpkgs, flake-utils, curl, fd, overmind, task, watchman, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        lib = pkgs.stdenv.lib;
      in
        {
          devShells.default = pkgs.mkShellNoCC {
            inputsFrom = [
              curl.packages.${system}.default
              fd.packages.${system}.default
              overmind.packages.${system}.default
              task.packages.${system}.default
              watchman.packages.${system}.default
            ];

            nativeBuildInputs = [
            ];
            
            packages = [
            ];

            shellHook = ''
              curl --version | head -n 1
              fd --version
              ghc --version
              overmind --version
              task --version
              echo "watchman $(watchman --version)"
            '';
          };
        }
    );
}
