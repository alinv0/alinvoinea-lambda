use alinvoinea_secret::get_secret;
use aws_config::SdkConfig;
use aws_sdk_secretsmanager::Client;
use lambda_runtime::Error;
use aws_sdk_secretsmanager::config::BehaviorVersion;
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
    pub variables: String,
}

pub async fn handle_query_event(body: Value) -> Result<String, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;

    let query_payload = body["query"].as_str().unwrap_or_default();
    println!("Query: {}", query_payload);
    let query_payload = if query_payload.is_empty() {
        eprintln!("No query provided!");
        std::process::exit(1);
    } else {
        query_payload
    };

    let variables = body["variables"].as_object().cloned().unwrap_or_else(|| serde_json::Map::new());
    let variables = serde_json::to_string_pretty(&variables)?;
    println!("Variables: {}", variables);

    let query_result = query(
        QueryRequest {
            query: query_payload.to_string(),
            variables,
        }, config).await;
    match query_result {
        Ok(response) => {
            println!("Query response: {:?}", response);
            let response_body = serde_json::to_string(&response.data.get("data"))?;
            println!("Query response body: {:?}", response_body);
            Ok(response_body)
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}

async fn query(request: QueryRequest, config: SdkConfig) -> Result<Response, Error> {
    let graphql_endpoint_key = env::var("GRAPHQL_API_KEY")
        .expect("No endpoint found for query!");
    let graphql_endpoint_result = get_secret(&Client::new(&config), graphql_endpoint_key.as_str()).await;

    match graphql_endpoint_result {
        Ok(endpoint_response) => {
            let endpoint = endpoint_response.secret_value;
            let client = ReqwestClient::new();
            let res = client.post(&endpoint)
                .json(&json!({"query": request.query, "variables": request.variables}))
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
