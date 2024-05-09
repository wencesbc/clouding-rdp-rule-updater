use std::env::{self, args};

mod clouding_api_client;
mod config;
mod getip_api_client;

#[tokio::main]
async fn main() {
    let mut api_client;
    let mut firewall_name;
    let mut new_ip: String;

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        new_ip = args[1].clone();
    } else {
        let ip_client = getip_api_client::IpApiClient::new();
        match ip_client.get_external_ip().await {
            Ok(ip) => {
                println!("Ip loaded: {:?}", ip);
                new_ip = ip;
            }
            Err(e) => {
                eprintln!("Failed to load configurations: {}", e);
                return;
            }
        }
    }

    match config::Config::load() {
        Ok(config) => {
            println!("Configurations loaded: {:?}", config);
            api_client = clouding_api_client::CloudingApiClient::new(config.api_key);
            firewall_name = config.firewall_name;
            // Continúa con la lógica de tu aplicación usando `api_client`
        }
        Err(e) => {
            eprintln!("Failed to load configurations: {}", e);
            return;
        }
    }

    // Obtener el firewall por nombre
    match api_client.get_firewall_by_name(&firewall_name).await {
        Ok(firewall) => {
            println!("Firewall: {:#?}", firewall);
            // Limpieza de reglas de RDP si el firewall fue encontrado
            match api_client.clean_firewall_rdp_rules(&firewall.rules).await {
                Ok(deleted_count) => println!("Number of RDP rules deleted: {}", deleted_count),
                Err(e) => println!("Error deleting RDP rules: {}", e),
            }

            match api_client.set_new_rdp_rules(&firewall.id, &new_ip ).await {
                Ok(_) => println!("New rules created"),
                Err(e) => println!("Error deleting RDP rules: {}", e),
            }
        }
        Err(e) => println!("Error retrieving firewall: {}", e),
    }
}
