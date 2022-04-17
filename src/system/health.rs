use crate::{system::types::Health, Client};
use color_eyre::Report;
use reqwest::{Method, Request};

pub async fn health(client: &mut Client) -> Result<Vec<Health>, Report> {
    let url = client.base_url.clone();
    let url = url.join(&format!("{}/health", super::BASE))?;

    let req = Request::new(Method::GET, url);

    let response = client.execute(req).await?.json::<Vec<Health>>().await?;

    Ok(response)
}
