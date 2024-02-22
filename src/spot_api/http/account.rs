use crate::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct Asset {
    pub asset: String,
    #[serde_as(as = "DisplayFromStr")]
    pub free: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub locked: f64,
}

#[derive(Debug, Serialize)]
pub struct GetAccountRequest;

#[derive(Debug, Deserialize)]
pub struct GetAccountResponse {
    pub balances: Vec<Asset>,
}

impl Rest for GetAccountRequest {
    type Response = GetAccountResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v3/account".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
