#!/usr/bin/env bash

rm -rf layer
rm bootstrap.zip
cargo clean

cd ../lib-alinvoinea-secret || exit
layer_secret_arn=$(./deploy.sh)
cd ../lib-alinvoinea-graphql || exit
layer_graphql_arn=$(./deploy.sh)
cd ../lib-alinvoinea-common || exit
layer_common_arn=$(./deploy.sh)

cd ../fn-alinvoinea-api || exit

cargo build --release --target x86_64-unknown-linux-musl

echo "$layer_secret_arn|$layer_graphql_arn|$layer_common_arn"