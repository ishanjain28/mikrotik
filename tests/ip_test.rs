use color_eyre::Report;
use mikrotik::Client;
use reqwest::Url;
use std::sync::Once;
use tracing_subscriber::EnvFilter;

static INIT: Once = Once::new();

fn setup() -> Result<(), Report> {
    INIT.call_once(|| {
        if std::env::var("RUST_LIB_BACKTRACE").is_err() {
            std::env::set_var("RUST_LIB_BACKTRACE", "1")
        }
        color_eyre::install();

        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "info");
        }

        tracing_subscriber::fmt::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    });

    Ok(())
}

#[tokio::test]
async fn list_dhcp_servers() -> Result<(), Report> {
    setup()?;

    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)?;

    let response = mikrotik::ip::dhcp_server::list(&mut client).await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn get_dhcp_server() -> Result<(), Report> {
    setup()?;

    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)?;

    let response = mikrotik::ip::dhcp_server::get(&mut client, "vlan-150").await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn list_network() -> Result<(), Report> {
    setup()?;

    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)?;

    let response = mikrotik::ip::dhcp_server::list_network(&mut client).await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn get_network() -> Result<(), Report> {
    setup()?;

    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)?;

    let response = mikrotik::ip::dhcp_server::list_network(&mut client).await?;

    let response = mikrotik::ip::dhcp_server::get_network(&mut client, &response[0].id).await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn list_leases() -> Result<(), Report> {
    setup()?;

    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)?;

    let response = mikrotik::ip::dhcp_server::list_leases(&mut client).await?;

    println!("{:?}", response);

    Ok(())
}

#[tokio::test]
async fn get_lease() -> Result<(), Report> {
    setup()?;

    let base = Url::parse("https://10.0.10.1")?;
    let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)?;

    let response = mikrotik::ip::dhcp_server::list_leases(&mut client).await?;

    let response = mikrotik::ip::dhcp_server::get_lease(&mut client, &response[0].id).await?;

    println!("{:?}", response);

    Ok(())
}
