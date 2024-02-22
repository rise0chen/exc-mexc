pub mod book;
pub mod trading;

use crate::interface::{Method, Rest};
use crate::key::{ApiKind, Key, ParamsFormat};
use http::Request;
use hyper::Body;

const HOST: &str = "https://contract.mexc.com";

pub fn req_to_http<Req: Rest>(req: &Req, key: &Key) -> Result<Request<Body>, anyhow::Error> {
    let mut builder = Request::builder()
        .method(req.method())
        .header("ApiKey", key.api_key.as_str())
        .header("content-type", "application/json");
    let mut uri = format!("{}/{}", HOST, req.path());
    let body = match req.method() {
        Method::GET => {
            let body_str = if req.need_sign() {
                let signature = key.sign(req, ParamsFormat::Urlencoded, ApiKind::FuturesApi)?;
                builder = builder
                    .header("Request-Time", signature.signing.timestamp)
                    .header("Signature", &signature.signature);
                serde_urlencoded::to_string(signature.signing)?
            } else {
                serde_urlencoded::to_string(req)?
            };
            uri.push('?');
            uri.push_str(&body_str);
            hyper::Body::empty()
        }
        _ => {
            let body_str = if req.need_sign() {
                let signature = key.sign(req, ParamsFormat::Json, ApiKind::FuturesApi)?;
                builder = builder
                    .header("Request-Time", signature.signing.timestamp)
                    .header("Signature", &signature.signature);
                serde_json::to_string(&signature.signing)?
            } else {
                serde_json::to_string(req)?
            };
            hyper::Body::from(body_str)
        }
    };

    let request = builder.uri(uri).body(body)?;
    Ok(request)
}
