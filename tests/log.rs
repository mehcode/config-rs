extern crate config;
extern crate log;

#[macro_use]
extern crate serde_derive;

use config::*;

#[derive(Debug, Deserialize)]
struct Settings {
    log: log::Level,
}

fn config(s: &str) -> Config {
    Config::default()
        .merge(File::from_str(s, FileFormat::Json))
        .unwrap()
        .clone()
}

#[test]
fn test_load_level_uppercase() {
    let s = r#"{ "log": "ERROR" }"#;
    let c = config(s);
    let l = c.get::<log::Level>("log").unwrap();
    assert_eq!(l, log::Level::Error);
}

