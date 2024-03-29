[![Build Status](https://travis-ci.com/jetl2/market-data-ingestor.svg?token=GHtekntqUN4NMBayeZ7i&branch=master)](https://travis-ci.com/jetl2/market-data-ingestor)

# Binance exchange ingestor
L2 data ingestor from Binance exchange https://www.binance.com

## Libs:
* https://github.com/wisespace-io/binance-rs
* https://github.com/inv2004/coinbase-pro-rs
* https://github.com/snapview/tungstenite-rs

## Contribution rules
* Use https://github.com/rust-lang/rustfmt
* Use .editorconfig https://editorconfig.org
    * Install EditorConfig with Package Control and restart Sublime

## Local environment
### Kafka
* use .env/kafka/docker-compose.yml
* run `docker-compose up -d` in /kafka directory or specify docker-compose file via `-f` option
* Running kafka-docker on a Mac: Install the Docker Toolbox and set `KAFKA_ADVERTISED_HOST_NAME` to the IP that is returned by the `docker-machine ip` command.

## Environment Variables
The service should be properly configured with following environment variables.

Key | Value | Description
:-- | :-- | :--
KAFKA_BROKERS | 127.0.0.1:9092 | Comma separated list of Kafka broker addresses.
KAFKA_TOPIC | market-data-feed | Kafka topic name.
