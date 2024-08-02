# alinvoinea-lambda

This repository contains the source code for the serverless backend of my website (alinvoinea.eu).

## Write lambda functions - tips
use musl

give +x to binary 

copy binary to zip with -j 

create role for function

add permission to role

add permission to function for SSM

works only for architectures x86_64


## How to build
#!/usr/bin/env bash

#### Define variables
AWS_ACCOUNT_ID="665063420499"
ROLE_NAME="alinvoinea-api-execution-role"
SECRETS_POLICY_NAME="SecretsManagerAccessPolicy"
KMS_POLICY_NAME="KMSDecryptPolicy"
SECRETS_POLICY_ARN="arn:aws:iam::$AWS_ACCOUNT_ID:policy/$SECRETS_POLICY_NAME"
KMS_POLICY_ARN="arn:aws:iam::$AWS_ACCOUNT_ID:policy/$KMS_POLICY_NAME"
FUNCTION_NAME="alinvoinea-api"

rm bootstrap.zip
cargo clean

cd ../lib-alinvoinea-secret || exit
layer_secret_arn=$(./deploy.sh)
cd ../lib-alinvoinea-graphql || exitx
layer_graphql_arn=$(./deploy.sh)

cd ../fn-alinvoinea-api || exit

cargo zigbuild --release --target x86_64-unknown-linux-musl

mv target/x86_64-unknown-linux-musl/release/alinvoinea-api target/x86_64-unknown-linux-musl/release/bootstrap
chmod +x target/x86_64-unknown-linux-musl/release/bootstrap
zip -j bootstrap.zip target/x86_64-unknown-linux-musl/release/bootstrap

# Create the IAM role
aws iam create-role --role-name alinvoinea-api-execution-role --assume-role-policy-document file://trust-policy.json
#Attach Basic Execution Policy
aws iam attach-role-policy --role-name alinvoinea-api-execution-role --policy-arn arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole
# Create the $SECRETS_POLICY_NAME policy and attach it to the Lambda execution role
aws iam create-policy --policy-name $SECRETS_POLICY_NAME --policy-document file://secrets-manager-policy.json
aws iam attach-role-policy --role-name alinvoinea-api-execution-role --policy-arn arn:aws:iam::$AWS_ACCOUNT_ID:policy/$SECRETS_POLICY_NAME

#Create the Lambda function
aws lambda create-function --function-name $FUNCTION_NAME \
  --zip-file fileb://bootstrap.zip --handler bootstrap --runtime provided.al2 \
  --role arn:aws:iam::$AWS_ACCOUNT_ID:role/alinvoinea-api-execution-role \
  --layers "arn:aws:lambda:eu-central-1:665063420499:layer:alinvoinea_secret_lib:13" "arn:aws:lambda:eu-central-1:665063420499:layer:alinvoinea_graphql_lib:13" \
  --environment Variables="{GRAPHQL_API_KEY=alinvoinea.eu/hygraph-endpoint}" \
  --architectures x86_64

aws lambda add-permission --function-name $FUNCTION_NAME \
  --statement-id SecretsManagerPolicy_GetPolicy --action "lambda:GetPolicy" \
  --principal secretsmanager.amazonaws.com \
  --source-arn arn:aws:lambda:eu-central-1:$AWS_ACCOUNT_ID:function:$FUNCTION_NAME \
  --source-account $AWS_ACCOUNT_ID \
  --resource arn:aws:lambda:eu-central-1:$AWS_ACCOUNT_ID:function:$FUNCTION_NAME

## How to deploy to AWS

## Diagram


API -> GraphQL Lib -> Get Secret Lib
