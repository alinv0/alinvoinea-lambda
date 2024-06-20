use std::str::FromStr;

use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use aws_lambda_events::http::HeaderMap;
use dotenv::dotenv;
use lambda_runtime::{Error, LambdaEvent, run, service_fn, tracing};
use serde::Deserialize;
use serde_json::Value;

use crate::route_action::route_action;

mod route_action;

async fn entrypoint(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<ApiGatewayProxyResponse, Error> {
    dotenv().ok();

    //log event
    println!("Received event: {:?}", event);
    let request = event.payload;

    let body: Value = serde_json::from_str(request.body.unwrap_or_default().as_str())?;

    let result = route_action(body).await?;

    let headers = default_headers();

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers,
        multi_value_headers: Default::default(),
        body: Some(Body::Text(result)),
        is_base64_encoded: false,
    })
}

fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Headers", "x-requested-with,invite-code,content-type,X-Amz-Date,X-Amz-Security-Token,Authorization,X-Api-Key,X-Requested-With,Accept,Access-Control-Allow-Methods,Access-Control-Allow-Origin,Access-Control-Allow-Headers".parse().unwrap());
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Methods", "POST,OPTIONS,GET".parse().unwrap());
    headers.insert("X-Requested-With", "*".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(entrypoint)).await
}
