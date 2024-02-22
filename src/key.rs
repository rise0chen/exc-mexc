pub use crate::interface::ApiKind;
use exc_core::Str;
use hmac::{Hmac, Mac};
use md5::{Digest, Md5};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use time::OffsetDateTime;

pub enum ParamsFormat {
    Json,
    Urlencoded,
}

/// The APIKey definition of MEXC.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Key {
    pub api_key: Str,
    pub secret_key: Str,
    pub web_key: Option<Str>,
}

impl Key {
    /// Create a new [`Key`].
    pub fn new(api_key: &str, secret_key: &str, web_key: Option<&str>) -> Self {
        Self {
            api_key: Str::new(api_key),
            secret_key: Str::new(secret_key),
            web_key: web_key.map(Str::new),
        }
    }
    pub fn sign<T: Serialize>(&self, params: T, format: ParamsFormat, kind: ApiKind) -> Result<SignedParams<T>, anyhow::Error> {
        SigningParams::now(params).signed(self, format, kind)
    }
}

/// Signing params.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SigningParams<T> {
    #[serde(flatten)]
    pub params: T,
    #[serde(rename = "recvWindow")]
    pub recv_window: i64,
    pub timestamp: i64,
}

impl<T> SigningParams<T> {
    fn with_timestamp(params: T, timestamp: i64) -> Self {
        Self {
            params,
            recv_window: 5000,
            timestamp,
        }
    }

    /// Sign the given params now.
    pub fn now(params: T) -> Self {
        let now = OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000;
        Self::with_timestamp(params, now as i64)
    }
}

/// Signed params.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignedParams<T> {
    #[serde(flatten)]
    pub signing: SigningParams<T>,
    pub signature: String,
}

impl<T: Serialize> SigningParams<T> {
    /// Get signed params.
    pub fn signed(self, key: &Key, format: ParamsFormat, kind: ApiKind) -> Result<SignedParams<T>, anyhow::Error> {
        let raw = match format {
            ParamsFormat::Json => serde_json::to_string(&self)?,
            ParamsFormat::Urlencoded => serde_urlencoded::to_string(&self)?,
        };
        let signature = match kind {
            ApiKind::SpotApi => {
                let mut mac = Hmac::<Sha256>::new_from_slice(key.secret_key.as_bytes())?;
                mac.update(raw.as_bytes());
                let mac_result = mac.finalize();
                hex::encode(mac_result.into_bytes())
            }
            ApiKind::FuturesApi => {
                let mut mac = Hmac::<Sha256>::new_from_slice(key.secret_key.as_bytes())?;
                mac.update(format!("{}{}{}", key.api_key, self.timestamp, raw).as_bytes());
                let mac_result = mac.finalize();
                hex::encode(mac_result.into_bytes())
            }
            ApiKind::SpotWeb | ApiKind::FuturesWeb => {
                let Some(token) = &key.web_key else {
                    return Err(anyhow::anyhow!("no web_key"));
                };
                let sign = &hex::encode(&*Md5::new().chain_update(format!("{}{}", token, self.timestamp)).finalize())[7..];
                let mut mac = Md5::new();
                mac.update(format!("{}{}{}", self.timestamp, raw, sign));
                let mac_result = mac.finalize();
                hex::encode(&*mac_result)
            }
        };
        Ok(SignedParams { signing: self, signature })
    }
}
