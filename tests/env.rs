extern crate config;
extern crate serde_derive;

use config::*;
use serde_derive::Deserialize;
use std::env;

/// Reminder that tests using env variables need to use different env variable names, since
/// tests can be run in parallel

#[test]
fn test_default() {
    env::set_var("A_B_C", "abc");

    let environment = Environment::new();

    assert!(environment.collect().unwrap().contains_key("a_b_c"));

    env::remove_var("A_B_C");
}

#[test]
fn test_prefix_is_removed_from_key() {
    env::set_var("B_A_C", "abc");

    let environment = Environment::with_prefix("B");

    assert!(environment.collect().unwrap().contains_key("a_c"));

    env::remove_var("B_A_C");
}

#[test]
fn test_prefix_with_variant_forms_of_spelling() {
    env::set_var("a_A_C", "abc");

    let environment = Environment::with_prefix("a");

    assert!(environment.collect().unwrap().contains_key("a_c"));

    env::remove_var("a_A_C");
    env::set_var("aB_A_C", "abc");

    let environment = Environment::with_prefix("aB");

    assert!(environment.collect().unwrap().contains_key("a_c"));

    env::remove_var("aB_A_C");
    env::set_var("Ab_A_C", "abc");

    let environment = Environment::with_prefix("ab");

    assert!(environment.collect().unwrap().contains_key("a_c"));

    env::remove_var("Ab_A_C");
}

#[test]
fn test_separator_behavior() {
    env::set_var("C_B_A", "abc");

    let environment = Environment::with_prefix("C").separator("_");

    assert!(environment.collect().unwrap().contains_key("b.a"));

    env::remove_var("C_B_A");
}

#[test]
fn test_empty_value_is_ignored() {
    env::set_var("C_A_B", "");

    let environment = Environment::new().ignore_empty(true);

    assert!(!environment.collect().unwrap().contains_key("c_a_b"));

    env::remove_var("C_A_B");
}

#[test]
fn test_custom_separator_behavior() {
    env::set_var("C.B.A", "abc");

    let environment = Environment::with_prefix("C").separator(".");

    assert!(environment.collect().unwrap().contains_key("b.a"));

    env::remove_var("C.B.A");
}

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

    let config = Config::builder()
        .set_default("tag", "Int")
        .unwrap()
        .add_source(environment)
        .build()
        .unwrap();

    let config: TestIntEnum = config.try_into().unwrap();

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

    let config = Config::builder()
        .set_default("tag", "Float")
        .unwrap()
        .add_source(environment)
        .build()
        .unwrap();

    let config: TestFloatEnum = config.try_into().unwrap();

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

    let config = Config::builder()
        .set_default("tag", "Bool")
        .unwrap()
        .add_source(environment)
        .build()
        .unwrap();

    let config: TestBoolEnum = config.try_into().unwrap();

    assert!(matches!(
        config,
        TestBoolEnum::Bool(TestBool { bool_val: true })
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

    let config = Config::builder()
        .set_default("tag", "Int")
        .unwrap()
        .add_source(environment)
        .build()
        .unwrap();

    env::remove_var("INT_VAL_1");

    config.try_into::<TestIntEnum>().unwrap();
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

    let config = Config::builder()
        .set_default("tag", "Float")
        .unwrap()
        .add_source(environment)
        .build()
        .unwrap();

    env::remove_var("FLOAT_VAL_1");

    config.try_into::<TestFloatEnum>().unwrap();
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

    let config = Config::builder()
        .set_default("tag", "Bool")
        .unwrap()
        .add_source(environment)
        .build()
        .unwrap();

    env::remove_var("BOOL_VAL_1");

    config.try_into::<TestBoolEnum>().unwrap();
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

    let config = Config::builder()
        .set_default("tag", "Int")
        .unwrap()
        .add_source(environment)
        .build()
        .unwrap();

    env::remove_var("INT_VAL_2");

    config.try_into::<TestIntEnum>().unwrap();
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

    let config = Config::builder()
        .set_default("tag", "Float")
        .unwrap()
        .add_source(environment)
        .build()
        .unwrap();

    env::remove_var("FLOAT_VAL_2");

    config.try_into::<TestFloatEnum>().unwrap();
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

    let config = Config::builder()
        .set_default("tag", "Bool")
        .unwrap()
        .add_source(environment)
        .build()
        .unwrap();

    env::remove_var("BOOL_VAL_2");

    config.try_into::<TestBoolEnum>().unwrap();
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

    let config = Config::builder()
        .set_default("tag", "String")
        .unwrap()
        .add_source(environment)
        .build()
        .unwrap();

    let config: TestStringEnum = config.try_into().unwrap();

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

    let config = Config::builder()
        .set_default("tag", "String")
        .unwrap()
        .add_source(environment)
        .build()
        .unwrap();

    let config: TestStringEnum = config.try_into().unwrap();

    let test_string = String::from("test string");

    match config {
        TestStringEnum::String(TestString { string_val_1 }) => {
            assert_eq!(test_string, string_val_1)
        }
    }

    env::remove_var("STRING_VAL_1");
}
