use std::collections::HashMap;

use crate::element::AsConfigElement;
use crate::element::ConfigElement;

#[derive(Debug, thiserror::Error)]
pub enum TomlIntoConfigElementError {}

impl<'source> AsConfigElement<'source> for toml::Value {
    type Error = TomlIntoConfigElementError;

    fn as_config_element(&'source self) -> Result<ConfigElement<'source>, Self::Error> {
        match self {
            toml::Value::String(s) => Ok(ConfigElement::Str(&s)),
            toml::Value::Integer(i) => Ok(ConfigElement::I64(*i)),
            toml::Value::Float(f) => Ok(ConfigElement::F64(*f)),
            toml::Value::Boolean(b) => Ok(ConfigElement::Bool(*b)),
            toml::Value::Datetime(_) => unimplemented!(), // TODO
            toml::Value::Array(ary) => {
                ary.into_iter()
                    .map(toml::Value::as_config_element)
                    .collect::<Result<Vec<_>, Self::Error>>()
                    .map(ConfigElement::List)
            },
            toml::Value::Table(table) => {
                table.into_iter()
                    .map(|(k, v)| v.as_config_element().map(|v| (k.as_ref(), v)))
                    .collect::<Result<HashMap<&str, ConfigElement<'_>>, Self::Error>>()
                    .map(ConfigElement::Map)
            }

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
                assert_eq!(*map.get("key").unwrap(), ConfigElement::Str("value"));
            }
            _ => panic!("Not a map"),
        }
    }
}

