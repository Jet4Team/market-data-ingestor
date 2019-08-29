extern crate chrono;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate tokio;
extern crate tokio_tungstenite;
extern crate uuid;

pub mod model;

pub const WS_URL: &str = "wss://stream.binance.com:9443/ws/";