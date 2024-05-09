use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub firewall_name: String,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        let mut settings = config::Config::new();
        settings.merge(config::File::with_name("appsettings.json"))?;
        settings.try_into()
    }
}