use crate::{
    ip::types::{DhcpServer, Lease, Network},
    Client,
};
use color_eyre::Report;
use reqwest::{Method, Request};

pub async fn list(client: &mut Client) -> Result<Vec<DhcpServer>, Report> {
    let url = client.base_url.clone();
    let url = url.join(&format!("{}/dhcp-server", super::BASE))?;
    let req = Request::new(Method::GET, url);
    let response = client.execute(req).await?.json::<Vec<DhcpServer>>().await?;

    Ok(response)
}

pub async fn get(client: &mut Client, dhcp_server_id: &str) -> Result<DhcpServer, Report> {
    let url = client.base_url.clone();
    let url = url.join(&format!("{}/dhcp-server/{}", super::BASE, dhcp_server_id))?;
    let req = Request::new(Method::GET, url);
    let response = client.execute(req).await?.json::<DhcpServer>().await?;

    Ok(response)
}

pub async fn list_network(client: &mut Client) -> Result<Vec<Network>, Report> {
    let url = client.base_url.clone();
    let url = url.join(&format!("{}/dhcp-server/network", super::BASE))?;
    let req = Request::new(Method::GET, url);
    let response = client.execute(req).await?.json::<Vec<Network>>().await?;

    Ok(response)
}

pub async fn get_network(client: &mut Client, nid: &str) -> Result<Network, Report> {
    let url = client.base_url.clone();
    let url = url.join(&format!("{}/dhcp-server/network/{}", super::BASE, nid))?;
    let req = Request::new(Method::GET, url);
    let response = client.execute(req).await?.json::<Network>().await?;

    Ok(response)
}

pub async fn list_leases(client: &mut Client) -> Result<Vec<Lease>, Report> {
    let url = client.base_url.clone();
    let url = url.join(&format!("{}/dhcp-server/lease", super::BASE))?;
    let req = Request::new(Method::GET, url);
    let response = client.execute(req).await?.json::<Vec<Lease>>().await?;

    Ok(response)
}

pub async fn get_lease(client: &mut Client, lease_id: &str) -> Result<Lease, Report> {
    let url = client.base_url.clone();
    let url = url.join(&format!("{}/dhcp-server/lease/{}", super::BASE, lease_id))?;
    let req = Request::new(Method::GET, url);
    let response = client.execute(req).await?.json::<Lease>().await?;

    Ok(response)
}