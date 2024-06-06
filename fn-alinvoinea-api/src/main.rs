use alinvoinea_graphql::{handle_query_event};
use aws_lambda_events::apigw::ApiGatewayV2httpResponse;
use dotenv::dotenv;
use lambda_runtime::{Error, LambdaEvent, run, service_fn, tracing};
use serde::Deserialize;
use serde_json::Value;
use std::str::FromStr;

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
            handle_query_event(event).await
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
