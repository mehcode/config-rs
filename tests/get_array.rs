extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use config::*;

#[derive(Debug, Deserialize)]
struct Settings {
    #[serde(rename = "arr")]
    elements: Vec<String>,
}

fn make() -> Config {
    let mut c = Config::default();
    c.merge(File::new("tests/Settings", FileFormat::Toml))
        .unwrap();

    c
}

#[test]
fn test_array_scalar() {
    let c = make();
    let arr: Vec<i64> = c.get("arr").unwrap();

    assert_eq!(arr.len(), 10);
    assert_eq!(arr[3], 4);
}

#[test]
fn test_struct_array() {
    let c = make();
    let s: Settings = c.deserialize().unwrap();

    assert_eq!(s.elements.len(), 10);
    assert_eq!(s.elements[3], "4".to_string());
}
