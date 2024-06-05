use alinvoinea_secret::get_secret;
use aws_config::SdkConfig;
use aws_sdk_secretsmanager::Client;
use lambda_runtime::Error;
use reqwest::Client as ReqwestClient;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::env;

#[derive(Serialize, Debug)]
pub struct Response {
    pub data: serde_json::Value,
}

#[derive(Deserialize)]
pub struct QueryRequest {
    pub query: String,
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