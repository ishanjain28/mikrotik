use mikrotik::{Client, ClientError};
use reqwest::Url;

#[tokio::test]
async fn health() -> Result<(), ClientError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::system::health(&mut client).await?;

    println!("{:?}", response);

    Ok(())
}
