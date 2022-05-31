#![allow(dead_code, unused)]
use std::collections::HashMap;

use serde::Deserialize;
use yaml_rust::YamlLoader;
use yaml_rust::Yaml;

use config::Config;

#[derive(Debug, Deserialize)]
struct Outer {
    inner_string: HashMap<String, Inner>,
    inner_int: HashMap<u32, Inner>,
}

#[derive(Debug, Deserialize)]
struct Inner {
    member: String,
}

const CONFIG: &str = r#"
inner_int:
  "1":
    member: "Test Int 1"
  2:
    member: "Test Int 2"
inner_string:
  stuff:
    member: "Test String"
"#;

#[test]
fn test_int_key() {
    let config = Config::builder()
        .add_source(config::File::from_str(CONFIG, config::FileFormat::Yaml))
        .build()
        .unwrap();

    let outer: Outer = config.try_deserialize().unwrap();
    assert_eq!(outer.inner_string.get("1").unwrap().member, "Test Int 1");
    assert_eq!(outer.inner_int.get(&2).unwrap().member, "Test Int 2");
}

#[test]
fn test_yaml_parsing_int_key() {
    let mut doc = YamlLoader::load_from_str(CONFIG).unwrap();
    assert!(std::matches!(doc[0], Yaml::Hash(_)));
    let hash = doc.get(0).unwrap().as_hash();
    let hash = hash.unwrap();

    let inner_int_map = hash.get(&Yaml::String("inner_int".to_string()));
    assert!(inner_int_map.is_some());
    let inner_int_map = inner_int_map.unwrap();
    assert!(std::matches!(inner_int_map, Yaml::Hash(_)));
    let inner_int_map = inner_int_map.as_hash().unwrap();

    let int_hash = inner_int_map.get(&Yaml::Integer(2));
    assert!(int_hash.is_some());
    let int_hash = int_hash.unwrap();

    assert!(std::matches!(int_hash, Yaml::Hash(_)));
    let int_hash = int_hash.as_hash().unwrap();

    let member = int_hash.get(&Yaml::String("member".to_string()));
    assert!(member.is_some());
    let member = member.unwrap();

    assert_eq!(member, &Yaml::String("Test Int 2".to_string()));
}

#[test]
fn test_yaml_parsing_string_key() {
    let mut doc = YamlLoader::load_from_str(CONFIG).unwrap();
    assert!(std::matches!(doc[0], Yaml::Hash(_)));
    let hash = doc.get(0).unwrap().as_hash();
    let hash = hash.unwrap();

    let inner_string_map = hash.get(&Yaml::String("inner_string".to_string()));
    assert!(inner_string_map.is_some());
    let inner_string_map = inner_string_map.unwrap();
    assert!(std::matches!(inner_string_map, Yaml::Hash(_)));
    let inner_string_map = inner_string_map.as_hash().unwrap();

    let int_hash = inner_string_map.get(&Yaml::String("1".to_string()));
    assert!(int_hash.is_some());
    let int_hash = int_hash.unwrap();

    assert!(std::matches!(int_hash, Yaml::Hash(_)));
    let int_hash = int_hash.as_hash().unwrap();

    let member = int_hash.get(&Yaml::String("member".to_string()));
    assert!(member.is_some());
    let member = member.unwrap();

    assert_eq!(member, &Yaml::String("Test Int 1".to_string()));
}
