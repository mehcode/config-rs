#![cfg(feature = "json5")]

extern crate config;
extern crate float_cmp;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use config::*;
use float_cmp::ApproxEqUlps;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Place {
    name: String,
    longitude: f64,
    latitude: f64,
    favorite: bool,
    telephone: Option<String>,
    reviews: u64,
    creator: Map<String, Value>,
    rating: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct Settings {
    debug: f64,
    production: Option<String>,
    place: Place,
    #[serde(rename = "arr")]
    elements: Vec<String>,
}

fn make() -> Config {
    Config::builder()
        .add_source(File::new("tests/Settings", FileFormat::Json5))
        .build()
        .unwrap()
}

#[test]
fn test_file() {
    let c = make();

    // Deserialize the entire file as single struct
    let s: Settings = c.try_into().unwrap();

    assert!(s.debug.approx_eq_ulps(&1.0, 2));
    assert_eq!(s.production, Some("false".to_string()));
    assert_eq!(s.place.name, "Torre di Pisa");
    assert!(s.place.longitude.approx_eq_ulps(&43.7224985, 2));
    assert!(s.place.latitude.approx_eq_ulps(&10.3970522, 2));
    assert_eq!(s.place.favorite, false);
    assert_eq!(s.place.reviews, 3866);
    assert_eq!(s.place.rating, Some(4.5));
    assert_eq!(s.place.telephone, None);
    assert_eq!(s.elements.len(), 10);
    assert_eq!(s.elements[3], "4".to_string());
    if cfg!(feature = "preserve_order") {
        assert_eq!(
            s.place
                .creator
                .into_iter()
                .collect::<Vec<(String, config::Value)>>(),
            vec![
                ("name".to_string(), "John Smith".into()),
                ("username".into(), "jsmith".into()),
                ("email".into(), "jsmith@localhost".into()),
            ]
        );
    } else {
        assert_eq!(
            s.place.creator["name"].clone().into_string().unwrap(),
            "John Smith".to_string()
        );
    }
}

#[test]
fn test_error_parse() {
    let res = Config::builder()
        .add_source(File::new("tests/Settings-invalid", FileFormat::Json5))
        .build();

    let path_with_extension: PathBuf = ["tests", "Settings-invalid.json5"].iter().collect();

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        format!(
            " --> 2:7\n  |\n2 |   ok: true␊\n  |       ^---\n  |\n  = expected null in {}",
            path_with_extension.display()
        )
    );
}

#[test]
fn test_error_root_not_table() {
    match Config::builder()
        .add_source(File::from_str(r#"false"#, FileFormat::Json5))
        .build()
    {
        Ok(_) => panic!("Should not merge if root is not a table"),
        Err(e) => match e {
            ConfigError::FileParse { cause, .. } => assert_eq!(
                "invalid type: boolean `false`, expected a map",
                format!("{}", cause)
            ),
            _ => panic!("Wrong error: {:?}", e),
        },
    }
}
