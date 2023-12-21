use std::error::Error;

use crate::format;
use crate::map::Map;
use crate::value::Value;

pub fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    // Parse a Gura input from the provided text
    let value = format::from_parsed_value(uri, serde_gura::from_str(text)?);
    format::extract_root_table(uri, value)
}
