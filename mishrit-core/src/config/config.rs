use crate::config::source::Source;
use crate::is_default;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Deserialize, Serialize)]
/// Configuration for the server.
pub struct Config {
    /// Server configuration.
    #[serde(default, skip_serializing_if = "is_default")]
    pub server: Server,
}

#[derive(Default, Clone, Deserialize, Serialize, PartialEq)]
pub enum HttpVersion {
    #[default]
    HTTP1,
    HTTP2,
}

#[derive(Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct Server {
    #[serde(default, skip_serializing_if = "is_default")]
    /// The hostname to bind to.
    pub hostname: Option<String>,

    #[serde(default, skip_serializing_if = "is_default")]
    /// The port to bind to.
    pub port: Option<u16>,

    #[serde(default, skip_serializing_if = "is_default")]
    /// `version` sets the HTTP version for the server. Options are `HTTP1` and
    /// `HTTP2`. @default `HTTP1`.
    pub version: Option<HttpVersion>,

    #[serde(default, skip_serializing_if = "is_default")]
    /// `workers` sets the number of worker threads. @default the number of
    /// system cores.
    pub workers: Option<usize>,
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

impl Server {
    pub fn get_hostname(&self) -> &str {
        self.hostname.as_deref().unwrap_or("localhost")
    }

    pub fn get_port(&self) -> u16 {
        self.port.unwrap_or(19194)
    }

    pub fn get_version(&self) -> HttpVersion {
        self.version.clone().unwrap_or_default()
    }
    pub fn get_workers(&self) -> usize {
        self.workers.unwrap_or(num_cpus::get())
    }
}
