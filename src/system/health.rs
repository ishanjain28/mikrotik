pub use crate::system::types::Health;
use crate::{Client, ClientError};

/// health can be used to get all health parameters of the device
pub async fn health(client: &mut Client) -> Result<Vec<Health>, ClientError> {
    let url = format!("{}/health", super::BASE);

    client.execute_get::<Vec<Health>>(&url).await
}

/// voltage can be used to get device's voltag reading
pub async fn voltage(client: &mut Client) -> Result<f32, ClientError> {
    let url = format!("{}/health/voltage", super::BASE);

    let response = client.execute_get::<Health>(&url).await?;

    response
        .value
        .parse::<f32>()
        .map_err(ClientError::ParseFloatError)
}

/// temperature can be used to get device's temperature reading
pub async fn temperature(client: &mut Client) -> Result<String, ClientError> {
    let url = format!("{}/health/temperature", super::BASE);

    let mut response = client.execute_get::<Health>(&url).await?;

    response.value.push_str(&response.type_field);
    Ok(response.value)
}
