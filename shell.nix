with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "my-rust-env";
  buildInputs = [
    pkgs.cargo
    pkgs.gcc
    pkgs.rustup
  ];
}
