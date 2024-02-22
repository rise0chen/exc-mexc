use crate::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PingRequest;

#[derive(Debug, Deserialize)]
pub struct PingResponse {}

impl Rest for PingRequest {
    type Response = PingResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v3/ping".to_string()
    }
}
