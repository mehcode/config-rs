extern crate config;
extern crate chrono;

use config::*;
use chrono::{DateTime, Utc, TimeZone};

fn make() -> Config {
    Config::default()
        .merge(File::from_str(
            r#"
            {
                "json_datetime": "2017-05-10T02:14:53Z"
            }
            "#,
            FileFormat::Json,
        ))
        .merge(File::from_str(
            r#"
            yaml_datetime: 2017-06-12T10:58:30Z
            "#,
            FileFormat::Yaml,
        ))
        .merge(File::from_str(
            r#"
            toml_datetime = 2017-05-11T14:55:15Z
            "#,
            FileFormat::Toml,
        ))
        .unwrap()
}

#[test]
fn test_datetime_string() {
    let s = make();

    // JSON
    let date: String = s.get("json_datetime").unwrap();

    assert_eq!(&date, "2017-05-10T02:14:53Z");

    // TOML
    let date: String = s.get("toml_datetime").unwrap();

    assert_eq!(&date, "2017-05-11T14:55:15Z");

    // YAML
    let date: String = s.get("yaml_datetime").unwrap();

    assert_eq!(&date, "2017-06-12T10:58:30Z");
}

#[test]
fn test_datetime() {
    let s = make();

    // JSON
    let date: DateTime<Utc> = s.get("json_datetime").unwrap();

    assert_eq!(date, Utc.ymd(2017, 5, 10).and_hms(2, 14, 53));

    // TOML
    let date: DateTime<Utc> = s.get("toml_datetime").unwrap();

    assert_eq!(date, Utc.ymd(2017, 5, 11).and_hms(14, 55, 15));

    // YAML
    let date: DateTime<Utc> = s.get("yaml_datetime").unwrap();

    assert_eq!(date, Utc.ymd(2017, 6, 12).and_hms(10, 58, 30));
}
