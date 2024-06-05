#!/usr/bin/env bash

./package.sh

# Capture the JSON output of the aws lambda publish-layer-version command
json_output=$(aws lambda publish-layer-version \
--layer-name alinvoinea_graphql_lib \
--zip-file fileb://alinvoinea_graphql_lib_layer.zip)

# Extract the LayerArn value using jq
layer_arn=$(echo "$json_output" | jq -r '.LayerArn')
version=$(echo "$json_output" | jq -r '.Version')

# Append the version to the LayerArn
layer_arn_version="${layer_arn}:${version}"

# Print the LayerArn value
echo "$layer_arn_version"