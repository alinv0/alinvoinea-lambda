#!/usr/bin/env bash

rm bootstrap.zip
cargo clean

cd ../lib-alinvoinea-secret || exit
layer_secret_arn=$(./deploy.sh)
cd ../lib-alinvoinea-graphql || exitx
layer_graphql_arn=$(./deploy.sh)

cd ../fn-alinvoinea-api || exit

cargo lambda build --release --output-format zip

cp target/lambda/alinvoinea-api/bootstrap.zip .

#Create the Lambda function
aws lambda create-function --function-name alinvoinea-api \
  --zip-file fileb://bootstrap.zip --handler bootstrap --runtime provided.al2 \
  --role arn:aws:iam::665063420499:role/alinvoinea-api-execution-role \
  --layers "arn:aws:lambda:eu-central-1:665063420499:layer:alinvoinea_secret_lib:13" "arn:aws:lambda:eu-central-1:665063420499:layer:alinvoinea_graphql_lib:13" \
  --environment Variables="{GRAPHQL_API_KEY=alinvoinea.eu/hygraph-endpoint}" \
  --architectures x86_64