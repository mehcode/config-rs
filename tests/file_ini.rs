#![cfg(feature = "ini")]

extern crate config;
extern crate float_cmp;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use config::*;

#[derive(Debug, Deserialize, PartialEq)]
struct Place {
    name: String,
    longitude: f64,
    latitude: f64,
    favorite: bool,
    reviews: u64,
    rating: Option<f32>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Settings {
    debug: f64,
    place: Place,
}

fn make() -> Config {
    let mut c = Config::default();
    c.merge(File::new("tests/Settings", FileFormat::Ini))
        .unwrap();
    c
}

#[test]
fn test_file() {
    let c = make();
    let s: Settings = c.try_into().unwrap();
    assert_eq!(
        s,
        Settings {
            debug: 1.0,
            place: Place {
                name: String::from("Torre di Pisa"),
                longitude: 43.7224985,
                latitude: 10.3970522,
                favorite: false,
                reviews: 3866,
                rating: Some(4.5),
            },
        }
    );
}

#[test]
fn test_error_parse() {
    let mut c = Config::default();
    let res = c.merge(File::new("tests/Settings-invalid", FileFormat::Ini));

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        r#"2:0 Expecting "[Some('='), Some(':')]" but found EOF. in tests/Settings-invalid.ini"#
    );
}



#[cfg(feature = "with_env_vars")]
use std::env;
use std::collections::HashMap;

#[test]
#[cfg(feature = "with_env_vars")]
fn test_config_with_envs() {
    env::set_var("famous_tower", "Torre di Pisa");
    let mut c = Config::default();
    c.merge(File::with_name("tests/Settings-with-envs"))
        .unwrap();
    let c  = c.collect().unwrap();
    let k = c.get("debug").unwrap().clone();
    assert_eq!(k.into_bool().ok(), Some(true));
    let k = c.get("production").unwrap().clone();
    assert_eq!(k.into_bool().ok(), Some(false));
    let m: HashMap<String, Value> = c.get("place").unwrap().clone().into_table().unwrap();
    assert_eq!(
        m["name"].clone().into_str().unwrap(),
        "Torre di Pisa".to_string()
    );
    env::remove_var("famous_tower");
}




#[test]
#[cfg(feature = "with_env_vars")]
fn test_config_with_invalid_envs() {
    env::set_var("famous&_tower", "Torre di Pisa");
    env::set_var("long", "Torre di Pisa");
    env::set_var("lat++", "Torre di Pisa");
    env::set_var("bool!echo", "Torre di Pisa");

    let mut c = Config::default();
    c.merge(File::with_name("tests/Settings-with-invalid-envs"))
        .unwrap();
    let c  = c.collect().unwrap();
    let k = c.get("debug").unwrap().clone();
    assert_eq!(k.into_bool().ok(), Some(true));
    let k = c.get("production").unwrap().clone();
    assert_eq!(k.into_bool().ok(), Some(false));
    let m: HashMap<String, Value> = c.get("place").unwrap().clone().into_table().unwrap();
    assert_eq!(
        m["name"].clone().into_str().unwrap(),
        "${famous&_tower}".to_string()
    );
    assert_eq!(
        m["longitude"].clone().into_str().unwrap(),
        "${long==}".to_string()
    );
    assert_eq!(
        m["latitude"].clone().into_str().unwrap(),
        "${lat++}".to_string()
    );

    assert_eq!(
        m["favorite"].clone().into_str().unwrap(),
        "${bool!echo}".to_string()
    );

    env::remove_var("famous&_tower");
    env::remove_var("long");
    env::remove_var("lat++");
    env::remove_var("bool!echo");
}
