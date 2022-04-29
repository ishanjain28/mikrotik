mod types;
pub mod wireguard;

pub use crate::interface::types::{Interface, InterfaceType, Mtu};
use crate::{Client, ClientError};

const BASE: &str = "rest/interface";

pub async fn list(client: &mut Client) -> Result<Vec<Interface>, ClientError> {
    client.execute_get::<Vec<Interface>>(BASE).await
}

pub async fn get(client: &mut Client, ifid: &str) -> Result<Interface, ClientError> {
    let url = format!("{}/{}", BASE, ifid);

    client.execute_get::<Interface>(&url).await
}
