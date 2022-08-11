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
struct DhcpClientInput {
    pub numbers: String,
}

/// Release a DHCP Lease
pub async fn release(client: &mut Client, input: &str) -> Result<(), ClientError> {
    let url = format!("{}/dhcp-client/release", super::BASE);

    client
        .execute_post_with_no_response::<DhcpClientInput>(
            &url,
            DhcpClientInput {
                numbers: input.to_string(),
            },
        )
        .await?;

    Ok(())
}

/// Renew a DHCP Lease
pub async fn renew(client: &mut Client, input: &str) -> Result<(), ClientError> {
    let url = format!("{}/dhcp-client/renew", super::BASE);

    client
        .execute_post_with_no_response::<DhcpClientInput>(
            &url,
            DhcpClientInput {
                numbers: input.to_string(),
            },
        )
        .await
}

/// Remove a DHCP client
pub async fn remove(client: &mut Client, input: &str) -> Result<(), ClientError> {
    let url = format!("{}/dhcp-client/remove", super::BASE);

    client
        .execute_post_with_no_response::<DhcpClientInput>(
            &url,
            DhcpClientInput {
                numbers: input.to_string(),
            },
        )
        .await
}

/// Disable a DHCP client
pub async fn disable(client: &mut Client, input: &str) -> Result<(), ClientError> {
    let url = format!("{}/dhcp-client/disable", super::BASE);

    client
        .execute_post_with_no_response::<DhcpClientInput>(
            &url,
            DhcpClientInput {
                numbers: input.to_string(),
            },
        )
        .await
}

/// Enable a DHCP client
pub async fn enable(client: &mut Client, input: &str) -> Result<(), ClientError> {
    let url = format!("{}/dhcp-client/enable", super::BASE);

    client
        .execute_post_with_no_response::<DhcpClientInput>(
            &url,
            DhcpClientInput {
                numbers: input.to_string(),
            },
        )
        .await
}

// TODO(ishan): Add reset/set/unset methods

#[cfg(test)]
mod test {
    use super::*;
    use reqwest::Url;

    #[tokio::test]
    async fn list_dhcp_client() -> Result<(), ClientError> {
        let base = Url::parse("https://10.0.10.1")?;
        let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
            .expect("error in creating client");

        let response = crate::ip::dhcp_client::list(&mut client).await?;

        println!("{:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn get_dhcp_client() -> Result<(), ClientError> {
        let base = Url::parse("https://10.0.10.1")?;
        let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
            .expect("error in creating client");

        let response = crate::ip::dhcp_client::list(&mut client).await?;

        let response = crate::ip::dhcp_client::get(&mut client, &response[0].id).await?;

        println!("{:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn renew_dhcp_lease() -> Result<(), ClientError> {
        let base = Url::parse("https://10.0.10.1")?;
        let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
            .expect("error in creating client");

        let response = crate::ip::dhcp_client::list(&mut client).await?;

        let response = crate::ip::dhcp_client::renew(&mut client, &response[0].id).await?;

        println!("{:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn release_dhcp_lease() -> Result<(), ClientError> {
        let base = Url::parse("https://10.0.10.1")?;
        let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
            .expect("error in creating client");

        let response = crate::ip::dhcp_client::list(&mut client).await?;

        println!("{:?}", response);

        let response = crate::ip::dhcp_client::release(&mut client, &response[0].id).await?;

        println!("{:?}", response);

        Ok(())
    }
}
