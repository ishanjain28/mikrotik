pub use crate::interface::types::{
    AddWireguardPeerInput, AddWireguardPeerOutput, WireguardInterface, WireguardPeer,
};
use crate::{Client, ClientError};
use serde::{Deserialize, Serialize};

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

/// list all wireguard peers
pub async fn list_peers(client: &mut Client) -> Result<Vec<WireguardPeer>, ClientError> {
    let url = format!("{}/wireguard/peers", super::BASE);

    client.execute_get::<Vec<WireguardPeer>>(&url).await
}

/// get wireguard peers with the given id
pub async fn get_peer(client: &mut Client, peer_id: &str) -> Result<WireguardPeer, ClientError> {
    let url = format!("{}/wireguard/peers/{}", super::BASE, peer_id);

    client.execute_get::<WireguardPeer>(&url).await
}

/// add a wireguard peer
pub async fn add_peer(
    client: &mut Client,
    peer: AddWireguardPeerInput,
) -> Result<AddWireguardPeerOutput, ClientError> {
    let url = format!("{}/wireguard/peers/add", super::BASE);

    client
        .execute_post::<AddWireguardPeerInput, AddWireguardPeerOutput>(&url, peer)
        .await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RemovePeerInput {
    pub numbers: String,
}

/// remove wireguard peer with given id
pub async fn remove_peer(client: &mut Client, peer_id: &str) -> Result<(), ClientError> {
    let url = format!("{}/wireguard/peers/remove", super::BASE);

    client
        .execute_post_with_no_response::<RemovePeerInput>(
            &url,
            RemovePeerInput {
                numbers: peer_id.to_string(),
            },
        )
        .await
}

#[cfg(test)]
mod test {
    use std::{net::IpAddr, str::FromStr};

    use crate::interface::types::AllowedAddresses;

    use super::*;
    use ipnetwork::IpNetwork;
    use reqwest::Url;

    #[tokio::test]
    async fn list_wireguard_interfaces() -> Result<(), ClientError> {
        let base = Url::parse("https://10.0.10.1")?;
        let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
            .expect("error in creating client");

        let response = crate::interface::wireguard::list(&mut client).await?;

        println!("{:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn get_wireguard_interfaces() -> Result<(), ClientError> {
        let base = Url::parse("https://10.0.10.1")?;
        let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
            .expect("error in creating client");

        let response = crate::interface::wireguard::list(&mut client).await?;

        let response = crate::interface::wireguard::get(&mut client, &response[0].id).await?;

        println!("{:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn list_wireguard_peers() -> Result<(), ClientError> {
        let base = Url::parse("https://10.0.10.1")?;
        let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
            .expect("error in creating client");

        let response = crate::interface::wireguard::list_peers(&mut client).await?;

        println!("{:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn get_wireguard_peers() -> Result<(), ClientError> {
        let base = Url::parse("https://10.0.10.1")?;
        let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
            .expect("error in creating client");

        let response = crate::interface::wireguard::list_peers(&mut client).await?;

        let response = crate::interface::wireguard::get_peer(&mut client, &response[0].id).await?;

        println!("{:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn add_remove_wireguard_peer() -> Result<(), ClientError> {
        let base = Url::parse("https://10.0.10.1")?;
        let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
            .expect("error in creating client");

        let response = crate::interface::wireguard::add_peer(
            &mut client,
            AddWireguardPeerInput {
                allowed_address: AllowedAddresses(vec![
                    IpNetwork::from_str("10.10.0.1/24").unwrap(),
                    IpNetwork::from_str("10.10.0.2").unwrap(),
                ]),
                comment: None,
                disabled: false,
                endpoint_address: None,
                endpoint_port: 0,
                interface: "pia".to_string(),
                persistent_keepalive: Some(10),
                preshared_key: None,
                public_key: "2DCHSS0QYE3a2t5Q5PjXaHXeLZhSeHRvjhdmlYke3Ho=".to_string(),
            },
        )
        .await?;

        println!("{:?}", response);

        crate::interface::wireguard::remove_peer(&mut client, &response.ret).await?;

        Ok(())
    }
}
