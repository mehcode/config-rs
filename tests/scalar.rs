extern crate config;

use config::*;

fn make() -> Config {
    let mut c = Config::default();
    c.merge(File::new("tests/Settings", FileFormat::Toml))
        .unwrap();

    c
}

#[test]
fn test_scalar() {
    let c = make();

    assert!(c.get("debug").ok() == Some(true));
    assert!(c.get("production").ok() == Some(false));
}

#[test]
fn test_scalar_type_loose() {
    let c = make();

    assert!(c.get("debug").ok() == Some(true));
    assert!(c.get("debug").ok() == Some("true".to_string()));
    assert!(c.get("debug").ok() == Some(1));
    assert!(c.get("debug").ok() == Some(1.0));

    assert!(c.get("debug_s").ok() == Some(true));
    assert!(c.get("debug_s").ok() == Some("true".to_string()));
    assert!(c.get("debug_s").ok() == Some(1));
    assert!(c.get("debug_s").ok() == Some(1.0));

    assert!(c.get("production").ok() == Some(false));
    assert!(c.get("production").ok() == Some("false".to_string()));
    assert!(c.get("production").ok() == Some(0));
    assert!(c.get("production").ok() == Some(0.0));

    assert!(c.get("production_s").ok() == Some(false));
    assert!(c.get("production_s").ok() == Some("false".to_string()));
    assert!(c.get("production_s").ok() == Some(0));
    assert!(c.get("production_s").ok() == Some(0.0));
}
