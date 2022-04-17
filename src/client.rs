use color_eyre::Report;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Request, Url,
};

pub struct Client {
    pub client: reqwest::Client,
    pub base_url: Url,
    pub basic_auth: HeaderValue,
    pub self_signed_cert: bool,
}

impl Client {
    pub fn new(
        base_url: Url,
        username: String,
        password: String,
        self_signed_cert: bool,
    ) -> Result<Self, Report> {
        let value = format!("{}:{}", username, password);
        let value = base64::encode(value);

        let basic_auth = HeaderValue::from_str(&format!("Basic {}", value))?;

        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(self_signed_cert)
            .build()?;

        Ok(Self {
            client,
            base_url,
            basic_auth,
            self_signed_cert,
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
}
