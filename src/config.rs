use config::{Config, File};
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct CapnpServer {
    pub enabled: bool,
    pub listener: String
}

#[derive(Debug,Deserialize)]
pub struct GrpcServer {
    pub enabled: bool,
    pub listener: String
}

#[derive(Debug,Deserialize)]
pub struct HttpServer {
    pub enabled: bool,
    pub listener: String
}

#[derive(Debug,Deserialize)]
pub struct Servers {
    pub capnp: CapnpServer,
    pub grpc: GrpcServer,
    pub http: HttpServer,
}


#[derive(Debug,Deserialize)]
pub struct AppConfig{
    pub servers: Servers
}

pub fn load_config()->Result<AppConfig,config::ConfigError>{
    let settings = Config::builder()
        .add_source(File::with_name("config/default"))
        .build()?.try_deserialize()?;
    Ok(settings)
}
