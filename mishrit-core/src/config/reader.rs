use crate::config::source::Source;
use crate::config::Config;
use crate::runtime::TargetRuntime;
use reqwest::Url;

pub struct ConfigReader {
    runtime: TargetRuntime,
}

#[derive(Debug)]
pub struct FileRead {
    pub content: String,
    pub path: String,
}

impl ConfigReader {
    pub fn new(runtime: TargetRuntime) -> Self {
        Self { runtime }
    }

    pub async fn read<T: AsRef<str>>(&self, path: T) -> anyhow::Result<Config> {
        let path = path.as_ref();
        let file_io = self.perform_io(path).await?;
        let source = Source::detect(&file_io.path)?;
        let config = Config::from_source(source, &file_io.content)?;

        Ok(config)
    }

    async fn perform_io<T: AsRef<str>>(&self, path: T) -> anyhow::Result<FileRead> {
        let content = if let Ok(url) = Url::parse(path.as_ref()) {
            // Is an HTTP URL
            if url.scheme().starts_with("http") {
                let response = self
                    .runtime
                    .http_io
                    .execute(reqwest::Request::new(reqwest::Method::GET, url))
                    .await?;

                String::from_utf8(response.body.to_vec())?
            } else {
                // Is a file path on Windows
                let content = self.runtime.file_io.read(path.as_ref()).await?;
                String::from_utf8(content)?
            }
        } else {
            // Is a file path
            let content = self.runtime.file_io.read(path.as_ref()).await?;
            String::from_utf8(content)?
        };

        Ok(FileRead {
            path: path.as_ref().to_string(),
            content,
        })
    }
}
