use bytes::Bytes;

#[derive(Clone, Debug, Default, derive_setters::Setters)]
pub struct Response<Body> {
    pub status: reqwest::StatusCode,
    pub headers: reqwest::header::HeaderMap,
    pub body: Body,
}

impl Response<Bytes> {
    pub async fn from_reqwest(resp: reqwest::Response) -> anyhow::Result<Self> {
        let status = resp.status();
        let headers = resp.headers().to_owned();
        let body = resp.bytes().await?;
        Ok(Response {
            status,
            headers,
            body,
        })
    }
}
