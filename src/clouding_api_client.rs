use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{error::Error, vec};

#[derive(Deserialize)]
struct Server {
    id: String,
    name: String,
}

#[derive(Deserialize)]
struct GetServersResponse {
    servers: Vec<Server>,
}

#[derive(Deserialize)]
struct GetFirewallsResponse {
    values: Vec<Firewall>,
}
#[derive(Debug, Deserialize)]
pub struct Firewall {
    pub id: String,
    name: String,
    description: String,
    pub rules: Vec<FirewallRule>,
}

#[derive(Debug, Deserialize)]
pub struct FirewallRule {
    id: String,
    protocol: String,
    description: String,
    portRangeMin: Option<i32>,
    portRangeMax: Option<i32>,
    sourceIp: String,
    enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct DeleteRuleResponse {
    title: String,
    status: Option<i32>,
    detail: Option<String>,
    instance: Option<String>,
    traceId: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRuleRequest {
    sourceIp: String,
    protocol: String,
    description: String,
    portRangeMin: i32,
    portRangeMax: i32,
}

pub struct CloudingApiClient {
    base_url: String,
    api_key: String,
    client: Client,
}

impl CloudingApiClient {
    pub fn new(api_key: String) -> Self {
        CloudingApiClient {
            base_url: "https://api.clouding.io/v1".to_string(),
            api_key,
            client: Client::new(),
        }
    }

    pub async fn get_server_id_by_name(&self, server_name: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/servers?page=1&pageSize=100", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Content-Type", "application/json")
            .header("X-API-KEY", &self.api_key)
            .send()
            .await?;

        let servers = response.json::<GetServersResponse>().await?;
        match servers
            .servers
            .into_iter()
            .find(|server| server.name == server_name)
        {
            Some(server) => Ok(server.id),
            None => Err("Server not found".into()), // Crear un error simple
        }
    }

    pub async fn get_firewall_by_name(
        &self,
        firewall_name: &str,
    ) -> Result<Firewall, Box<dyn Error>> {
        let url = format!("{}/firewalls?page=1&pageSize=100", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Content-Type", "application/json")
            .header("X-API-KEY", &self.api_key)
            .send()
            .await?;

        let firewalls = response.json::<GetFirewallsResponse>().await?.values;
        match firewalls
            .into_iter()
            .find(|firewall| firewall.name == firewall_name)
        {
            Some(firewall) => Ok(firewall),
            None => Err("Server not found".into()), // Crear un error simple
        }
    }

    pub async fn clean_firewall_rdp_rules(
        &self,
        rules: &Vec<FirewallRule>,
    ) -> Result<i8, Box<dyn Error>> {
        
        let mut counter = 0;
        for rule in rules {
            if (rule.portRangeMax == Some(3389) && rule.portRangeMin == Some(3389)) {
                let url = format!("{}/firewalls/rules/{}", self.base_url, rule.id);
                println!("Deleting {}", url);
                let response = self
                    .client
                    .delete(&url)
                    .header("Content-Type", "application/json")
                    .header("X-API-KEY", &self.api_key)
                    .send()
                    .await?;

                // Verificar que la respuesta sea exitosa
                if response.status().is_success() {
                    println!("Successfully deleted rule: {}", rule.id);
                    counter += 1;
                } else {
                    println!("Failed to delete rule: {}", rule.id);
                    // Opcional: Puedes decidir si quieres recoger más detalles del error o simplemente continuar
                }
            }
        }

        Ok((counter))
    }

    pub async fn set_new_rdp_rules(
        &self,
        firewall_id: &str,
        ip: &str,
    ) -> Result<(), Box<dyn Error>> {  // Changed return type to Result<(), Box<dyn Error>>
        let mut rule_request = CreateRuleRequest {
            sourceIp: format!("{}/32", ip), // Ensure IP is correctly formatted
            protocol: "TCP".to_string(),
            description: "Allow RDP Rust".to_string(),
            portRangeMin: 3389,
            portRangeMax: 3389,
        };
    
        let url = format!("{}/firewalls/{}/rules", self.base_url, firewall_id);
        println!("{}",url);
        let responseTCP = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("X-API-KEY", &self.api_key)
            .json(&rule_request)
            .send()
            .await?;

            rule_request.protocol="UDP".to_string();
            let responseUDP = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("X-API-KEY", &self.api_key)
            .json(&rule_request)
            .send()
            .await?;
    
        if responseTCP.status().is_success() && responseUDP.status().is_success() {
            println!("Rule successfully created.");
            Ok(())  // Return an empty Ok since no data needs to be returned
        } else {
            Err(format!("Failed to create rule: HTTP Status {} {}", responseTCP.status(), responseUDP.status() ).into())
        }
    }

}

#[tokio::main]
async fn main() {
    let api_client = CloudingApiClient::new("your_api_key_here".to_string());
    match api_client
        .get_server_id_by_name("desired_server_name")
        .await
    {
        Ok(server_id) => println!("Server ID: {}", server_id),
        Err(e) => println!("Error: {}", e),
    }
}
