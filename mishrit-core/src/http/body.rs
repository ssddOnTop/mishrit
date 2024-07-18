use bytes::Bytes;
use http_body_util::Full;

pub struct Body(Full<Bytes>);

impl From<Bytes> for Body {
    fn from(value: Bytes) -> Self {
        Self(Full::new(value))
    }
}

impl Default for Body {
    fn default() -> Self {
        Self(Full::new(Bytes::default()))
    }
}