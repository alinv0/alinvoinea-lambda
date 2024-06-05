#!/usr/bin/env bash

rm -rf layer
rm bootstrap.zip
cargo clean

cd ../lib-alinvoinea-secret || exit
layer_secret_arn=$(./deploy.sh)
cd ../lib-alinvoinea-graphql || exitx
layer_graphql_arn=$(./deploy.sh)

cd ../fn-alinvoinea-api || exit

cargo lambda build --release --arm64  --output-format zip