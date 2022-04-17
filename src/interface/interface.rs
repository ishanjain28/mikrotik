use crate::{interface::types::Interface, Client};
use reqwest::{Method, Request};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterfaceError {
    #[error(transparent)]
    UrlError(#[from] url::ParseError),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

pub async fn list(client: &mut Client) -> Result<Vec<Interface>, InterfaceError> {
    let url = client.base_url.clone();
    let url = url.join(super::BASE)?;

    let req = Request::new(Method::GET, url);

    let response = client.execute(req).await?.json::<Vec<Interface>>().await?;

    Ok(response)
}

pub async fn get(client: &mut Client, ifid: &str) -> Result<Interface, InterfaceError> {
    let url = client.base_url.clone();
    let url = url.join(&format!("{}/{}", super::BASE, ifid))?;

    let req = Request::new(Method::GET, url);

    let response = client.execute(req).await?.json::<Interface>().await?;

    Ok(response)
}
