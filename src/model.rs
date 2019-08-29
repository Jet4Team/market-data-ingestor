#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bids {
    #[serde(with = "string_or_float")] pub price: f64,
    #[serde(with = "string_or_float")] pub qty: f64,

    #[serde(skip)]
    ignore: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asks {
    #[serde(with = "string_or_float")] pub price: f64,
    #[serde(with = "string_or_float")] pub qty: f64,

    #[serde(skip)]
    ignore: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepthOrderBookEvent {
    #[serde(rename = "e")] pub event_type: String,

    #[serde(rename = "E")] pub event_time: u64,

    #[serde(rename = "s")] pub symbol: String,

    #[serde(rename = "U")] pub first_update_id: u64,

    #[serde(rename = "u")] pub final_update_id: u64,

    #[serde(rename = "b")] pub bids: Vec<Bids>,

    #[serde(rename = "a")] pub asks: Vec<Asks>,
}

mod string_or_float {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where T: fmt::Display,
              S: Serializer
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
            StringOrFloat::Float(i) => Ok(i),
        }
    }
}
