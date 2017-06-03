extern crate config;

use config::*;

fn make() -> Config {
    let mut c = Config::default();
    c.merge(File::new("tests/Settings", FileFormat::Toml))
        .unwrap();

    c
}

#[test]
fn test_error_parse() {
    let mut c = Config::default();
    c.merge(File::new("tests/Settings.invalid", FileFormat::Toml))
        .unwrap();

    assert!(false)
}

#[test]
fn test_error_type_bool() {
    let c = make();

    let err = c.get::<bool>("boolean_s_parse");

    assert!(err.is_err());
    assert_eq!(err.unwrap_err().to_string(),
        "invalid type: string \"fals\", expected a boolean from tests/Settings.toml".to_string());
}
