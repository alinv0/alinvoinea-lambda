use std::fmt;

use aws_sdk_secretsmanager::Client;
use lambda_runtime::Error;
use serde::Serialize;

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