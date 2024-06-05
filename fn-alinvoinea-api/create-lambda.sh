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

cp target/lambda/alinvoinea-api/bootstrap.zip .

#Create the IAM role
aws iam create-role --role-name lambda-execution-role --assume-role-policy-document file://trust-policy.json

#Attach Basic Execution Policy
aws iam attach-role-policy --role-name lambda-execution-role --policy-arn arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole

#Create the Lambda function
aws lambda create-function --function-name alinvoinea-api \
  --zip-file fileb://bootstrap.zip --handler bootstrap --runtime provided.al2 \
  --role arn:aws:iam::665063420499:role/lambda-execution-role \
  --layers "$layer_secret_arn" "$layer_graphql_arn" \
  --architectures arm64