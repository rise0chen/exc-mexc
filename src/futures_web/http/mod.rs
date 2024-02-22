pub mod trading;

use crate::interface::{Method, Rest};
use crate::key::{ApiKind, Key, ParamsFormat};
use http::Request;
use hyper::Body;

const HOST: &str = "https://futures.mexc.com";

pub fn req_to_http<Req: Rest>(req: &Req, key: &Key) -> Result<Request<Body>, anyhow::Error> {
    let mut builder = Request::builder().method(req.method()).header("content-type", "application/json");
    if req.need_sign() {
        if let Some(token) = &key.web_key {
            builder = builder
                .header("cookie", format!("u_id={}", token))
                .header("Authorization", token.as_str());
        }
    }
    let mut uri = format!("{}/{}", HOST, req.path());
    let body = match req.method() {
        Method::GET => {
            let body_str = if req.need_sign() {
                let signature = key.sign(req, ParamsFormat::Urlencoded, ApiKind::FuturesWeb)?;
                builder = builder
                    .header("x-mxc-nonce", signature.signing.timestamp)
                    .header("x-mxc-sign", &signature.signature);
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
                let signature = key.sign(req, ParamsFormat::Json, ApiKind::FuturesWeb)?;
                builder = builder
                    .header("x-mxc-nonce", signature.signing.timestamp)
                    .header("x-mxc-sign", &signature.signature);
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
