{ pkgs }:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "mdbook-${version}";
  version = "0.4.2";
  src = ./.;
  cargoSha256 = "0ch25ivww62j2kwsd20159hwjfyhfarwmqy9hqn1lv2vhfskry2b";
}
