use crate::{interface::types::Interface, Client, ClientError};

pub async fn list(client: &mut Client) -> Result<Vec<Interface>, ClientError> {
    let url = super::BASE;

    client.execute_get::<Vec<Interface>>(&url).await
}

pub async fn get(client: &mut Client, ifid: &str) -> Result<Interface, ClientError> {
    let url = format!("{}/{}", super::BASE, ifid);

    client.execute_get::<Interface>(&url).await
}
