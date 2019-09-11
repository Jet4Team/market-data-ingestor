extern crate binance_ingestor;
extern crate futures;

use binance_ingestor::ws::WSStream;
use binance_ingestor::producer::Producer;
use futures::{Future, Stream};

const KAFKA_BROKERS: &str = "127.0.0.1:9092";
const KAFKA_TOPIC: &str = "market-data-feed";

fn main() {
    let ws = WSStream::new("btcusdt");
    let producer = Producer::new(KAFKA_BROKERS.to_string(), KAFKA_TOPIC.to_string());

    let stream = ws
        .for_each(move |message| {
            //println!("{:?}", message);

            let producer_future = producer.send(message)
                .map_err(|e| println!("Producer error occurred: {:?}", e));

            // executing in the tokio thread pool
            tokio::spawn(producer_future);

            Ok(())
        })
        .map_err(|e| println!("Websocket error occurred: {:?}", e));

    tokio::run(stream);
}
