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
    let res = c.merge(File::new("tests/Settings-invalid", FileFormat::Toml));

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(),
               "invalid number at line 2 in tests/Settings-invalid.toml".to_string());
}

#[test]
fn test_error_type() {
    let c = make();

    let res = c.get::<bool>("boolean_s_parse");

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(),
               "invalid type: string \"fals\", expected a boolean for key `boolean_s_parse` in tests/Settings.toml"
                   .to_string());
}

#[test]
fn test_error_type_detached() {
    let c = make();

    let value = c.get::<Value>("boolean_s_parse").unwrap();
    let res = value.try_into::<bool>();

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(),
               "invalid type: string \"fals\", expected a boolean".to_string());
}
