extern crate config;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::env;

pub mod producer;
pub mod conf;

pub fn get_env_var(name: &str) -> Result<String, String> {
    match env::var(name) {
        Ok(val) => Ok(val),
        Err(e) => Err(String::from(format!("{} - {}", name, e.to_string()))),
    }
}

