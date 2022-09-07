#!/bin/bash
set -e
cd "`dirname $0`"
echo ">> Building contract"
rustup target add wasm32-unknown-unknown
RUSTFLAGS='-C link-arg=-s' cargo build --all --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/contract.wasm ./out/nearlott.wasm

