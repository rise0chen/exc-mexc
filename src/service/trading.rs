use super::Mexc;
use crate::types::order::{FuturesOpenType, Order, OrderId, OrderSide, OrderType};
use exc_core::{ExchangeError, Symbol};
use tower::ServiceExt;

impl Mexc {
    pub async fn place_order(&mut self, symbol: &Symbol, size: f64, price: f64, kind: OrderType) -> Result<OrderId, (OrderId, ExchangeError)> {
        let custom_id = format!(
            "{:08x?}{:08x?}{:016x?}",
            (price as f32).ln().to_bits(),
            (size as f32).ln().to_bits(),
            time::OffsetDateTime::now_utc().unix_timestamp_nanos() as u64
        );
        let mut ret = OrderId {
            symbol: symbol.clone(),
            order_id: None,
            custom_order_id: Some(custom_id.clone()),
        };
        let order_id = if let Some((base, quote)) = symbol.as_spot() {
            use crate::spot_api::http::trading::PlaceOrderRequest;
            let req = PlaceOrderRequest {
                symbol: format!("{base}{quote}"),
                side: if size > 0.0 { OrderSide::Buy } else { OrderSide::Sell },
                r#type: kind,
                quantity: size.abs(),
                quote_order_qty: size.abs() * price * 1.05,
                price,
                new_client_order_id: Some(custom_id),
            };
            self.oneshot(req).await.map(|resp| resp.order_id)
        } else {
            use crate::futures_web::http::trading::PlaceOrderRequest;
            let req = PlaceOrderRequest {
                symbol: symbol.as_derivative().map_or(String::new(), |(p, s)| format!("{p}{s}")),
                external_oid: Some(custom_id),
                side: if size > 0.0 { OrderSide::Buy } else { OrderSide::Sell },
                r#type: kind,
                vol: size.abs(),
                price,
                open_type: FuturesOpenType::Isolated,
                leverage: 10.0,
            };
            self.oneshot(req).await.map(|resp| resp.order_id)
        };
        match order_id {
            Ok(id) => {
                ret.order_id = Some(id);
                Ok(ret)
            }
            Err(e) => Err((ret, e)),
        }
    }
    pub async fn get_order(&mut self, order_id: OrderId) -> Result<Order, ExchangeError> {
        let OrderId {
            symbol,
            order_id,
            custom_order_id,
        } = order_id;
        let order = if let Some((base, quote)) = symbol.as_spot() {
            use crate::spot_api::http::trading::GetOrderRequest;
            let req = GetOrderRequest {
                symbol: format!("{base}{quote}"),
                order_id,
                orig_client_order_id: custom_order_id,
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
        } else if let Some((_prefix, symbol)) = symbol.as_derivative() {
            use crate::futures_api::http::trading::GetOrderRequest;
            let req = GetOrderRequest {
                symbol: symbol.to_owned(),
                order_id,
                external_oid: custom_order_id,
            };
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
        } else {
            return Err(ExchangeError::OrderNotFound);
        };
        Ok(order)
    }
}
