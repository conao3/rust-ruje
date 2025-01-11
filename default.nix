{ rustPlatform }:

rustPlatform.buildRustPackage {
  pname = "ruje";
  version = "0.1.0";

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;
}

