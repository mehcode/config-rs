use std::error::Error;

use ini::Ini;

use crate::map::Map;
use crate::value::{Value, ValueKind};

pub fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    let mut map: Map<String, Value> = Map::new();
    let i = Ini::load_from_str(text)?;
    for (sec, prop) in i.iter() {
        match sec {
            Some(sec) => {
                let mut sec_map: Map<String, Value> = Map::new();
                for (k, v) in prop.iter() {
                    sec_map.insert(k.to_owned(), Value::new(uri, try_parse(v)));
                }
                map.insert(sec.to_owned(), Value::new(uri, ValueKind::Table(sec_map)));
            }
            None => {
                for (k, v) in prop.iter() {
                    map.insert(k.to_owned(), Value::new(uri, try_parse(v)));
                }
            }
        }
    }
    Ok(map)
}

fn try_parse(s: &str) -> ValueKind {
    if let Ok(parsed) = s.parse::<bool>() {
        return ValueKind::Boolean(parsed);
    }
    if let Ok(parsed) = s.parse::<i64>() {
        return ValueKind::I64(parsed);
    }
    if let Ok(parsed) = s.parse::<f64>() {
        return ValueKind::Float(parsed);
    }
    ValueKind::String(s.to_string())
}
