extern crate tokio_tungstenite;
extern crate tokio;
extern crate futures;
extern crate url;

use tokio_tungstenite::{connect_async, PeerAddr};
use futures::{Future, Stream};
use tokio_tungstenite::tungstenite::protocol::Message;

fn handle_message(msg: &str) {
    println!("{:?}", msg);
}

fn main() {
    let connect_addr = "wss://stream.binance.com:9443/ws/btcusdt@depth@100ms";
    let url = url::Url::parse(&connect_addr).unwrap();

    let client = connect_async(url)
        .and_then(|(ws_stream, _)| {
            println!("WebSocket handshake has been successfully completed");

            let addr = ws_stream
                .peer_addr()
                .expect("connected streams should have a peer address");
            println!("Peer address: {}", addr);

            // `sink` is the stream of messages going out.
            // `stream` is the stream of incoming messages.
            let (_, stream) = ws_stream.split();

            stream.for_each(|message| {
                match message {
                    Message::Text(str) => handle_message(&str),
                    Message::Ping(_) => println!("ping"),
                    Message::Close(_) => println!("connection closed"),
                    _ => (),
                }

                Ok(())
            })
        })
        .map_err(|e| {
            println!("Error occurred: {}", e);
        });

    tokio::run(client);
}
