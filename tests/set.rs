extern crate config;

use config::*;

#[test]
fn test_set_scalar() {
    let mut c = Config::default();

    c.set("value", true).unwrap();

    assert_eq!(c.get("value").ok(), Some(true));
}

#[test]
fn test_set_scalar_default() {
    let mut c = Config::default();

    c.merge(File::new("tests/Settings", FileFormat::Toml))
        .unwrap();

    c.set_default("debug", false).unwrap();
    c.set_default("staging", false).unwrap();

    assert_eq!(c.get("debug").ok(), Some(true));
    assert_eq!(c.get("staging").ok(), Some(false));
}

#[test]
fn test_set_scalar_path() {
    let mut c = Config::default();

    c.merge(File::new("tests/Settings", FileFormat::Toml))
        .unwrap();

    c.set_default("place.favorite", true).unwrap();
    c.set_default("place.blocked", true).unwrap();

    assert_eq!(c.get("place.favorite").ok(), Some(false));
    assert_eq!(c.get("place.blocked").ok(), Some(true));
}
