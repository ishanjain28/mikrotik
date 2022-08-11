pub use crate::ip::types::{DhcpServer, Lease, Network};
use crate::{Client, ClientError};

/// list all dhcp servers
pub async fn list(client: &mut Client) -> Result<Vec<DhcpServer>, ClientError> {
    let url = format!("{}/dhcp-server", super::BASE);

    client.execute_get::<Vec<DhcpServer>>(&url).await
}

/// get details of a specific dhcp server. takes dhcp server name as input
pub async fn get(client: &mut Client, dhcp_server_id: &str) -> Result<DhcpServer, ClientError> {
    let url = format!("{}/dhcp-server/{}", super::BASE, dhcp_server_id);

    client.execute_get::<DhcpServer>(&url).await
}

/// list all networks
pub async fn list_network(client: &mut Client) -> Result<Vec<Network>, ClientError> {
    let url = format!("{}/dhcp-server/network", super::BASE);

    client.execute_get::<Vec<Network>>(&url).await
}

/// get details of a specific network
pub async fn get_network(client: &mut Client, nid: &str) -> Result<Network, ClientError> {
    let url = format!("{}/dhcp-server/network/{}", super::BASE, nid);

    client.execute_get::<Network>(&url).await
}

/// list all dhcp leases from all dhcp servers
pub async fn list_leases(client: &mut Client) -> Result<Vec<Lease>, ClientError> {
    let url = format!("{}/dhcp-server/lease", super::BASE);

    client.execute_get::<Vec<Lease>>(&url).await
}

/// get details of a specific lease
pub async fn get_lease(client: &mut Client, lease_id: &str) -> Result<Lease, ClientError> {
    let url = format!("{}/dhcp-server/lease/{}", super::BASE, lease_id);

    client.execute_get::<Lease>(&url).await
}

#[cfg(test)]
mod test {
    use super::*;
    use reqwest::Url;

    #[tokio::test]
    async fn list_dhcp_servers() -> Result<(), ClientError> {
        let base = Url::parse("https://10.0.10.1")?;
        let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
            .expect("error in creating client");

        let response = crate::ip::dhcp_server::list(&mut client).await?;

        println!("{:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn get_dhcp_server() -> Result<(), ClientError> {
        let base = Url::parse("https://10.0.10.1")?;
        let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
            .expect("error in creating client");
        let response = crate::ip::dhcp_server::get(&mut client, "vlan-150").await?;

        println!("{:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn list_network() -> Result<(), ClientError> {
        let base = Url::parse("https://10.0.10.1")?;
        let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
            .expect("error in creating client");

        let response = crate::ip::dhcp_server::list_network(&mut client).await?;

        println!("{:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn get_network() -> Result<(), ClientError> {
        let base = Url::parse("https://10.0.10.1")?;
        let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
            .expect("error in creating client");

        let response = crate::ip::dhcp_server::list_network(&mut client).await?;

        let response = crate::ip::dhcp_server::get_network(&mut client, &response[0].id).await?;

        println!("{:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn list_leases() -> Result<(), ClientError> {
        let base = Url::parse("https://10.0.10.1")?;
        let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
            .expect("error in creating client");

        let response = crate::ip::dhcp_server::list_leases(&mut client).await?;

        println!("{:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn get_lease() -> Result<(), ClientError> {
        let base = Url::parse("https://10.0.10.1")?;
        let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
            .expect("error in creating client");

        let response = crate::ip::dhcp_server::list_leases(&mut client).await?;

        let response = crate::ip::dhcp_server::get_lease(&mut client, &response[0].id).await?;

        println!("{:?}", response);

        Ok(())
    }
}
