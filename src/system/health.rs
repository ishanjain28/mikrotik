use crate::{system::types::Health, Client};
use reqwest::{Method, Request};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HealthError {
    #[error(transparent)]
    UrlError(#[from] url::ParseError),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

pub async fn health(client: &mut Client) -> Result<Vec<Health>, HealthError> {
    let url = client.base_url.clone();
    let url = url.join(&format!("{}/health", super::BASE))?;

    let req = Request::new(Method::GET, url);

    let response = client.execute(req).await?.json::<Vec<Health>>().await?;

    Ok(response)
}
