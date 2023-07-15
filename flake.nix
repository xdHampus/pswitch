{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};

      in rec {
        # For `nix build` & `nix run`:
        defaultPackage = pkgs.callPackage ./default.nix { naersk = naersk'; };

        # For `nix develop`:
        devShell = pkgs.mkShell rec {
          name = "devShell";
          nativeBuildInputs = with pkgs; [ rustc cargo rustfmt ];
        };
      }
    );
}
