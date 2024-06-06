#!/usr/bin/env bash

rm -rf layer
rm alinvoinea_graphql_lib_layer.zip

cargo clean

cargo lambda build --release  --output-format zip

mkdir -p layer/rust/lib
cp target/x86_64-unknown-linux-gnu/release/libalinvoinea_graphql.rlib layer/rust/lib

cd layer || exit
zip -r9 ../alinvoinea_graphql_lib_layer.zip .
cd ..