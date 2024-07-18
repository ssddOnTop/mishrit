use bytes::Bytes;
use http_body_util::BodyExt;
use hyper::{HeaderMap, Method, Uri, Version};
use crate::plan::Plan;

#[derive(Default)]
pub struct Request<Body> {
    pub method: Method,
    pub uri: Uri,
    pub version: Version,
    pub headers: HeaderMap,
    pub body: Body,
}

impl<Body> Request<Body> {
    pub fn new(headers: HeaderMap, body: Body) -> Self {
        Self { headers, body, ..Default::default() }
    }
}

impl<Body: AsRef<[u8]>> Request<Body> {
    pub fn to_plan(&self) -> anyhow::Result<Plan> {
        Ok(serde_json::from_slice(self.body.as_ref())?)
    }
}

impl Request<Bytes> {
    pub async fn from_hyper(req: hyper::Request<hyper::body::Incoming>) -> anyhow::Result<Self> {
        let (parts, body) = req.into_parts();
        let body = body.collect().await?.to_bytes();

        Ok(Self {
            method: parts.method,
            uri: parts.uri,
            version: parts.version,
            headers: parts.headers,
            body,
        })
    }
}