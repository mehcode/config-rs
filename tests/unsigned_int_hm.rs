#![cfg(not(feature = "preserve_order"))]

#[derive(serde::Deserialize, Eq, PartialEq, Debug)]
struct Container<T> {
    inner: T,
}

#[derive(serde::Deserialize, Eq, PartialEq, Debug)]
struct Unsigned {
    unsigned: u16,
}

impl Default for Unsigned {
    fn default() -> Self {
        Self { unsigned: 128 }
    }
}

impl From<Unsigned> for config::ValueKind {
    fn from(unsigned: Unsigned) -> Self {
        let mut properties = std::collections::HashMap::new();
        properties.insert(
            "unsigned".to_string(),
            config::Value::from(unsigned.unsigned),
        );

        Self::Table(properties)
    }
}

#[test]
fn test_deser_unsigned_int_hm() {
    let container = Container {
        inner: Unsigned::default(),
    };

    let built = config::Config::builder()
        .set_default("inner", Unsigned::default())
        .unwrap()
        .build()
        .unwrap()
        .try_deserialize::<Container<Unsigned>>()
        .unwrap();

    assert_eq!(container, built);
}
