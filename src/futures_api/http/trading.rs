use crate::interface::{ApiKind, Method, Rest};
use crate::types::order::{OrderSide, OrderStatus, OrderType};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, FromInto};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderRequest {
    #[serde(skip)]
    pub symbol: String,
    #[serde(skip)]
    pub order_id: Option<String>,
    #[serde(skip)]
    pub external_oid: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderResponse {
    pub symbol: String,
    pub order_id: String,
    pub external_oid: Option<String>,
    pub price: f64,
    pub vol: f64,
    pub deal_vol: f64,
    pub deal_avg_price: f64,
    #[serde_as(as = "FromInto<i8>")]
    pub state: OrderStatus,
    #[serde_as(as = "FromInto<i8>")]
    pub order_type: OrderType,
    #[serde_as(as = "FromInto<i8>")]
    pub side: OrderSide,
}

impl Rest for GetOrderRequest {
    type Response = GetOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        if let Some(id) = &self.order_id {
            return format!("/api/v1/private/order/get/{}", id);
        }
        if let Some(id) = &self.external_oid {
            return format!("/api/v1/private/order/external/{}/{}", self.symbol, id);
        }
        String::new()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
