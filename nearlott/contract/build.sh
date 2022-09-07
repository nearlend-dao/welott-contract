#!/bin/bash
set -e
echo ">> Building contract"
rustup target add wasm32-unknown-unknown
cargo build --all --target wasm32-unknown-unknown --release
echo ">> Building contract"
cp ../target/wasm32-unknown-unknown/release/*.wasm ../out

