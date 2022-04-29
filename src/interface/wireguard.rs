use crate::{interface::types::WireguardInterface, Client, ClientError};

/// list all wireguard interfaces
pub async fn list(client: &mut Client) -> Result<Vec<WireguardInterface>, ClientError> {
    let url = format!("{}/wireguard", super::BASE);

    client.execute_get::<Vec<WireguardInterface>>(&url).await
}

/// get wireguard interface with the provided id
pub async fn get(client: &mut Client, wg_id: &str) -> Result<WireguardInterface, ClientError> {
    let url = format!("{}/wireguard/{}", super::BASE, wg_id);

    client.execute_get::<WireguardInterface>(&url).await
}
