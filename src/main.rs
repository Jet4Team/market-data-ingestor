extern crate tokio_tungstenite;
extern crate tokio;
extern crate futures;
extern crate url;
extern crate rdkafka;

use tokio_tungstenite::{connect_async, PeerAddr};
use futures::{Future, Stream};
use tokio_tungstenite::tungstenite::protocol::Message;

use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::config::ClientConfig;

const KAFKA_TOPIC: &str = "market-data-feed";
const KAFKA_BROKERS: &str = "127.0.0.1:9092";
const BINANCE_WEBSOCKET: &str = "wss://stream.binance.com:9443/ws";
const EXCHANGE_NAME: &str = "Binance";

fn handle_message(producer: FutureProducer, msg: &str) {
    println!("{:?}", msg);

    let producer_future = producer.send(
        FutureRecord::to(&KAFKA_TOPIC.to_string())
            .key(&EXCHANGE_NAME.to_string())
            .payload(&msg.to_string()),
        0)
        .then(|result| {
            match result {
                Ok(Ok(delivery)) => println!("Sent: {:?}", delivery),
                Ok(Err((e, _))) => println!("Error: {:?}", e),
                Err(_) => println!("Future cancelled")
            }
            Ok(())
        });

    // executing in the tokio thread pool
    tokio::spawn(producer_future);
}

fn main() {
    let connect_addr = format!("{}/btcusdt@depth@100ms", &BINANCE_WEBSOCKET);
    let url = url::Url::parse(&connect_addr).unwrap();

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &KAFKA_BROKERS)
        .set("produce.offset.report", "true")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let client = connect_async(url)
        .and_then(move |(ws_stream, _)| {
            println!("WebSocket handshake has been successfully completed");

            let addr = ws_stream
                .peer_addr()
                .expect("connected streams should have a peer address");
            println!("Peer address: {}", addr);

            // `sink` is the stream of messages going out.
            // `stream` is the stream of incoming messages.
            let (_, stream) = ws_stream.split();

            stream.for_each(move |message| {

                // clone the producer
                let producer = producer.clone();

                match message {
                    Message::Text(str) => handle_message(producer, &str),
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
