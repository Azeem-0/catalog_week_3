use serde::{Deserialize, Deserializer};
use std::str::FromStr;

// Custom function to handle deserialization from string to i64
pub fn deserialize_string_to_number<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Default,
    T::Err: std::fmt::Display,
{
    let s = String::deserialize(deserializer)?; // Deserialize to string first
    s.parse::<T>().map_err(serde::de::Error::custom) // Attempt to parse to i64 or f64
}
