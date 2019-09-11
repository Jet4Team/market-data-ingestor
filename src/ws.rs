extern crate tokio_tungstenite;
extern crate tokio;
extern crate futures;
extern crate url;

use self::tokio_tungstenite::connect_async;
use self::tokio_tungstenite::tungstenite::protocol::Message as TMessage;
use self::futures::{Future, Stream};
use self::tokio_tungstenite::tungstenite::error::Error;

const WS_URL: &str = "wss://stream.binance.com:9443/ws";

#[derive(Debug)]
pub struct WSError(Error);

pub type WSMessage = String;

pub struct WSStream;

impl WSStream {
    pub fn new(pair: &str) -> impl Stream<Item = WSMessage, Error = WSError> {
        let connect_addr = format!("{}/{}@depth@100ms", WS_URL, pair);
        let url = url::Url::parse(&connect_addr).unwrap();

        connect_async(url)
            // wrapping connection errors
            .map_err(|e| WSError(e))
            .and_then(|(ws_stream, _)| {

                let (_, stream) = ws_stream.split();
                let stream = stream
                    // filtering out ping and other system messages
                    .filter(|msg| msg.is_text())
                    // wrapping errors with our own error type
                    .map_err(|e| WSError(e))
                    // wrapping messages with our own message type
                    .map(convert_msg);

                Ok(stream)
            })
            // converting future of stream to stream
            .flatten_stream()
    }
}

fn convert_msg(msg: TMessage) -> WSMessage {
    match msg {
        TMessage::Text(str) =>  str,
        _ => unreachable!(), // filtered in stream
    }
}
