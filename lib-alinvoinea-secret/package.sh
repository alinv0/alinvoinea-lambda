#!/usr/bin/env bash

rm -rf layer
rm alinvoinea_secret_lib_layer.zip

cargo clean

cargo lambda build --release --arm64  --output-format zip

mkdir -p layer/rust/lib
cp target/aarch64-unknown-linux-gnu/release/libalinvoinea_secret.rlib layer/rust/lib

cd layer || exit
zip -r9 ../alinvoinea_secret_lib_layer.zip .
cd ..