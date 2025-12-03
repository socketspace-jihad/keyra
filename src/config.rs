use config::{Config, File};
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Server {
    pub address: String,
    pub tls_enabled: bool,
}

#[derive(Debug,Deserialize)]
pub struct AppConfig{
    pub server: Server
}

pub fn load_config()->Result<AppConfig,config::ConfigError>{
    let settings = Config::builder()
        .add_source(File::with_name("config/default"))
        .build()?.try_deserialize()?;
    Ok(settings)
}
