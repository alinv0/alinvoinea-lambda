use alinvoinea_secret::get_secret;
use aws_config::{BehaviorVersion, SdkConfig};
use aws_lambda_events::apigw::ApiGatewayV2httpResponse;
use aws_lambda_events::encodings::Body;
use aws_sdk_secretsmanager::Client;
use lambda_runtime::{Error, LambdaEvent};
use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;

#[derive(Serialize, Debug)]
pub struct Response {
    pub data: serde_json::Value,
}

#[derive(Deserialize)]
pub struct QueryRequest {
    pub query: String,
}

pub async fn handle_query_event(event: LambdaEvent<Value>) -> Result<ApiGatewayV2httpResponse, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let query_payload = match &event.payload["query"] {
        Value::String(s) => s.clone(),
        _ => panic!("No query provided!"),
    };
    let query_result = query(
        QueryRequest { query: query_payload }, config).await;
    match query_result {
        Ok(response) => {
            println!("Query response {:?}", response);
            let response_body = serde_json::to_string(&response.data)?;
            Ok(ApiGatewayV2httpResponse {
                status_code: 200,
                headers: Default::default(),
                multi_value_headers: Default::default(),
                body: Option::from(Body::Text(response_body)),
                is_base64_encoded: false,
                cookies: vec![],
            })
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}

pub async fn query(request: QueryRequest, config: SdkConfig) -> Result<Response, Error> {
    let graphql_endpoint_key = env::var("GRAPHQL_API_KEY")
        .expect("No endpoint found for query!");
    // println!("GRAPHQL_API_KEY: {}", graphql_endpoint_key);
    let graphql_endpoint_result = get_secret(&Client::new(&config), graphql_endpoint_key.as_str()).await;

    match graphql_endpoint_result {
        Ok(endpoint_response) => {
            let endpoint = endpoint_response.secret_value;
            // println!("GRAPHQL_API: {}", &endpoint);
            let client = ReqwestClient::new();
            let res = client.post(&endpoint)
                .json(&json!({"query": request.query}))
                .send().await?
                .json::<serde_json::Value>().await?;

            Ok(Response { data: res })
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            Err(e)
        }
    }
}