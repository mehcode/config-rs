use ini::Ini;
use source::Source;
use std::collections::HashMap;
use std::error::Error;
use value::{Value, ValueKind};

pub fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<HashMap<String, Value>, Box<dyn Error + Send + Sync>> {
    let mut map: HashMap<String, Value> = HashMap::new();
    let i = Ini::load_from_str(text)?;
    for (sec, prop) in i.iter() {
        match sec {
            Some(sec) => {
                let mut sec_map: HashMap<String, Value> = HashMap::new();
                for (k, v) in prop.iter() {
                    sec_map.insert(
                        k.to_owned(),
                        Value::new(uri, ValueKind::String(v.to_owned())),
                    );
                }
                map.insert(sec.to_owned(), Value::new(uri, ValueKind::Table(sec_map)));
            }
            None => {
                for (k, v) in prop.iter() {
                    map.insert(
                        k.to_owned(),
                        Value::new(uri, ValueKind::String(v.to_owned())),
                    );
                }
            }
        }
    }
    Ok(map)
}
