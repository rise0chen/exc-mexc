//! Exc-mexc: Mexc exchange services.

#[macro_use]
extern crate tracing;

pub mod futures_api;
pub mod futures_web;
pub mod interface;
pub mod key;
pub mod response;
pub mod spot_api;
pub mod types;

cfg_if::cfg_if! {
    if #[cfg(any(feature = "rustls-tls", feature = "native-tls"))] {
        /// Exchange.
        //pub mod exchange;
        pub mod service;

        //pub use exchange::MexcExchange;
    } else {
        compile_error!("Either feature 'rustls-tls' or 'native-tls' must be enabled");
    }
}
