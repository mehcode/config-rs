extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use config::*;

#[derive(Debug, Deserialize)]
struct Settings {
    place: HashMap<String, Value>,
}

fn make() -> Config {
    let mut c = Config::default();
    c.merge(File::new("tests/Settings", FileFormat::Toml))
        .unwrap();

    c
}

#[test]
fn test_map() {
    let c = make();
    let m: HashMap<String, Value> = c.get("place").unwrap();

    assert_eq!(m.len(), 7);
    assert_eq!(m["name"].clone().into_str().unwrap(), "Torre di Pisa".to_string());
    assert_eq!(m["reviews"].clone().into_int().unwrap(), 3866);
}

#[test]
fn test_map_str() {
    let c = make();
    let m: HashMap<String, String> = c.get("place.creator").unwrap();

    assert_eq!(m.len(), 1);
    assert_eq!(m["name"], "John Smith".to_string());
}

#[test]
fn test_map_struct() {
    let c = make();
    let s: Settings = c.deserialize().unwrap();

    assert_eq!(s.place.len(), 7);
    assert_eq!(s.place["name"].clone().into_str().unwrap(), "Torre di Pisa".to_string());
    assert_eq!(s.place["reviews"].clone().into_int().unwrap(), 3866);
}
