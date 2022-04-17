use super::types::*;
use crate::Client;
use color_eyre::Report;
use reqwest::{Method, Request};
use tracing::debug;

pub async fn list(client: &mut Client) -> Result<Vec<Interface>, Report> {
    let url = client.base_url.clone();
    let url = url.join(super::BASE)?;

    let req = Request::new(Method::GET, url);

    let response = client.execute(req).await?.json::<Vec<Interface>>().await?;

    Ok(response)
}

pub async fn get(client: &mut Client, ifid: &str) -> Result<Interface, Report> {
    let url = client.base_url.clone();
    let url = url.join(&format!("{}/{}", super::BASE, ifid))?;
    debug!("url {}", url);

    let req = Request::new(Method::GET, url);

    let response = client.execute(req).await?.json::<Interface>().await?;

    Ok(response)
}
