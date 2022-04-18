use crate::{system::types::Health, Client, ClientError};

pub async fn health(client: &mut Client) -> Result<Vec<Health>, ClientError> {
    let url = format!("{}/health", super::BASE);

    client.execute_get::<Vec<Health>>(&url).await
}
