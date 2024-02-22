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
