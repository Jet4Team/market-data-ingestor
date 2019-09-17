extern crate tokio_tungstenite;
extern crate futures;

use futures::{Future, Stream};

mod wsfeed;
use wsfeed::WSStream;

pub fn run() -> Result<(), String> {

    let kafka_brokers = common::get_env_var("KAFKA_BROKERS")?;
    let kafka_topic = common::get_env_var("KAFKA_TOPIC")?;

    let ws = WSStream::new("btcusdt");
    let producer = common::producer::Producer::new(kafka_brokers, kafka_topic);

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

    Ok(())
}
