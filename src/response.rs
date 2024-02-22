use exc_core::ExchangeError;
use serde::Deserialize;

/// HTTP API Response (with `code` and `msg`).
#[derive(Debug, Deserialize)]
pub struct FullHttpResponse<T> {
    /// Code.
    pub code: i64,
    /// Message.
    #[serde(alias = "message")]
    pub msg: Option<String>,
    /// Data.
    pub data: Option<T>,
}
impl<T> From<FullHttpResponse<T>> for Result<T, ExchangeError> {
    fn from(value: FullHttpResponse<T>) -> Self {
        if value.code == 0 {
            value.data.ok_or(ExchangeError::UnexpectedResponseType(String::new()))
        } else {
            Err(ExchangeError::Api(anyhow::anyhow!("[{}]: {}", value.code, value.msg.unwrap_or_default())))
        }
    }
}
