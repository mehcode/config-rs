use std::collections::HashMap;

use crate::element::ConfigElement;
use crate::element::IntoConfigElement;

#[derive(Debug, thiserror::Error)]
pub enum TomlIntoConfigElementError {}

impl IntoConfigElement for toml::Value {
    type Error = TomlIntoConfigElementError;

    fn into_config_element(self) -> Result<ConfigElement, Self::Error> {
        match self {
            toml::Value::String(s) => Ok(ConfigElement::Str(s)),
            toml::Value::Integer(i) => Ok(ConfigElement::I64(i)),
            toml::Value::Float(f) => Ok(ConfigElement::F64(f)),
            toml::Value::Boolean(b) => Ok(ConfigElement::Bool(b)),
            toml::Value::Datetime(_) => unimplemented!(), // TODO
            toml::Value::Array(ary) => ary
                .into_iter()
                .map(|e| e.into_config_element())
                .collect::<Result<Vec<_>, Self::Error>>()
                .map(ConfigElement::List),
            toml::Value::Table(table) => table
                .into_iter()
                .map(|(k, v)| v.into_config_element().map(|v| (k.to_string(), v)))
                .collect::<Result<HashMap<String, ConfigElement>, Self::Error>>()
                .map(ConfigElement::Map),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::element::ConfigElement;

    #[test]
    fn deser_toml_1() {
        let s = r#"
            key = "value"
        "#;

        let e: ConfigElement = toml::from_str(s).unwrap();
        match e {
            ConfigElement::Map(map) => {
                assert_eq!(
                    *map.get("key").unwrap(),
                    ConfigElement::Str("value".to_string())
                );
            }
            _ => panic!("Not a map"),
        }
    }
}
