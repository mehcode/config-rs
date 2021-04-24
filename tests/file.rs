#![cfg(feature = "yaml")]

extern crate config;

use config::*;

#[test]
fn test_file_not_required() {
    let mut c = Config::builder();
    c.add_source(File::new("tests/NoSettings", FileFormat::Yaml).required(false));
    let res = c.build();

    assert!(res.is_ok());
}

#[test]
fn test_file_required_not_found() {
    let mut c = Config::builder();
    c.add_source(File::new("tests/NoSettings", FileFormat::Yaml));
    let res = c.build();

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "configuration file \"tests/NoSettings\" not found".to_string()
    );
}

#[test]
fn test_file_auto() {
    let mut builder = Config::builder();
    builder.add_source(File::with_name("tests/Settings-production"));

    let c = builder.build().unwrap();

    assert_eq!(c.get("debug").ok(), Some(false));
    assert_eq!(c.get("production").ok(), Some(true));
}

#[test]
fn test_file_auto_not_found() {
    let mut c = Config::builder();
    c.add_source(File::with_name("tests/NoSettings"));
    let res = c.build();

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "configuration file \"tests/NoSettings\" not found".to_string()
    );
}

#[test]
fn test_file_ext() {
    let mut builder = Config::builder();
    builder.add_source(File::with_name("tests/Settings.json"));

    let c = builder.build().unwrap();

    assert_eq!(c.get("debug").ok(), Some(true));
    assert_eq!(c.get("production").ok(), Some(false));
}
