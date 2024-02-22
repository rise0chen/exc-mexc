use exc_core::{Asset, Symbol};
use exc_mexc::service::Mexc;
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_mexc=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("MEXC_KEY")?)?;
    let mut mexc = Mexc::new(key);

    let symbol = Symbol::spot(&Asset::try_from("MX").unwrap(), &Asset::usdt());
    let bid_ask = mexc.get_bid_ask(symbol.clone()).await.unwrap();
    tracing::info!("{:?}", bid_ask);

    let symbol = Symbol::derivative("", "MX_USDT").unwrap();
    let bid_ask = mexc.get_bid_ask(symbol.clone()).await.unwrap();
    tracing::info!("{:?}", bid_ask);
    Ok(())
}
