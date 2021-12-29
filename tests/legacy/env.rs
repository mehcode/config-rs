
use config::{Config};
use serde_derive::Deserialize;
use std::env;

/// Reminder that tests using env variables need to use different env variable names, since
/// tests can be run in parallel


#[test]
fn test_parse_int() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestIntEnum {
        Int(TestInt),
    }

    #[derive(Deserialize, Debug)]
    struct TestInt {
        int_val: i32,
    }

    env::set_var("INT_VAL", "42");

    let environment = Environment::new().try_parsing(true);
    let mut config = Config::default();

    config.set("tag", "Int").unwrap();

    config.merge(environment).unwrap();

    let config: TestIntEnum = config.try_deserialize().unwrap();

    assert!(matches!(config, TestIntEnum::Int(TestInt { int_val: 42 })));

    env::remove_var("INT_VAL");
}

#[test]
fn test_parse_float() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestFloatEnum {
        Float(TestFloat),
    }

    #[derive(Deserialize, Debug)]
    struct TestFloat {
        float_val: f64,
    }

    env::set_var("FLOAT_VAL", "42.3");

    let environment = Environment::new().try_parsing(true);
    let mut config = Config::default();

    config.set("tag", "Float").unwrap();

    config.merge(environment).unwrap();

    let config: TestFloatEnum = config.try_deserialize().unwrap();

    // can't use `matches!` because of float value
    match config {
        TestFloatEnum::Float(TestFloat { float_val }) => assert_eq!(float_val, 42.3),
    }

    env::remove_var("FLOAT_VAL");
}

#[test]
fn test_parse_bool() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestBoolEnum {
        Bool(TestBool),
    }

    #[derive(Deserialize, Debug)]
    struct TestBool {
        bool_val: bool,
    }

    env::set_var("BOOL_VAL", "true");

    let environment = Environment::new().try_parsing(true);
    let mut config = Config::default();

    config.set("tag", "Bool").unwrap();

    config.merge(environment).unwrap();

    let config: TestBoolEnum = config.try_deserialize().unwrap();

    assert!(matches!(
        config,
        TestBoolEnum::Bool(TestBool { bool_val: true }),
    ));

    env::remove_var("BOOL_VAL");
}

#[test]
#[should_panic(expected = "invalid type: string \"42\", expected i32")]
fn test_parse_off_int() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestIntEnum {
        Int(TestInt),
    }

    #[derive(Deserialize, Debug)]
    struct TestInt {
        int_val_1: i32,
    }

    env::set_var("INT_VAL_1", "42");

    let environment = Environment::new().try_parsing(false);
    let mut config = Config::default();

    config.set("tag", "Int").unwrap();

    config.merge(environment).unwrap();

    env::remove_var("INT_VAL_1");

    config.try_deserialize::<TestIntEnum>().unwrap();
}

#[test]
#[should_panic(expected = "invalid type: string \"42.3\", expected f64")]
fn test_parse_off_float() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestFloatEnum {
        Float(TestFloat),
    }

    #[derive(Deserialize, Debug)]
    struct TestFloat {
        float_val_1: f64,
    }

    env::set_var("FLOAT_VAL_1", "42.3");

    let environment = Environment::new().try_parsing(false);
    let mut config = Config::default();

    config.set("tag", "Float").unwrap();

    config.merge(environment).unwrap();

    env::remove_var("FLOAT_VAL_1");

    config.try_deserialize::<TestFloatEnum>().unwrap();
}

#[test]
#[should_panic(expected = "invalid type: string \"true\", expected a boolean")]
fn test_parse_off_bool() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestBoolEnum {
        Bool(TestBool),
    }

    #[derive(Deserialize, Debug)]
    struct TestBool {
        bool_val_1: bool,
    }

    env::set_var("BOOL_VAL_1", "true");

    let environment = Environment::new().try_parsing(false);
    let mut config = Config::default();

    config.set("tag", "Bool").unwrap();

    config.merge(environment).unwrap();

    env::remove_var("BOOL_VAL_1");

    config.try_deserialize::<TestBoolEnum>().unwrap();
}

#[test]
#[should_panic(expected = "invalid type: string \"not an int\", expected i32")]
fn test_parse_int_fail() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestIntEnum {
        Int(TestInt),
    }

    #[derive(Deserialize, Debug)]
    struct TestInt {
        int_val_2: i32,
    }

    env::set_var("INT_VAL_2", "not an int");

    let environment = Environment::new().try_parsing(true);
    let mut config = Config::default();

    config.set("tag", "Int").unwrap();

    config.merge(environment).unwrap();

    env::remove_var("INT_VAL_2");

    config.try_deserialize::<TestIntEnum>().unwrap();
}

#[test]
#[should_panic(expected = "invalid type: string \"not a float\", expected f64")]
fn test_parse_float_fail() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestFloatEnum {
        Float(TestFloat),
    }

    #[derive(Deserialize, Debug)]
    struct TestFloat {
        float_val_2: f64,
    }

    env::set_var("FLOAT_VAL_2", "not a float");

    let environment = Environment::new().try_parsing(true);
    let mut config = Config::default();

    config.set("tag", "Float").unwrap();

    config.merge(environment).unwrap();

    env::remove_var("FLOAT_VAL_2");

    config.try_deserialize::<TestFloatEnum>().unwrap();
}

#[test]
#[should_panic(expected = "invalid type: string \"not a bool\", expected a boolean")]
fn test_parse_bool_fail() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestBoolEnum {
        Bool(TestBool),
    }

    #[derive(Deserialize, Debug)]
    struct TestBool {
        bool_val_2: bool,
    }

    env::set_var("BOOL_VAL_2", "not a bool");

    let environment = Environment::new().try_parsing(true);
    let mut config = Config::default();

    config.set("tag", "Bool").unwrap();

    config.merge(environment).unwrap();

    env::remove_var("BOOL_VAL_2");

    config.try_deserialize::<TestBoolEnum>().unwrap();
}

#[test]
fn test_parse_string() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestStringEnum {
        String(TestString),
    }

    #[derive(Deserialize, Debug)]
    struct TestString {
        string_val: String,
    }

    env::set_var("STRING_VAL", "test string");

    let environment = Environment::new().try_parsing(true);
    let mut config = Config::default();

    config.set("tag", "String").unwrap();

    config.merge(environment).unwrap();

    let config: TestStringEnum = config.try_deserialize().unwrap();

    let test_string = String::from("test string");

    match config {
        TestStringEnum::String(TestString { string_val }) => assert_eq!(test_string, string_val),
    }

    env::remove_var("STRING_VAL");
}

#[test]
fn test_parse_off_string() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestStringEnum {
        String(TestString),
    }

    #[derive(Deserialize, Debug)]
    struct TestString {
        string_val_1: String,
    }

    env::set_var("STRING_VAL_1", "test string");

    let environment = Environment::new().try_parsing(false);
    let mut config = Config::default();

    config.set("tag", "String").unwrap();

    config.merge(environment).unwrap();

    let config: TestStringEnum = config.try_deserialize().unwrap();

    let test_string = String::from("test string");

    match config {
        TestStringEnum::String(TestString { string_val_1 }) => {
            assert_eq!(test_string, string_val_1)
        }
    }

    env::remove_var("STRING_VAL_1");
}
