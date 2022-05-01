use mikrotik::{ip::dhcp_client::DhcpClientInput, Client, ClientError};
use reqwest::Url;

#[tokio::test]
async fn list_dhcp_servers() -> Result<(), ClientError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::dhcp_server::list(&mut client).await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn get_dhcp_server() -> Result<(), ClientError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::dhcp_server::get(&mut client, "vlan-150").await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn list_network() -> Result<(), ClientError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::dhcp_server::list_network(&mut client).await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn get_network() -> Result<(), ClientError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::dhcp_server::list_network(&mut client).await?;

    let response = mikrotik::ip::dhcp_server::get_network(&mut client, &response[0].id).await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn list_leases() -> Result<(), ClientError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::dhcp_server::list_leases(&mut client).await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn get_lease() -> Result<(), ClientError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::dhcp_server::list_leases(&mut client).await?;

    let response = mikrotik::ip::dhcp_server::get_lease(&mut client, &response[0].id).await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn list_address() -> Result<(), ClientError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::address::list(&mut client).await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn get_address() -> Result<(), ClientError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::address::list(&mut client).await?;

    let response = mikrotik::ip::address::get(&mut client, &response[0].id).await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn list_dhcp_client() -> Result<(), ClientError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::dhcp_client::list(&mut client).await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn get_dhcp_client() -> Result<(), ClientError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::dhcp_client::list(&mut client).await?;

    let response = mikrotik::ip::dhcp_client::get(&mut client, &response[0].id).await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn renew_dhcp_lease() -> Result<(), ClientError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::dhcp_client::list(&mut client).await?;

    let response = mikrotik::ip::dhcp_client::renew(
        &mut client,
        DhcpClientInput {
            numbers: response[0].id.clone(),
        },
    )
    .await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn release_dhcp_lease() -> Result<(), ClientError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::dhcp_client::list(&mut client).await?;

    println!("{:?}", response);

    let response = mikrotik::ip::dhcp_client::release(
        &mut client,
        DhcpClientInput {
            numbers: response[0].id.clone(),
        },
    )
    .await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn list_routes() -> Result<(), ClientError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::route::list(&mut client).await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn get_route() -> Result<(), ClientError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::route::list(&mut client).await?;

    let response = mikrotik::ip::route::get(&mut client, &response[0].id).await?;

    println!("{:?}", response);

    Ok(())
}
