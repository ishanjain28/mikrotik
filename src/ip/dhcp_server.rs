use crate::{
    ip::types::{DhcpServer, Lease, Network},
    Client, ClientError,
};

pub async fn list(client: &mut Client) -> Result<Vec<DhcpServer>, ClientError> {
    let url = format!("{}/dhcp-server", super::BASE);

    client.execute_get::<Vec<DhcpServer>>(&url).await
}

pub async fn get(client: &mut Client, dhcp_server_id: &str) -> Result<DhcpServer, ClientError> {
    let url = format!("{}/dhcp-server/{}", super::BASE, dhcp_server_id);

    client.execute_get::<DhcpServer>(&url).await
}

pub async fn list_network(client: &mut Client) -> Result<Vec<Network>, ClientError> {
    let url = format!("{}/dhcp-server/network", super::BASE);

    client.execute_get::<Vec<Network>>(&url).await
}

pub async fn get_network(client: &mut Client, nid: &str) -> Result<Network, ClientError> {
    let url = format!("{}/dhcp-server/network/{}", super::BASE, nid);

    client.execute_get::<Network>(&url).await
}

pub async fn list_leases(client: &mut Client) -> Result<Vec<Lease>, ClientError> {
    let url = format!("{}/dhcp-server/lease", super::BASE);

    client.execute_get::<Vec<Lease>>(&url).await
}

pub async fn get_lease(client: &mut Client, lease_id: &str) -> Result<Lease, ClientError> {
    let url = format!("{}/dhcp-server/lease/{}", super::BASE, lease_id);

    client.execute_get::<Lease>(&url).await
}
