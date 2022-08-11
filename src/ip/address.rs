pub use crate::ip::types::{AddAddressInput, AddAddressOutput, Address};
use crate::{Client, ClientError};
use serde::{Deserialize, Serialize};

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

pub async fn add(
    client: &mut Client,
    input: AddAddressInput,
) -> Result<AddAddressOutput, ClientError> {
    let url = format!("{}/address/add", super::BASE);

    client
        .execute_post::<AddAddressInput, AddAddressOutput>(&url, input)
        .await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RemoveAddressInput {
    pub numbers: String,
}

pub async fn remove(client: &mut Client, id: &str) -> Result<(), ClientError> {
    let url = format!("{}/address/remove", super::BASE);

    client
        .execute_post_with_no_response::<RemoveAddressInput>(
            &url,
            RemoveAddressInput {
                numbers: id.to_string(),
            },
        )
        .await
}

#[cfg(test)]
mod test {
    use super::*;
    use reqwest::Url;

    #[tokio::test]
    async fn remove_address() -> Result<(), ClientError> {
        let base = Url::parse("https://10.0.10.1")?;
        let mut client = Client::new(base, "pia".to_string(), "qwertyuiop".to_string(), true)
            .expect("error in creating client");

        let response = crate::ip::address::add(
            &mut client,
            AddAddressInput {
                address: "10.20.0.1/24".to_string(),
                disabled: false,
                interface: "pia".to_string(),
                ..Default::default()
            },
        )
        .await?;

        crate::ip::address::remove(&mut client, &response.ret).await?;

        Ok(())
    }

    #[tokio::test]
    async fn add_address() -> Result<(), ClientError> {
        let base = Url::parse("https://10.0.10.1")?;
        let mut client = Client::new(base, "pia".to_string(), "qwertyuiop".to_string(), true)
            .expect("error in creating client");

        let response = crate::ip::address::add(
            &mut client,
            AddAddressInput {
                address: "10.20.0.1/24".to_string(),
                disabled: false,
                interface: "pia".to_string(),
                ..Default::default()
            },
        )
        .await?;

        println!("{:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn list_address() -> Result<(), ClientError> {
        let base = Url::parse("https://10.0.10.1")?;
        let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
            .expect("error in creating client");

        let response = crate::ip::address::list(&mut client).await?;

        println!("{:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn get_address() -> Result<(), ClientError> {
        let base = Url::parse("https://10.0.10.1")?;
        let mut client = Client::new(base, "admin".to_string(), "ifd783far".to_string(), true)
            .expect("error in creating client");

        let response = crate::ip::address::list(&mut client).await?;

        let response = crate::ip::address::get(&mut client, &response[0].id).await?;

        println!("{:?}", response);

        Ok(())
    }
}
