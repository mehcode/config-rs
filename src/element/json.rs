use std::collections::HashMap;

use crate::element::IntoConfigElement;
use crate::element::ConfigElement;

#[derive(Debug, thiserror::Error)]
pub enum JsonIntoConfigElementError {}

impl IntoConfigElement for serde_json::Value {
    type Error = JsonIntoConfigElementError;

    fn into_config_element(self) -> Result<ConfigElement, Self::Error> {
        match self {
            serde_json::Value::Null => Ok(ConfigElement::Null),
            serde_json::Value::Bool(b) => Ok(ConfigElement::Bool(b)),
            serde_json::Value::Number(num) => num
                .as_i64()
                .map(ConfigElement::I64)
                .or_else(|| num.as_u64().map(ConfigElement::U64))
                .or_else(|| num.as_f64().map(ConfigElement::F64))
                .ok_or_else(|| unimplemented!()),
            serde_json::Value::String(s) => Ok(ConfigElement::Str(s)),
            serde_json::Value::Array(vec) => vec
                .into_iter()
                .map(|v| v.into_config_element())
                .collect::<Result<Vec<_>, Self::Error>>()
                .map(ConfigElement::List),
            serde_json::Value::Object(obj) => obj
                .into_iter()
                .map(|(k, v)| v.into_config_element().map(|v| (k.to_string(), v)))
                .collect::<Result<HashMap<String, ConfigElement>, JsonIntoConfigElementError>>()
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
                assert_eq!(*map.get("key").unwrap(), ConfigElement::Str("value".to_string()));
            }
            _ => panic!("Not a map"),
        }
    }
}
