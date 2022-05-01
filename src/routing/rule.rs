use serde::{Deserialize, Serialize};

use crate::{Client, ClientError};

pub use crate::routing::types::Rule;

/// List all routing rules
pub async fn list(client: &mut Client) -> Result<Vec<Rule>, ClientError> {
    let url = format!("{}/rule", super::BASE);

    client.execute_get::<Vec<Rule>>(&url).await
}

/// Get a specific routing rule
pub async fn get(client: &mut Client, id: &str) -> Result<Rule, ClientError> {
    let url = format!("{}/rule/{}", super::BASE, id);

    client.execute_get::<Rule>(&url).await
}

// TODO(ishan): Figure out a smarter way to implement these common operations
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleInput {
    pub numbers: String,
}

/// Disable a rule
pub async fn disable(client: &mut Client, input: RuleInput) -> Result<(), ClientError> {
    let url = format!("{}/rule/disable", super::BASE);

    client
        .execute_post_with_no_response::<RuleInput>(&url, input)
        .await
}

/// Enable a rule
pub async fn enable(client: &mut Client, input: RuleInput) -> Result<(), ClientError> {
    let url = format!("{}/rule/enable", super::BASE);

    client
        .execute_post_with_no_response::<RuleInput>(&url, input)
        .await
}

/// Remove a rule
pub async fn remove(client: &mut Client, input: RuleInput) -> Result<(), ClientError> {
    let url = format!("{}/rule/remove", super::BASE);

    client
        .execute_post_with_no_response::<RuleInput>(&url, input)
        .await
}
