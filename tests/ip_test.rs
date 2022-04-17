use mikrotik::{ip::dhcp_server::DhcpServerError, Client};
use reqwest::Url;

#[tokio::test]
async fn list_dhcp_servers() -> Result<(), DhcpServerError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::dhcp_server::list(&mut client).await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn get_dhcp_server() -> Result<(), DhcpServerError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::dhcp_server::get(&mut client, "vlan-150").await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn list_network() -> Result<(), DhcpServerError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::dhcp_server::list_network(&mut client).await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn get_network() -> Result<(), DhcpServerError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::dhcp_server::list_network(&mut client).await?;

    let response = mikrotik::ip::dhcp_server::get_network(&mut client, &response[0].id).await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn list_leases() -> Result<(), DhcpServerError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::dhcp_server::list_leases(&mut client).await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn get_lease() -> Result<(), DhcpServerError> {
    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
        .expect("error in creating client");

    let response = mikrotik::ip::dhcp_server::list_leases(&mut client).await?;

    let response = mikrotik::ip::dhcp_server::get_lease(&mut client, &response[0].id).await?;

    println!("{:?}", response);

    Ok(())
}
