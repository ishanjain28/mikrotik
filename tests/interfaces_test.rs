use mikrotik::{interface::InterfaceError, Client};
use reqwest::Url;

#[tokio::test]
async fn list_interfaces() -> Result<(), InterfaceError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::interface::list(&mut client).await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn get_interface() -> Result<(), InterfaceError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::interface::get(&mut client, "ether5").await?;

    println!("{:?}", response);

    Ok(())
}
