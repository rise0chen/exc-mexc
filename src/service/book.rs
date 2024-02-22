use super::Mexc;
use crate::types::book::BidAsk;
use exc_core::{ExchangeError, Symbol};
use tower::ServiceExt;

impl Mexc {
    pub async fn get_bid_ask(&mut self, symbol: Symbol) -> Result<BidAsk, ExchangeError> {
        let bid_ask = if let Some((base, quote)) = symbol.as_spot() {
            use crate::spot_api::http::book::GetBidAskRequest;
            let req = GetBidAskRequest {
                symbol: format!("{base}{quote}"),
                limit: 1,
            };
            let resp = self.oneshot(req).await?;
            BidAsk {
                bid: (resp.bids[0].0, resp.bids[0].1),
                ask: (resp.asks[0].0, resp.asks[0].1),
                version: resp.last_update_id,
            }
        } else {
            use crate::futures_api::http::book::GetBidAskRequest;
            let req = GetBidAskRequest {
                symbol: symbol.as_derivative().map_or(String::new(), |(p, s)| format!("{p}{s}")),
                limit: 1,
            };
            let resp = self.oneshot(req).await?;
            BidAsk {
                bid: (resp.bids[0].0, resp.bids[0].1),
                ask: (resp.asks[0].0, resp.asks[0].1),
                version: resp.version,
            }
        };
        Ok(bid_ask)
    }
}
