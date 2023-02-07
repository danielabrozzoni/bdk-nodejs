# BDK + nodejs = ❤️

This repository shows how to use the bdk library in nodejs. It's just a proof-of-concept, not a complete example, and as such, it's pretty limited: it only supports generating a mnemonic, creating a BIP86 descriptor from it, printing the first address, syncing, and printing the total balance. (If this doesn't seem limited to you, it just means you don't know how powerful bdk is! 😇)

## Setup

You'll need `wasm-pack` and `wasm-bindgen` installed.

To build the project:

```
wasm-pack build --target nodejs && wasm-bindgen --target nodejs --out-dir target/wasm32-unknown-unknown/release/ target/wasm32-unknown-unknown/release/nodejs_example.wasm
```

To run the example:

```
node bdk.js
```

## Limitations

At the moment bdk wasm only supports the memory database and the esplora backend. The upcoming [1.0 release](https://bitcoindevkit.org/blog/road-to-bdk-1/) will hopefully change that, so stay tuned!

Wasm optimization is disabled (it didn't work on my machine™️). You can re-enable it by removing the last two lines in the `Cargo.toml` (`wasm-opt = false`).
