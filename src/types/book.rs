use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct BidAsk {
    /// (price, size)
    pub bid: (f64, f64),
    /// (price, size)
    pub ask: (f64, f64),
    pub version: u64,
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
