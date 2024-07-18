use crate::config::source::Source;
use crate::is_default;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Deserialize, Serialize)]
/// Configuration for the server.
pub struct Config {
    /// Server configuration.
    #[serde(default, skip_serializing_if = "is_default")]
    pub server: Option<ServerConfig>,
}

#[derive(Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct ServerConfig {
    #[serde(default, skip_serializing_if = "is_default")]
    /// The hostname to bind to.
    pub hostname: Option<String>,

    #[serde(default, skip_serializing_if = "is_default")]
    /// The port to bind to.
    pub port: Option<u16>,

    #[serde(default, skip_serializing_if = "is_default")]
    /// Webhook url to for logging based on log level.
    pub webhook: Option<String>,

    #[serde(default, skip_serializing_if = "is_default")]
    /// Log level for the server.
    pub log_level: Option<String>,

    #[serde(default, skip_serializing_if = "is_default")]
    /// Should the server ask for admin permission
    pub admin: Option<bool>,
}

impl Config {
    pub fn from_source(source: Source, schema: &str) -> anyhow::Result<Self> {
        match source {
            Source::Json => Ok(Config::from_json(schema)?),
            Source::Yml => Ok(Config::from_yaml(schema)?),
        }
    }
    pub fn from_json(json: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn from_yaml(yaml: &str) -> anyhow::Result<Self> {
        Ok(serde_yaml::from_str(yaml)?)
    }
}
