use std::fmt;

use aws_sdk_secretsmanager::Client;
use aws_sdk_secretsmanager::config::BehaviorVersion;
use lambda_runtime::{Error};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Debug)]
pub struct Response {
    pub secret_key: String,
    pub secret_value: String,
}

#[derive(Debug, Clone)]
struct SecretNotFoundError;

impl fmt::Display for SecretNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Secret not found")
    }
}

impl std::error::Error for SecretNotFoundError {}


pub async fn get_secret_value(body: Value) -> Result<String, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    let secret_key = body["key"].as_str().unwrap_or_default();
    println!("Secret key: {}", secret_key);

    match get_secret(&client, secret_key).await {
        Ok(response) => {
            println!("Secret response {:?}", response);
            let value = serde_json::to_string(&response)?;
            Ok(value)
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            Err(e)
        }
    }
}

pub async fn get_secret(client: &Client, name: &str) -> Result<Response, Error> {
    let resp = client.get_secret_value().secret_id(name).send().await?;

    let secret_value = match resp.secret_string() {
        Some(value) => value,
        None => return Err(Box::new(SecretNotFoundError)),
    };

    let response = Response {
        secret_key: name.to_string(),
        secret_value: secret_value.to_string(),
    };

    Ok(response)
}