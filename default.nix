{ pkgs }:
let cargo_nix = pkgs.callPackage ./Cargo.nix {};
in cargo_nix.rootCrate.build
