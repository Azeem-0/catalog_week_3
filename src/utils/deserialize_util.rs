use serde::{Deserialize, Deserializer};
use serde_json::Value;

// Custom function to handle deserialization from string to f64
pub fn deserialize_string_to_number<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize to a serde_json::Value first to check its type
    let value: Value = Value::deserialize(deserializer)?;

    match value {
        // If it's a string, attempt to parse it as f64
        Value::String(s) => s.parse::<f64>().map_err(serde::de::Error::custom),
        // If it's a number (either integer or floating-point), handle accordingly
        Value::Number(num) => {
            if let Some(f) = num.as_f64() {
                Ok(f) // Return f64 as is
            } else {
                // Handle unexpected number type
                Err(serde::de::Error::custom("Expected a valid number"))
            }
        }
        // Handle unexpected types
        _ => Err(serde::de::Error::custom("Expected a string or number")),
    }
}
