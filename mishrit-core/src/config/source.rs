use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub enum Source {
    #[default]
    Json,
    Yml,
}

const JSON_EXT: &str = "json";
const YML_EXT: &str = "yml";

#[derive(Debug, Error, PartialEq)]
#[error("Unsupported config extension: {0}")]
pub struct UnsupportedConfigFormat(pub String);

const ALL: &[Source] = &[Source::Json, Source::Yml];

impl Source {
    pub fn detect(name: &str) -> Result<Source, UnsupportedConfigFormat> {
        ALL.iter()
            .find(|format| format.ends_with(name))
            .ok_or(UnsupportedConfigFormat(name.to_string()))
            .cloned()
    }
    fn ends_with(&self, file: &str) -> bool {
        file.ends_with(&format!(".{}", self.ext()))
    }
    pub fn ext(&self) -> &'static str {
        match self {
            Source::Json => JSON_EXT,
            Source::Yml => YML_EXT,
        }
    }
}
