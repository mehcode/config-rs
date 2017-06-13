extern crate config;

use config::*;

fn make() -> Config {
    let mut c = Config::default();
    c.merge(File::new("tests/Settings", FileFormat::Toml))
        .unwrap();

    c.merge(File::new("tests/Settings-production", FileFormat::Toml))
        .unwrap();

    c
}

#[test]
fn test_merge() {
    let c = make();

    assert_eq!(c.get("debug").ok(), Some(false));
    assert_eq!(c.get("production").ok(), Some(true));
    assert_eq!(c.get("place.creator.name").ok(), Some("Somebody New".to_string()));
    assert_eq!(c.get("place.rating").ok(), Some(4.9));
}
