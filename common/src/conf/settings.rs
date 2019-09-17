use std::env;
use config::{ConfigError, Config, File, Environment};

#[derive(Debug, Deserialize)]
struct Kafka {
    brokers: Vec<String>, // fmt: [host:port, ..., host:port]
    topic: String,
}

#[derive(Debug, Deserialize)]
struct Cloud {
    s3: String,
}

#[derive(Debug, Deserialize)]
struct Exchange {
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    debug: bool,
    kafka: Kafka,
    cloud: Cloud,
    exchange: Exchange,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/default"))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let env = env::var("RUN_MODE").unwrap_or("development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("config/local").required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app"))?;

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", s.get_bool("debug"));
        println!("database: {:?}", s.get::<String>("kafka.brokers"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
