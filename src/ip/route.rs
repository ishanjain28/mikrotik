pub use crate::ip::types::Route;
use crate::{Client, ClientError};
use serde::{Deserialize, Serialize};

/// List all IPv4 Routes
pub async fn list(client: &mut Client) -> Result<Vec<Route>, ClientError> {
    let url = format!("{}/route", super::BASE);

    client.execute_get::<Vec<Route>>(&url).await
}

/// Get a specific IPv4 Route
pub async fn get(client: &mut Client, rid: &str) -> Result<Route, ClientError> {
    let url = format!("{}/route/{}", super::BASE, rid);

    client.execute_get::<Route>(&url).await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteInput {
    pub numbers: String,
}

/// Disable an IPv4 Route
pub async fn disable(client: &mut Client, input: RouteInput) -> Result<(), ClientError> {
    let url = format!("{}/route/disable", super::BASE);

    client
        .execute_post_with_no_response::<RouteInput>(&url, input)
        .await
}

/// Enable an IPv4 Route
pub async fn enable(client: &mut Client, input: RouteInput) -> Result<(), ClientError> {
    let url = format!("{}/route/enable", super::BASE);

    client
        .execute_post_with_no_response::<RouteInput>(&url, input)
        .await
}

/// Remove a IPv4 Route
pub async fn remove(client: &mut Client, input: RouteInput) -> Result<(), ClientError> {
    let url = format!("{}/route/remove", super::BASE);

    client
        .execute_post_with_no_response::<RouteInput>(&url, input)
        .await
}

//TODO(ishan): add set/unset/reset
