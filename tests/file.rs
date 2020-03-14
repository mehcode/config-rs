#![cfg(feature = "yaml")]

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
    assert_eq!(
        res.unwrap_err().to_string(),
        "configuration file \"tests/NoSettings\" not found".to_string()
    );
}

#[test]
fn test_file_exact_not_exist() {
    let mut c = Config::default();
    let res = c.merge(File::with_exact_name("tests/Settings.wrong"));

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "configuration file \"tests/Settings.wrong\" not found".to_string()
    );
}

#[test]
fn test_file_exact_exist_invalid_extension() {
    let mut c = Config::default();
    let res = c.merge(File::with_exact_name("tests/Settings.wrongextension"));

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "configuration file \"tests/Settings.wrongextension\" is not of a registered file format".to_string()
    );
}

#[test]
fn test_file_exact_explicit_format() {
    let mut c = Config::default();
    c.merge(File::new("tests/Settings.wrongextension", FileFormat::Toml).exact_name(true))
        .unwrap();

    assert_eq!(c.get("debug").ok(), Some(false));
    assert_eq!(c.get("production").ok(), Some(true));
}

#[test]
fn test_file_auto() {
    let mut c = Config::default();
    c.merge(File::with_name("tests/Settings-production"))
        .unwrap();

    assert_eq!(c.get("debug").ok(), Some(false));
    assert_eq!(c.get("production").ok(), Some(true));
}

#[test]
fn test_file_auto_not_found() {
    let mut c = Config::default();
    let res = c.merge(File::with_name("tests/NoSettings"));

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "configuration file \"tests/NoSettings\" not found".to_string()
    );
}

#[test]
fn test_file_ext() {
    let mut c = Config::default();
    c.merge(File::with_name("tests/Settings.json")).unwrap();

    assert_eq!(c.get("debug").ok(), Some(true));
    assert_eq!(c.get("production").ok(), Some(false));
}
