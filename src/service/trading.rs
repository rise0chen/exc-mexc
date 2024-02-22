use super::Mexc;
use crate::types::order::{FuturesOpenType, Order, OrderSide, OrderType};
use exc_core::{ExchangeError, Symbol};
use tower::ServiceExt;

impl Mexc {
    pub async fn place_order(&mut self, symbol: Symbol, size: f64, price: f64, kind: OrderType) -> Result<String, ExchangeError> {
        let order_id = if let Some((base, quote)) = symbol.as_spot() {
            use crate::spot_api::http::trading::PlaceOrderRequest;
            let req = PlaceOrderRequest {
                symbol: format!("{base}{quote}"),
                side: if size > 0.0 { OrderSide::Buy } else { OrderSide::Sell },
                r#type: kind,
                quantity: size.abs(),
                quote_order_qty: size.abs() * price * 1.05,
                price,
                new_client_order_id: None,
            };
            let resp = self.oneshot(req).await?;
            resp.order_id
        } else {
            use crate::futures_web::http::trading::PlaceOrderRequest;
            let req = PlaceOrderRequest {
                symbol: symbol.as_derivative().map_or(String::new(), |(p, s)| format!("{p}{s}")),
                side: if size > 0.0 { OrderSide::Buy } else { OrderSide::Sell },
                r#type: kind,
                vol: size.abs(),
                price,
                open_type: FuturesOpenType::Isolated,
                leverage: 10.0,
            };
            let resp = self.oneshot(req).await?;
            resp.order_id
        };
        Ok(order_id)
    }
    pub async fn get_order(&mut self, symbol: Symbol, order_id: String) -> Result<Order, ExchangeError> {
        let order = if let Some((base, quote)) = symbol.as_spot() {
            use crate::spot_api::http::trading::GetOrderRequest;
            let req = GetOrderRequest {
                symbol: format!("{base}{quote}"),
                order_id: Some(order_id),
                orig_client_order_id: None,
            };
            let resp = self.oneshot(req).await?;
            Order {
                symbol: resp.symbol,
                order_id: resp.order_id,
                price: resp.price,
                vol: resp.orig_qty,
                deal_vol: resp.executed_qty,
                deal_avg_price: resp.cummulative_quote_qty / resp.executed_qty,
                state: resp.status,
                order_type: resp.r#type,
                side: resp.side,
            }
        } else {
            use crate::futures_api::http::trading::GetOrderRequest;
            let req = GetOrderRequest { order_id };
            let resp = self.oneshot(req).await?;
            Order {
                symbol: resp.symbol,
                order_id: resp.order_id,
                price: resp.price,
                vol: resp.vol,
                deal_vol: resp.deal_vol,
                deal_avg_price: resp.deal_avg_price,
                state: resp.state,
                order_type: resp.order_type,
                side: resp.side,
            }
        };
        Ok(order)
    }
}
