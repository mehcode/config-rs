use std::error::Error;

use ini::Ini;

use crate::map::Map;
use crate::value::{Table, Value, ValueKind};

pub fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    let value = from_ini(uri, Ini::load_from_str(text)?);

    match value.kind {
        ValueKind::Table(map) => Ok(map),

        _ => Ok(Map::new()),
    }
}

fn from_ini(
    uri: Option<&String>,
    data: Ini,
) -> Value {
    let mut map = Map::new();

    let mut sections: Map<Option<&str>, Table> = data.into_iter().map(|(section, props)| {(
        section,
        props.iter().map(|(k, v)| {(
            k.to_owned(),
            Value::new(uri, ValueKind::String(v.to_owned())),
        )}).collect()
    )}).collect();

    // These (optional) properties should exist top-level alongside sections:
    if let Some(sectionless) = sections.remove(&None) {
        map.extend(sectionless);
    }

    // Wrap each section Table into Value for merging into `map`:
    map.extend(sections.into_iter().map(|(k,v)| {(
        k.unwrap_or_default().to_owned(),
        Value::new(uri, ValueKind::Table(v)),
    )}));

    Value::new(uri, ValueKind::Table(map))
}
