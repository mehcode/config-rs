#![cfg(feature = "toml")]

extern crate config;

#[macro_use]
extern crate serde_derive;

use config::*;

fn make() -> Config {
    let mut c = Config::default();
    c.merge(File::new("tests/Settings", FileFormat::Toml))
        .unwrap();

    c
}

#[test]
fn test_error_parse() {
    let mut c = Config::default();
    let res = c.merge(File::new("tests/Settings-invalid", FileFormat::Toml));

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "invalid number at line 2 in tests/Settings-invalid.toml".to_string()
    );
}

#[test]
fn test_error_type() {
    let c = make();

    let res = c.get::<bool>("boolean_s_parse");

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "invalid type: string \"fals\", expected a boolean for key \
         `boolean_s_parse` in tests/Settings.toml"
            .to_string()
    );
}

#[test]
fn test_error_type_detached() {
    let c = make();

    let value = c.get::<Value>("boolean_s_parse").unwrap();
    let res = value.try_into::<bool>();

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "invalid type: string \"fals\", expected a boolean".to_string()
    );
}

#[test]
fn test_error_enum_de() {
    #[derive(Debug, Deserialize, PartialEq)]
    enum Diode {
        Off,
        Brightness(i32),
        Blinking(i32, i32),
        Pattern { name: String, inifinite: bool },
    }

    let on_v: Value = "on".into();
    let on_d = on_v.try_into::<Diode>();
    assert_eq!(
        on_d.unwrap_err().to_string(),
        "enum Diode does not have variant constructor on".to_string()
    );

    let array_v: Value = vec![100, 100].into();
    let array_d = array_v.try_into::<Diode>();
    assert_eq!(
        array_d.unwrap_err().to_string(),
        "value of enum Diode should be represented by either string or table with exactly one key"
    );


    let confused_v: Value =
    [("Brightness".to_string(), 100.into()),
     ("Blinking".to_string(), vec![300, 700].into())]
    .iter().cloned().collect::<std::collections::HashMap<String, Value>>().into();
    let confused_d = confused_v.try_into::<Diode>();
    assert_eq!(
        confused_d.unwrap_err().to_string(),
        "value of enum Diode should be represented by either string or table with exactly one key"
    );
}

