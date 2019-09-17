extern crate rdkafka;
extern crate futures;

use self::rdkafka::producer::{FutureProducer, FutureRecord};
use self::rdkafka::config::ClientConfig;
use self::futures::{Future};

const KAFKA_KEY: &str = "Binance";

pub struct Producer {
    topic: String,
    producer: FutureProducer,
}

#[derive(Debug)]
pub struct ProducerError(String);

impl Producer {
    pub fn new(brokers: String, topic: String) -> Producer {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &brokers)
            .set("produce.offset.report", "true")
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");

        Producer {
            producer,
            topic,
        }
    }

    pub fn send(&self, payload: String) -> impl Future<Item = (), Error = ProducerError> {
        // clone the producer as it will be executed in a thread pool
        let producer = self.producer.clone();

        producer.send(
            FutureRecord::to(&self.topic)
                .key(KAFKA_KEY)
                .payload(&payload),
            0)
            .then(|result| {
                match result {
                    Ok(Ok(_)) => Ok(()),
                    Ok(Err((e, _))) => Err(ProducerError(e.to_string())),
                    Err(_) => Ok(()),
                }
            })
    }
}
