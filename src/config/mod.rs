mod builder;
mod config;
mod error;

pub use crate::config::builder::*;
pub use crate::config::config::*;
pub use crate::config::error::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::IntoConfigElement;
    use crate::element::ConfigElement;

    #[test]
    fn test_compile_loading() {
        let _c = Config::builder()
            .load(Box::new(crate::source::test_source::TestSource(ConfigElement::Null)))
            .build()
            .unwrap();
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_load_json() {
        let json: serde_json::Value = serde_json::from_str(r#"
            { "key": "value" }
        "#).unwrap();

        let _c = Config::builder()
            .load(Box::new(crate::source::test_source::TestSource(json.into_config_element().unwrap())))
            .build()
            .unwrap();
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_load_json_get_value() {
        let json: serde_json::Value = serde_json::from_str(r#"
            { "key": "value" }
        "#).unwrap();

        let source = crate::source::test_source::TestSource(json.into_config_element().unwrap());

        let c = Config::builder()
            .load(Box::new(source))
            .build()
            .unwrap();

        let r = c.get("key");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        match r {
            ConfigElement::Str(s) => assert_eq!(s, "value"),
            _ => panic!(),
        }
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_layered_json_config() {
        let json1: serde_json::Value = serde_json::from_str(r#"
            { "key1": "value1" }
        "#).unwrap();

        let json2: serde_json::Value = serde_json::from_str(r#"
            { "key1": "value2", "key2": "value3" }
        "#).unwrap();

        let source1 = crate::source::test_source::TestSource(json1.into_config_element().unwrap());
        let source2 = crate::source::test_source::TestSource(json2.into_config_element().unwrap());

        let c = Config::builder()
            .load(Box::new(source1))
            .load(Box::new(source2))
            .build()
            .unwrap();

        let r = c.get("key1");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        match r {
            ConfigElement::Str(s) => assert_eq!(s, "value1"),
            _ => panic!(),
        }

        let r = c.get("key2");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        match r {
            ConfigElement::Str(s) => assert_eq!(s, "value3"),
            _ => panic!(),
        }
    }

    #[test]
    #[cfg(all(feature = "json", feature = "toml"))]
    fn test_layered_json_toml_config() {
        let json: serde_json::Value = serde_json::from_str(r#"
            { "key1": "value1" }
        "#).unwrap();

        let toml: toml::Value = toml::from_str(r#"
            key1 = "value2"
            key2 = "value3"
        "#).unwrap();

        let source1 = crate::source::test_source::TestSource(json.into_config_element().unwrap());
        let source2 = crate::source::test_source::TestSource(toml.into_config_element().unwrap());

        let c = Config::builder()
            .load(Box::new(source1))
            .load(Box::new(source2))
            .build()
            .unwrap();

        let r = c.get("key1");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        match r {
            ConfigElement::Str(s) => assert_eq!(s, "value1"),
            _ => panic!(),
        }

        let r = c.get("key2");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        match r {
            ConfigElement::Str(s) => assert_eq!(s, "value3"),
            _ => panic!(),
        }
    }
}
