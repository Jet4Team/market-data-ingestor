extern crate futures;

use ws::WSStream;
use producer::Producer;
use futures::{Future, Stream};
use std::env;

pub mod ws;
pub mod producer;

fn get_env_var(name: &str) -> Result<String, String> {
    match env::var(name) {
        Ok(val) => Ok(val),
        Err(e) => Err(String::from(format!("{} - {}", name, e.to_string()))),
    }
}

pub fn run() -> Result<(), String> {

    let kafka_brokers = get_env_var("KAFKA_BROKERS")?;
    let kafka_topic = get_env_var("KAFKA_TOPIC")?;

    let ws = WSStream::new("btcusdt");
    let producer = Producer::new(kafka_brokers, kafka_topic);

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
