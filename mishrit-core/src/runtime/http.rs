use crate::http::Response;
use bytes::Bytes;

#[async_trait::async_trait]
pub trait HttpIO: Sync + Send + 'static {
    async fn execute(&self, request: reqwest::Request) -> anyhow::Result<Response<Bytes>>;
}

#[cfg(test)]
pub mod test {
    use super::*;
    use async_trait::async_trait;
    use reqwest::Client;

    #[derive(Default)]
    pub struct HttpIOMock {
        client: Client,
    }

    #[async_trait]
    impl HttpIO for HttpIOMock {
        async fn execute(&self, request: reqwest::Request) -> anyhow::Result<Response<Bytes>> {
            let resp = self.client.execute(request).await?;
            Response::from_reqwest(resp).await
        }
    }
}
