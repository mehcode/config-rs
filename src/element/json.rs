use std::collections::HashMap;

use crate::element::AsConfigElement;
use crate::element::ConfigElement;

#[derive(Debug, thiserror::Error)]
pub enum JsonIntoConfigElementError {}

impl<'source> AsConfigElement<'source> for serde_json::Value {
    type Error = JsonIntoConfigElementError;

    fn as_config_element(&'source self) -> Result<ConfigElement<'source>, Self::Error> {
        match self {
            serde_json::Value::Null => Ok(ConfigElement::Null),
            serde_json::Value::Bool(b) => Ok(ConfigElement::Bool(*b)),
            serde_json::Value::Number(num) => num
                .as_i64()
                .map(ConfigElement::I64)
                .or_else(|| num.as_u64().map(ConfigElement::U64))
                .or_else(|| num.as_f64().map(ConfigElement::F64))
                .ok_or_else(|| unimplemented!()),
            serde_json::Value::String(s) => Ok(ConfigElement::Str(&s)),
            serde_json::Value::Array(vec) => vec
                .iter()
                .map(serde_json::Value::as_config_element)
                .collect::<Result<Vec<_>, Self::Error>>()
                .map(ConfigElement::List),
            serde_json::Value::Object(obj) => obj
                .iter()
                .map(|(k, v)| v.as_config_element().map(|v| (k.as_ref(), v)))
                .collect::<Result<HashMap<&str, ConfigElement<'_>>, JsonIntoConfigElementError>>()
                .map(|map| ConfigElement::Map(map)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::element::ConfigElement;

    #[test]
    fn deser_json_1() {
        let s = r#"
            { "key": "value" }
        "#;

        let e: ConfigElement = serde_json::from_str(s).unwrap();
        match e {
            ConfigElement::Map(map) => {
                assert_eq!(*map.get("key").unwrap(), ConfigElement::Str("value"));
            }
            _ => panic!("Not a map"),
        }
    }
}
