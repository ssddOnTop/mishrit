use crate::http::Body;
use anyhow::Result;
use bytes::Bytes;
use crate::http::request::Request;

pub async fn handle_request(req: hyper::Request<hyper::body::Incoming>) -> Result<hyper::Response<Body>> {
    let request = Request::from_hyper(req).await?;
    handle_request_inner(request).await
}


async fn handle_request_inner(req: Request<Bytes>) -> Result<hyper::Response<Body>> {
    let _plan = req.to_plan()?;
    // TODO: maybe just add `execute` function to `Plan` or new `ExecutionPlan`?
    
    todo!()
}