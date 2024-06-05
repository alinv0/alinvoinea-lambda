use std::str::FromStr;

use alinvoinea_graphql::{query, QueryRequest};
use aws_config::BehaviorVersion;
use aws_lambda_events::apigw::ApiGatewayV2httpResponse;
use lambda_runtime::{Error, LambdaEvent, run, service_fn, tracing};
use aws_lambda_events::encodings::Body;
use dotenv::dotenv;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub enum LambdaAction {
    Query,
    Mutation,
}

impl FromStr for LambdaAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Query" => Ok(LambdaAction::Query),
            "Mutation" => Ok(LambdaAction::Mutation),
            _ => Err(()),
        }
    }
}

async fn entrypoint(event: LambdaEvent<Value>) -> Result<ApiGatewayV2httpResponse, Error> {
    println!("Starting....");

    dotenv().ok();
    let action = LambdaAction::from_str(
        event.payload["action"].as_str().unwrap_or("")
    ).expect("No action provided!");

    println!("Received action: {:?}", action);

    match action {
        LambdaAction::Query => {
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
        _ => {
            eprintln!("Invalid action");
            std::process::exit(1);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(entrypoint)).await
}
