#![cfg(feature = "toml")]

extern crate config;

use config::*;

fn make() -> Config {
    let mut builder = Config::builder();
    builder
        .add_source(File::new("tests/Settings", FileFormat::Toml))
        .add_source(File::new("tests/Settings-production", FileFormat::Toml));
    builder.build().unwrap()
}

#[test]
fn test_merge() {
    let c = make();

    assert_eq!(c.get("debug").ok(), Some(false));
    assert_eq!(c.get("production").ok(), Some(true));
    assert_eq!(
        c.get("place.creator.name").ok(),
        Some("Somebody New".to_string())
    );
    assert_eq!(c.get("place.rating").ok(), Some(4.9));
}

#[test]
fn test_merge_whole_config() {
    let mut builder1 = Config::builder();
    let mut builder2 = Config::builder();

    builder1.set_override("x", 10).unwrap();
    builder2.set_override("y", 25).unwrap();

    let config1 = builder1.build_cloned().unwrap();
    let config2 = builder2.build_cloned().unwrap();

    assert_eq!(config1.get("x").ok(), Some(10));
    assert_eq!(config2.get::<()>("x").ok(), None);

    assert_eq!(config2.get("y").ok(), Some(25));
    assert_eq!(config1.get::<()>("y").ok(), None);

    builder1.add_source(config2);

    let config3 = builder1.build().unwrap();

    assert_eq!(config3.get("x").ok(), Some(10));
    assert_eq!(config3.get("y").ok(), Some(25));
}
