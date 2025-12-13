// src/utils/config.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::utils::error::{Result, Error};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShodanConfig {
    pub api_key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FofaConfig {
    pub email: String,
    pub api_key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetasploitConfig {
    pub msfgrpc_host: String,
    pub msfgrpc_port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub shodan: Option<ShodanConfig>,
    pub fofa: Option<FofaConfig>,
    pub metasploit: Option<MetasploitConfig>,

    #[serde(default)]
    pub log_level: String,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            shodan: None,
            fofa: None,
            metasploit: None,
            log_level: "info".to_string(),
            extra: HashMap::new(),
        }
    }
}

pub struct ConfigLoader;

impl ConfigLoader {
    /// Carrega configuração de `./config.{yaml,toml,json}` e variáveis de ambiente
    pub fn load() -> Result<AppConfig> {
        let mut settings = config::Config::builder();

        // Adiciona arquivo de configuração (sem exigir que exista)
        settings = settings
            .add_source(config::File::with_name("config").required(false));

        // Adiciona variáveis de ambiente com prefixo PH_ (ex: PH_SHODAN_API_KEY)
        settings = settings
            .add_source(config::Environment::with_prefix("PH").separator("_"));

        let cfg = settings
            .build()
            .map_err(|e| Error::Other(format!("Config deserialization error: {}", e)))?;

        let app_config = cfg
            .try_deserialize()
            .map_err(|e| Error::Other(format!("Config deserialization error: {}", e)))?;

        Ok(app_config)
    }
}