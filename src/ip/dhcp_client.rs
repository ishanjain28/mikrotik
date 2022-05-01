pub use crate::ip::types::DhcpClient;
use crate::{Client, ClientError};
use serde::{Deserialize, Serialize};

/// List all DHCPv4 Clients
pub async fn list(client: &mut Client) -> Result<Vec<DhcpClient>, ClientError> {
    let url = format!("{}/dhcp-client", super::BASE);

    client.execute_get::<Vec<DhcpClient>>(&url).await
}

/// Get a specific DHCPv4 Client
pub async fn get(client: &mut Client, id: &str) -> Result<DhcpClient, ClientError> {
    let url = format!("{}/dhcp-client/{}", super::BASE, id);

    client.execute_get::<DhcpClient>(&url).await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DhcpClientInput {
    pub numbers: String,
}

/// Release a DHCP Lease
pub async fn release(client: &mut Client, input: DhcpClientInput) -> Result<(), ClientError> {
    let url = format!("{}/dhcp-client/release", super::BASE);

    client
        .execute_post_with_no_response::<DhcpClientInput>(&url, input)
        .await?;

    Ok(())
}

/// Renew a DHCP Lease
pub async fn renew(client: &mut Client, input: DhcpClientInput) -> Result<(), ClientError> {
    let url = format!("{}/dhcp-client/renew", super::BASE);

    client
        .execute_post_with_no_response::<DhcpClientInput>(&url, input)
        .await
}

/// Remove a DHCP client
pub async fn remove(client: &mut Client, input: DhcpClientInput) -> Result<(), ClientError> {
    let url = format!("{}/dhcp-client/remove", super::BASE);

    client
        .execute_post_with_no_response::<DhcpClientInput>(&url, input)
        .await
}

/// Disable a DHCP client
pub async fn disable(client: &mut Client, input: DhcpClientInput) -> Result<(), ClientError> {
    let url = format!("{}/dhcp-client/disable", super::BASE);

    client
        .execute_post_with_no_response::<DhcpClientInput>(&url, input)
        .await
}

/// Enable a DHCP client
pub async fn enable(client: &mut Client, input: DhcpClientInput) -> Result<(), ClientError> {
    let url = format!("{}/dhcp-client/enable", super::BASE);

    client
        .execute_post_with_no_response::<DhcpClientInput>(&url, input)
        .await
}

// TODO(ishan): Add reset/set/unset methods
