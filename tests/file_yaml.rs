#![cfg(feature = "yaml")]

use serde_derive::Deserialize;

use std::path::PathBuf;

use config::{Config, File, FileFormat, Map, Value};
use float_cmp::ApproxEqUlps;

#[derive(Debug, Deserialize)]
struct Place {
    name: String,
    longitude: f64,
    latitude: f64,
    favorite: bool,
    telephone: Option<String>,
    reviews: u64,
    creator: Map<String, Value>,
    rating: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct Settings {
    debug: f64,
    production: Option<String>,
    place: Place,
    #[serde(rename = "arr")]
    elements: Vec<String>,
    #[serde(rename = "192.168.1.1")]
    ip_key: String,
}

fn make() -> Config {
    Config::builder()
        .add_source(File::new("tests/Settings", FileFormat::Yaml))
        .build()
        .unwrap()
}

#[test]
fn test_keys_with_periods_deserialize_serde_yaml() {
    let map = "192.168.1.1: a string value";

    let c: HashMap<String, String> = serde_yaml::from_str(map).unwrap();

    assert_eq!(c.get("192.168.1.1").unwrap(), "a string value");
}

#[test]
fn test_keys_with_periods_deserialize_yaml_rust() {
    use yaml_rust::YamlLoader;

    let map = "192.168.1.1: a string value";

    let c = YamlLoader::load_from_str(map).unwrap().first().unwrap().clone();

    assert_eq!(c["192.168.1.1"].as_str().unwrap(), "a string value");
}

#[test]
fn test_keys_with_periods_deserialize() {
    let c = make();

    let s: Settings = c.try_deserialize().unwrap();

    assert_eq!(s.ip_key, "a string value");
}

#[test]
fn test_file() {
    let c = make();

    // Deserialize the entire file as single struct
    let s: Settings = c.try_deserialize().unwrap();

    assert!(s.debug.approx_eq_ulps(&1.0, 2));
    assert_eq!(s.production, Some("false".to_string()));
    assert_eq!(s.place.name, "Torre di Pisa");
    assert!(s.place.longitude.approx_eq_ulps(&43.722_498_5, 2));
    assert!(s.place.latitude.approx_eq_ulps(&10.397_052_2, 2));
    assert!(!s.place.favorite);
    assert_eq!(s.place.reviews, 3866);
    assert_eq!(s.place.rating, Some(4.5));
    assert_eq!(s.place.telephone, None);
    assert_eq!(s.elements.len(), 10);
    assert_eq!(s.elements[3], "4".to_string());
    if cfg!(feature = "preserve_order") {
        assert_eq!(
            s.place
                .creator
                .into_iter()
                .collect::<Vec<(String, config::Value)>>(),
            vec![
                ("name".to_string(), "John Smith".into()),
                ("username".into(), "jsmith".into()),
                ("email".into(), "jsmith@localhost".into()),
            ]
        );
    } else {
        assert_eq!(
            s.place.creator["name"].clone().into_string().unwrap(),
            "John Smith".to_string()
        );
    }
}

#[test]
fn test_error_parse() {
    let res = Config::builder()
        .add_source(File::new("tests/Settings-invalid", FileFormat::Yaml))
        .build();

    let path_with_extension: PathBuf = ["tests", "Settings-invalid.yaml"].iter().collect();

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        format!(
            "while parsing a block mapping, did not find expected key at \
         line 2 column 1 in {}",
            path_with_extension.display()
        )
    );
}

use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Outer {
    inner_string: HashMap<String, Inner>,
    inner_int: HashMap<u32, Inner>,
}

#[derive(Debug, Deserialize)]
struct Inner {
    member: String,
}

#[test]
fn test_yaml_parsing_key() {
    let config = Config::builder()
        .add_source(File::new("tests/test-keys.yaml", FileFormat::Yaml))
        .build()
        .unwrap()
        .try_deserialize::<Outer>()
        .unwrap();
    assert_eq!(config.inner_int.get(&1).unwrap().member, "Test Int 1");
    assert_eq!(config.inner_int.get(&2).unwrap().member, "Test Int 2");
    assert_eq!(
        config.inner_string.get("str_key").unwrap().member,
        "Test String"
    );
}

#[derive(Debug, Deserialize, PartialEq)]
enum EnumSettings {
    Bar(String),
}

#[derive(Debug, Deserialize, PartialEq)]
struct StructSettings {
    foo: String,
    bar: String,
}
#[derive(Debug, Deserialize, PartialEq)]
#[allow(non_snake_case)]
struct CapSettings {
    FOO: String,
}

#[test]
fn test_override_uppercase_value_for_struct() {
    std::env::set_var("APP_FOO", "I HAVE BEEN OVERRIDDEN_WITH_UPPER_CASE");

    let cfg = Config::builder()
        .add_source(File::new("tests/Settings.yaml", FileFormat::Yaml))
        .add_source(config::Environment::with_prefix("APP").separator("_"))
        .build()
        .unwrap();

    let cap_settings = cfg.clone().try_deserialize::<CapSettings>();
    let lower_settings = cfg.try_deserialize::<StructSettings>().unwrap();

    match cap_settings {
        Ok(v) => {
            // this assertion will ensure that the map has only lowercase keys
            assert_ne!(v.FOO, "FOO should be overridden");
            assert_eq!(
                lower_settings.foo,
                "I HAVE BEEN OVERRIDDEN_WITH_UPPER_CASE".to_string()
            );
        }
        Err(e) => {
            if e.to_string().contains("missing field `FOO`") {
                println!("triggered error {:?}", e);
                assert_eq!(
                    lower_settings.foo,
                    "I HAVE BEEN OVERRIDDEN_WITH_UPPER_CASE".to_string()
                );
            } else {
                panic!("{}", e);
            }
        }
    }
}

#[test]
fn test_override_lowercase_value_for_struct() {
    std::env::set_var("config_bar", "I have been overridden_with_lower_case");

    let cfg = Config::builder()
        .add_source(File::new("tests/Settings.yaml", FileFormat::Yaml))
        .add_source(config::Environment::with_prefix("config").separator("_"))
        .build()
        .unwrap();

    let values: StructSettings = cfg.try_deserialize().unwrap();
    assert_eq!(
        values.bar,
        "I have been overridden_with_lower_case".to_string()
    );
    assert_ne!(values.bar, "I am bar".to_string());
}

#[test]
fn test_override_uppercase_value_for_enums() {
    std::env::set_var("APPS_BAR", "I HAVE BEEN OVERRIDDEN_WITH_UPPER_CASE");

    let cfg = Config::builder()
        .add_source(File::new("tests/Settings-enum-test.yaml", FileFormat::Yaml))
        .add_source(config::Environment::with_prefix("APPS").separator("_"))
        .build()
        .unwrap();
    let values: EnumSettings = cfg.try_deserialize().unwrap();

    assert_eq!(
        values,
        EnumSettings::Bar("I HAVE BEEN OVERRIDDEN_WITH_UPPER_CASE".to_string())
    );
}

#[test]
fn test_override_lowercase_value_for_enums() {
    std::env::set_var("test_bar", "I have been overridden_with_lower_case");

    let cfg = Config::builder()
        .add_source(File::new("tests/Settings-enum-test.yaml", FileFormat::Yaml))
        .add_source(config::Environment::with_prefix("test").separator("_"))
        .build()
        .unwrap();

    let values: EnumSettings = cfg.try_deserialize().unwrap();

    assert_eq!(
        values,
        EnumSettings::Bar("I have been overridden_with_lower_case".to_string())
    );
}
