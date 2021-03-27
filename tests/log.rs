extern crate config;
extern crate log;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

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

#[test]
fn test_load_level_lowercase() {
    // to verify that this works
    {
        use std::str::FromStr;
        let l = log::Level::from_str("error").unwrap();
        assert_eq!(l, log::Level::Error);
    }

    let s = r#"{ "log": "error" }"#;

    // to confirm serde_json works as expected
    {
        let j: Settings = serde_json::from_str(s).unwrap();
        assert_eq!(j.log, log::Level::Error);
    }

    let c = config(s);
    assert_eq!(c.get_str("log").unwrap(), "error");
    let l = c.get::<log::Level>("log");
    assert!(l.is_ok(), "Expected Ok(_) for {:?}", l);
    assert_eq!(l.unwrap(), log::Level::Error);
}
