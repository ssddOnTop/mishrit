use crate::runtime::file::FileIO;
use crate::runtime::http::HttpIO;
use std::sync::Arc;

#[derive(Clone)]
pub struct TargetRuntime {
    pub http_io: Arc<dyn HttpIO>,
    pub file_io: Arc<dyn FileIO>,
}
