{ pkgs ? import <nixpkgs> {} }:
let
  rustupToolchain = "stable";

  rustBuildTargetTriple = "wasm32-unknown-unknown";
  rustBuildHostTriple = "x86_64-unknown-linux-gnu";
in
pkgs.mkShell rec {
  buildInputs = with pkgs; [
    rustup
    clang_9
    wasm-pack
    yarn
    openssl
    wasm-bindgen-cli
    pkg-config
  ];
  # Avoid polluting home dir with local project stuff.
  RUSTUP_HOME = toString ./.rustup;
  CARGO_HOME = toString ./.cargo;

  RUSTUP_TOOLCHAIN = rustupToolchain;

  shellHook = ''
    export PATH=$PATH:${CARGO_HOME}/bin
    export PATH=$PATH:${RUSTUP_HOME}/toolchains/${rustupToolchain}-${rustBuildHostTriple}/bin/
    export CC_wasm32_unknown_unknown="clang-9"
    export CFLAGS_wasm32_unknown_unknown="-I${pkgs.clang_9}/resource-root/include"

    # Ensures our windows target is added via rustup.
    rustup target add "${rustBuildTargetTriple}"
  '';
}
