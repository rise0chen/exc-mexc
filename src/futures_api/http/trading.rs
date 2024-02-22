use crate::interface::{ApiKind, Method, Rest};
use crate::types::order::{OrderSide, OrderStatus, OrderType};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, FromInto};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderRequest {
    pub order_id: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderResponse {
    pub symbol: String,
    pub order_id: String,
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
        format!("/api/v1/private/order/get/{}", self.order_id)
    }
    fn need_sign(&self) -> bool {
        true
    }
}
