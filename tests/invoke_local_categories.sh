#!/bin/bash

# start the alinvoinea_api function first with cargo lambda watch command
cargo lambda invoke --data-ascii '{
  "httpMethod": "POST",
  "body": "{\"action\":\"Query\",\"query\":\"\\n        query GetCategories {\\n          categories {\\n            name\\n            slug\\n          }\\n        }\\n    \"}",
  "requestContext": {
    "accountId": "665063420499",
    "resourceId": "nlsk93",
    "stage": "Prod",
    "domainName": "vic0e9qoyl.execute-api.eu-central-1.amazonaws.com",
    "domainPrefix": "vic0e9qoyl",
    "requestId": "cbc55d63-f90f-49d5-a3ff-05880e730e0e",
    "protocol": "HTTP/1.1",
    "identity": {
      "cognitoIdentityPoolId": null,
      "accountId": null,
      "cognitoIdentityId": null,
      "caller": null,
      "apiKey": "cjuG6ta0cT83VpfBnOmll21EgHvVMMjT30cV13Rw",
      "apiKeyId": "x9hksd4c9e",
      "accessKey": null,
      "sourceIp": "193.105.140.131",
      "cognitoAuthenticationType": null,
      "cognitoAuthenticationProvider": null,
      "userArn": null,
      "userAgent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36",
      "user": null
    },
    "resourcePath": "/api",
    "path": "/Prod/api",
    "httpMethod": "POST",
    "requestTime": "19/Jun/2024:08:00:23 +0000",
    "requestTimeEpoch": 1718784023198,
    "apiId": "vic0e9qoyl"
  }
}'

