use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{error::Error, vec};

#[derive(Deserialize)]
struct GetIpResponse {
    ip: String
}

pub struct IpApiClient {
    base_url: String,
    client: Client,
}



impl IpApiClient {
    pub fn new() -> Self {
        IpApiClient {
            base_url: "https://api.ipify.org?format=json".to_string(),
            client: Client::new(),
        }
    }

    pub async fn get_external_ip(&self) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(&self.base_url)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let ipResponse = response.json::<GetIpResponse>().await?;
        match ipResponse
            .into()
        {
            Some(r) => Ok(r.ip),
            None => Err("Error gettting ip".into()), // Crear un error simple
        }
    }


}
