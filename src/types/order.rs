use exc_core::Symbol;
use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct PlaceOrderRequest {
    pub size: f64,
    pub price: f64,
    pub kind: OrderType,
    pub leverage: f64,
}
impl PlaceOrderRequest {
    pub fn new(size: f64, price: f64, kind: OrderType) -> Self {
        Self {
            size,
            price,
            kind,
            leverage: 1.0,
        }
    }
    pub fn set_leverage(&mut self, leverage: f64) {
        self.leverage = leverage;
    }
}

#[derive(Debug)]
pub struct OrderId {
    pub symbol: Symbol,
    pub order_id: Option<String>,
    pub custom_order_id: Option<String>,
}

#[derive(Debug)]
pub struct Order {
    pub symbol: String,
    pub order_id: String,
    pub price: f64,
    pub vol: f64,
    pub deal_vol: f64,
    pub deal_avg_price: f64,
    pub state: OrderStatus,
    pub order_type: OrderType,
    pub side: OrderSide,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(i8)]
pub enum OrderSide {
    #[num_enum(default)]
    Unknown = 0,
    Buy = 1,
    Sell = 3,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(i8)]
pub enum OrderType {
    #[num_enum(default)]
    Unknown = 0,
    Limit = 1,
    Market = 5,
    LimitMaker = 2,
    LmmediateOrCancel = 3,
    FillOrKill = 4,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(i8)]
pub enum OrderStatus {
    #[num_enum(default)]
    Unknown = 0,
    New = 1,
    Filled = 3,
    PartiallyFilled = 2,
    Canceled = 4,
    PartiallyCanceled = 5,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(i8)]
pub enum FuturesOpenType {
    #[num_enum(default)]
    Unknown = 0,
    Isolated = 1,
    Cross = 2,
}
