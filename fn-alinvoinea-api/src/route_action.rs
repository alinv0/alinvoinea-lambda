use std::str::FromStr;
use alinvoinea_common::not_yet_implemented;
use alinvoinea_graphql::handle_query_event;
use alinvoinea_secret::get_secret_value;
use lambda_runtime::Error;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub enum LambdaAction {
    GetSecret,
    Query,
    Mutation,
}

impl FromStr for LambdaAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Query" => Ok(LambdaAction::Query),
            "Mutation" => Ok(LambdaAction::Mutation),
            "GetSecret" => Ok(LambdaAction::GetSecret),
            _ => Err(()),
        }
    }
}


pub async fn route_action(body: Value) -> Result<String, Error> {
    let action = LambdaAction::from_str(
        body["action"].as_str().unwrap_or("")
    ).expect("No action provided!");
    println!("Action: {:?}", action);
    let result = match action {
        LambdaAction::Query => {
            handle_query_event(body).await
        }
        LambdaAction::Mutation => {
            // Return a Result here
            Ok(not_yet_implemented())
        }
        LambdaAction::GetSecret => {
            get_secret_value(body).await
        }
    };
    result
}