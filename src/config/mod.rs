mod builder;
mod config;
mod error;

pub use crate::config::builder::*;
pub use crate::config::config::*;
pub use crate::config::error::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::AsConfigElement;
    use crate::element::ConfigElement;

    #[test]
    fn test_compile_loading() {
        let _c = Config::builder()
            .load(&crate::source::test_source::TestSource(|| ConfigElement::Null))
            .unwrap()
            .build();
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_load_json() {
        let json: serde_json::Value = serde_json::from_str(r#"
            { "key": "value" }
        "#).unwrap();
        let json = std::sync::Arc::new(json);

        let _c = Config::builder()
            .load(&crate::source::test_source::TestSource(|| json.as_config_element().unwrap()))
            .unwrap()
            .build();
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_load_json_get_value() {
        let json: serde_json::Value = serde_json::from_str(r#"
            { "key": "value" }
        "#).unwrap();
        let json = std::sync::Arc::new(json);

        let source = crate::source::test_source::TestSource(|| json.as_config_element().unwrap());

        let c = Config::builder()
            .load(&source)
            .unwrap()
            .build();

        let r = c.get("key");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(std::matches!(r, ConfigElement::Str("value")));
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_layered_json_config() {
        let json1: serde_json::Value = serde_json::from_str(r#"
            { "key1": "value1" }
        "#).unwrap();
        let json1 = std::sync::Arc::new(json1);

        let json2: serde_json::Value = serde_json::from_str(r#"
            { "key1": "value2", "key2": "value3" }
        "#).unwrap();
        let json2 = std::sync::Arc::new(json2);

        let source1 = crate::source::test_source::TestSource(|| json1.as_config_element().unwrap());
        let source2 = crate::source::test_source::TestSource(|| json2.as_config_element().unwrap());

        let c = Config::builder()
            .load(&source1)
            .unwrap()
            .load(&source2)
            .unwrap()
            .build();

        let r = c.get("key1");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(std::matches!(r, ConfigElement::Str("value1")));

        let r = c.get("key2");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(std::matches!(r, ConfigElement::Str("value3")));
    }

    #[test]
    #[cfg(all(feature = "json", feature = "toml"))]
    fn test_layered_json_toml_config() {
        let json: serde_json::Value = serde_json::from_str(r#"
            { "key1": "value1" }
        "#).unwrap();
        let json = std::sync::Arc::new(json);

        let toml: toml::Value = toml::from_str(r#"
            key1 = "value2"
            key2 = "value3"
        "#).unwrap();
        let toml = std::sync::Arc::new(toml);

        let source1 = crate::source::test_source::TestSource(|| json.as_config_element().unwrap());
        let source2 = crate::source::test_source::TestSource(|| toml.as_config_element().unwrap());

        let c = Config::builder()
            .load(&source1)
            .unwrap()
            .load(&source2)
            .unwrap()
            .build();

        let r = c.get("key1");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(std::matches!(r, ConfigElement::Str("value1")));

        let r = c.get("key2");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(std::matches!(r, ConfigElement::Str("value3")));
    }
}
