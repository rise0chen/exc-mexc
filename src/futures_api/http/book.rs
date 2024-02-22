use crate::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBidAskRequest {
    #[serde(skip)]
    pub symbol: String,
    pub limit: u16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
/// price, size, ordder_num
pub struct BidAsk(pub f64, pub f64, pub f64);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBidAskResponse {
    pub asks: Vec<BidAsk>,
    pub bids: Vec<BidAsk>,
    pub version: u64,
    pub timestamp: u64,
}

impl Rest for GetBidAskRequest {
    type Response = GetBidAskResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("/api/v1/contract/depth/{}", self.symbol)
    }
    fn need_sign(&self) -> bool {
        false
    }
}
