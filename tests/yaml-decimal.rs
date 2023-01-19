#![cfg(feature = "yaml")]

use rust_decimal::Decimal;
use serde::Deserialize;
use config::{File, FileFormat};

const YAML: &str = r"
foo: 100.0
bar: 200.0
";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub foo: Decimal,
    pub bar: Option<Decimal>,
}

#[test]
fn test_yaml_decimal() {
    let c: Config = config::Config::builder()
        .add_source(File::from_str(YAML, FileFormat::Yaml))
        .build()
        .unwrap()
        .try_deserialize()
        .expect("Deserialization failed");
    assert_eq!(c.foo, Decimal::from(100));
    assert_eq!(c.bar, Some(Decimal::from(200)));
}
