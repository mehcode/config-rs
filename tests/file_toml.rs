#![cfg(feature = "toml")]

extern crate config;
extern crate float_cmp;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use config::*;
use float_cmp::ApproxEqUlps;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Place {
    number: PlaceNumber,
    name: String,
    longitude: f64,
    latitude: f64,
    favorite: bool,
    telephone: Option<String>,
    reviews: u64,
    creator: HashMap<String, Value>,
    rating: Option<f32>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct PlaceNumber(u8);

#[derive(Debug, Deserialize, PartialEq)]
struct AsciiCode(i8);

#[derive(Debug, Deserialize)]
struct Settings {
    debug: f64,
    production: Option<String>,
    code: AsciiCode,
    place: Place,
    #[serde(rename = "arr")]
    elements: Vec<String>,
}

fn make() -> Config {
    let mut c = Config::default();
    c.merge(File::new("tests/Settings", FileFormat::Toml))
        .unwrap();

    c
}

#[test]
fn test_file() {
    let c = make();

    // Deserialize the entire file as single struct
    let s: Settings = c.try_into().unwrap();

    assert!(s.debug.approx_eq_ulps(&1.0, 2));
    assert_eq!(s.production, Some("false".to_string()));
    assert_eq!(s.code, AsciiCode(53));
    assert_eq!(s.place.number, PlaceNumber(1));
    assert_eq!(s.place.name, "Torre di Pisa");
    assert!(s.place.longitude.approx_eq_ulps(&43.7224985, 2));
    assert!(s.place.latitude.approx_eq_ulps(&10.3970522, 2));
    assert_eq!(s.place.favorite, false);
    assert_eq!(s.place.reviews, 3866);
    assert_eq!(s.place.rating, Some(4.5));
    assert_eq!(s.place.telephone, None);
    assert_eq!(s.elements.len(), 10);
    assert_eq!(s.elements[3], "4".to_string());
    assert_eq!(
        s.place.creator["name"].clone().into_str().unwrap(),
        "John Smith".to_string()
    );
}

#[test]
fn test_error_parse() {
    let mut c = Config::default();
    let res = c.merge(File::new("tests/Settings-invalid", FileFormat::Toml));

    let path_with_extension: PathBuf = ["tests", "Settings-invalid.toml"].iter().collect();

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        format!(
            "invalid TOML value, did you mean to use a quoted string? at line 2 column 9 in {}",
            path_with_extension.display()
        )
    );
}
