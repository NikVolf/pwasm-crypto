#!/bin/bash

cargo build --release --target wasm32-unknown-unknown

wasm-build ../target pwasm_crypto_keccak --public-api=hash --target=wasm32-unknown-unknown --final=keccak --save-raw=../target/keccak-raw.wasm

cp ../target/keccak.wasm ../compiled
cp ../target/keccak-raw.wasm ../compiled

wasm2wat ../compiled/keccak.wasm -o ../compiled/keccak.wat
wasm2wat ../compiled/keccak-raw.wasm -o ../compiled/keccak-raw.wat
