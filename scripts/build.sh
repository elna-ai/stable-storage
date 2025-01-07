#!/bin/bash
set -ex
MODULE=$1

canister_root="src/$MODULE"


cargo build --manifest-path="$canister_root/Cargo.toml" \
      --target wasm32-unknown-unknown \
      --release --package $MODULE


candid-extractor "target/wasm32-unknown-unknown/release/$MODULE.wasm" > "$canister_root/$MODULE.did"

# candid-extractor "target/wasm32-wasip1/release/$MODULE.wasm" > "src/$MODULE/$MODULE.did"%