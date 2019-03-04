#![cfg(feature = "hocon")]

extern crate config;
extern crate float_cmp;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use config::*;
use float_cmp::ApproxEqUlps;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Place {
    name: String,
    longitude: f64,
    latitude: f64,
    favorite: bool,
    telephone: Option<String>,
    reviews: u64,
    creator: HashMap<String, Value>,
    rating: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct Settings {
    debug: f64,
    production: Option<String>,
    place: Place,
    #[serde(rename = "arr")]
    elements: Vec<String>,
    #[serde(rename = "json-included")]
    json: Option<Box<Settings>>,
}

fn make() -> Config {
    let mut c = Config::default();
    c.merge(File::new("tests/Settings", FileFormat::Hocon))
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
    assert_eq!(s.place.name, "Torre di Pisa");
    assert!(s.place.longitude.approx_eq_ulps(&43.7224985, 2));
    assert!(s.place.latitude.approx_eq_ulps(&10.3970522, 2));
    assert_eq!(s.place.favorite, false);
    assert_eq!(s.place.reviews, 3866);
    assert_eq!(s.place.rating, Some(4.6));
    assert_eq!(s.place.telephone, None);
    assert_eq!(s.elements.len(), 10);
    assert_eq!(s.elements[3], "4".to_string());
    assert_eq!(
        s.place.creator["name"].clone().into_str().unwrap(),
        "John Smith".to_string()
    );
    assert_eq!(s.json.unwrap().place.rating, Some(4.5));
}

#[test]
fn test_error_parse() {
    let mut c = Config::default();
    let res = c.merge(File::new("tests/Settings-invalid", FileFormat::Hocon));

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "Error wile parsing document in tests/Settings-invalid.conf".to_string()
    );
}
