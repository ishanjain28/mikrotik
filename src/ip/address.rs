pub use crate::ip::types::Address;
use crate::{Client, ClientError};

/// list all configurd ipv4 addresses
pub async fn list(client: &mut Client) -> Result<Vec<Address>, ClientError> {
    let url = format!("{}/address", super::BASE);

    client.execute_get::<Vec<Address>>(&url).await
}

/// get details of a specific ipv4 address
pub async fn get(client: &mut Client, aid: &str) -> Result<Address, ClientError> {
    let url = format!("{}/address/{}", super::BASE, aid);

    client.execute_get::<Address>(&url).await
}
