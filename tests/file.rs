extern crate config;

use config::*;

#[test]
fn test_file_not_required() {
    let mut c = Config::default();
    let res = c.merge(File::new("tests/NoSettings", FileFormat::Yaml).required(false));

    assert!(res.is_ok());
}

#[test]
fn test_file_required_not_found() {
    let mut c = Config::default();
    let res = c.merge(File::new("tests/NoSettings", FileFormat::Yaml));

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(),
               "configuration file \"tests/NoSettings\" not found"
                   .to_string());
}
