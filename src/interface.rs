pub use http::Method;
use serde::{Deserialize, Serialize};

pub enum ApiKind {
    SpotApi,
    SpotWeb,
    FuturesApi,
    FuturesWeb,
}

pub trait Rest: Serialize {
    type Response: for<'de> Deserialize<'de> + Send + 'static;

    fn api_kind(&self) -> ApiKind;

    /// Get request method.
    fn method(&self) -> Method;

    /// Get request path.
    fn path(&self) -> String;

    /// Whether need sign.
    fn need_sign(&self) -> bool {
        false
    }
}
