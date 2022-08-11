use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method, Request, Url,
};
use serde::{de::DeserializeOwned, ser::Serialize};
use std::{fmt::Display, num::ParseFloatError};
use thiserror::Error;

pub struct Client {
    pub client: reqwest::Client,
    base_url: Url,
    basic_auth: HeaderValue,
}

impl Client {
    pub fn new(
        base_url: Url,
        username: String,
        password: String,
        self_signed_cert: bool,
    ) -> Result<Self, ClientError> {
        let value = format!("{}:{}", username, password);
        let value = base64::encode(value);

        let basic_auth = HeaderValue::from_str(&format!("Basic {}", value))
            .expect("invalid value for Authorization header");

        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(self_signed_cert)
            .build()?;

        Ok(Self {
            client,
            base_url,
            basic_auth,
        })
    }

    pub fn add_credentials(&self, m: &mut HeaderMap<HeaderValue>) {
        let token = m
            .entry("Authorization")
            .or_insert_with(|| self.basic_auth.clone());

        *token = self.basic_auth.clone();
    }

    pub async fn execute(&mut self, mut r: Request) -> Result<reqwest::Response, reqwest::Error> {
        self.add_credentials(r.headers_mut());

        self.client.execute(r).await
    }

    pub async fn execute_get<T: DeserializeOwned>(&mut self, url: &str) -> Result<T, ClientError> {
        let url = self.base_url.join(url)?;
        let mut req = Request::new(Method::GET, url);
        self.add_credentials(req.headers_mut());

        let response = self.client.execute(req).await?.json::<T>().await?;

        Ok(response)
    }

    pub async fn execute_post<P: Serialize, Q: DeserializeOwned>(
        &mut self,
        url: &str,
        payload: P,
    ) -> Result<Q, ClientError> {
        let url = self.base_url.join(url)?;

        // Serialize request body
        let body = serde_json::to_string(&payload)?;
        let body: reqwest::Body = body.into();

        let mut req = Request::new(Method::POST, url);
        self.add_credentials(req.headers_mut());
        *req.body_mut() = Some(body);

        // Add Content-Type Header
        req.headers_mut()
            .insert("Content-Type", HeaderValue::from_static("application/json"));

        match self.client.execute(req).await {
            Ok(v) if v.status().is_success() => Ok(v.json::<Q>().await?),
            Ok(v) => Err(ClientError::MikrotikError(v.json::<MikrotikError>().await?)),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn execute_post_with_no_response<P: Serialize>(
        &mut self,
        url: &str,
        payload: P,
    ) -> Result<(), ClientError> {
        let url = self.base_url.join(url)?;

        // Serialize request body
        let body = serde_json::to_string(&payload)?;
        let body: reqwest::Body = body.into();

        let mut req = Request::new(Method::POST, url);
        self.add_credentials(req.headers_mut());
        *req.body_mut() = Some(body);

        // Add Content-Type Header
        req.headers_mut()
            .insert("Content-Type", HeaderValue::from_static("application/json"));

        match self.client.execute(req).await {
            Ok(v) if v.status().is_success() => Ok(()),
            Ok(v) => Err(ClientError::MikrotikError(v.json::<MikrotikError>().await?)),
            Err(e) => Err(e.into()),
        }
    }
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error(transparent)]
    UrlError(#[from] url::ParseError),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    ParseFloatError(#[from] ParseFloatError),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    MikrotikError(#[from] MikrotikError),
}

use serde::{Deserialize as DeserializeDerive, Serialize as SerializeDerive};

#[derive(Default, Error, Debug, Clone, PartialEq, SerializeDerive, DeserializeDerive)]
#[serde(rename_all = "camelCase")]
pub struct MikrotikError {
    pub error: u16,
    pub message: String,
    pub detail: String,
}

impl Display for MikrotikError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}
