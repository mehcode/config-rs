extern crate config;
extern crate serde;
extern crate float_cmp;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use float_cmp::ApproxEqUlps;
use config::*;

#[derive(Debug, Deserialize)]
struct Place {
    name: String,
    longitude: f64,
    latitude: f64,
    favorite: bool,
    telephone: Option<String>,
    reviews: u64,
    rating: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct Settings {
    debug: f64,
    production: Option<String>,
    place: Place,
}

fn make() -> Config {
    let mut c = Config::default();
    c.merge(File::new("tests/Settings", FileFormat::Toml))
        .unwrap();

    c
}

#[test]
fn test_scalar() {
    let c = make();

    assert_eq!(c.get("debug").ok(), Some(true));
    assert_eq!(c.get("production").ok(), Some(false));
}

#[test]
fn test_scalar_type_loose() {
    let c = make();

    assert_eq!(c.get("debug").ok(), Some(true));
    assert_eq!(c.get("debug").ok(), Some("true".to_string()));
    assert_eq!(c.get("debug").ok(), Some(1));
    assert_eq!(c.get("debug").ok(), Some(1.0));

    assert_eq!(c.get("debug_s").ok(), Some(true));
    assert_eq!(c.get("debug_s").ok(), Some("true".to_string()));
    assert_eq!(c.get("debug_s").ok(), Some(1));
    assert_eq!(c.get("debug_s").ok(), Some(1.0));

    assert_eq!(c.get("production").ok(), Some(false));
    assert_eq!(c.get("production").ok(), Some("false".to_string()));
    assert_eq!(c.get("production").ok(), Some(0));
    assert_eq!(c.get("production").ok(), Some(0.0));

    assert_eq!(c.get("production_s").ok(), Some(false));
    assert_eq!(c.get("production_s").ok(), Some("false".to_string()));
    assert_eq!(c.get("production_s").ok(), Some(0));
    assert_eq!(c.get("production_s").ok(), Some(0.0));
}

#[test]
fn test_get_scalar_path() {
    let c = make();

    assert_eq!(c.get("place.favorite").ok(), Some(false));
    assert_eq!(c.get("place.creator.name").ok(), Some("John Smith".to_string()));
}

#[test]
fn test_map() {
    let c = make();
    let m: HashMap<String, Value> = c.get("place").unwrap();

    assert_eq!(m.len(), 7);
    assert_eq!(m["name"].clone().into_str().unwrap(), "Torre di Pisa".to_string());
    assert_eq!(m["reviews"].clone().into_int().unwrap(), 3866);
}

#[test]
fn test_map_str() {
    let c = make();
    let m: HashMap<String, String> = c.get("place.creator").unwrap();

    assert_eq!(m.len(), 1);
    assert_eq!(m["name"], "John Smith".to_string());
}

#[test]
fn test_map_struct() {
    #[derive(Debug, Deserialize)]
    struct Settings {
        place: HashMap<String, Value>,
    }

    let c = make();
    let s: Settings = c.deserialize().unwrap();

    assert_eq!(s.place.len(), 7);
    assert_eq!(s.place["name"].clone().into_str().unwrap(), "Torre di Pisa".to_string());
    assert_eq!(s.place["reviews"].clone().into_int().unwrap(), 3866);
}

#[test]
fn test_file_struct() {
    let c = make();

    // Deserialize the entire file as single struct
    let s: Settings = c.deserialize().unwrap();

    assert!(s.debug.approx_eq_ulps(&1.0, 2));
    assert_eq!(s.production, Some("false".to_string()));
    assert_eq!(s.place.name, "Torre di Pisa");
    assert!(s.place.longitude.approx_eq_ulps(&43.7224985, 2));
    assert!(s.place.latitude.approx_eq_ulps(&10.3970522, 2));
    assert_eq!(s.place.favorite, false);
    assert_eq!(s.place.reviews, 3866);
    assert_eq!(s.place.rating, Some(4.5));
    assert_eq!(s.place.telephone, None);
}

#[test]
fn test_scalar_struct() {
    let c = make();

    // Deserialize a scalar struct that has lots of different
    // data types
    let p: Place = c.get("place").unwrap();

    assert_eq!(p.name, "Torre di Pisa");
    assert!(p.longitude.approx_eq_ulps(&43.7224985, 2));
    assert!(p.latitude.approx_eq_ulps(&10.3970522, 2));
    assert_eq!(p.favorite, false);
    assert_eq!(p.reviews, 3866);
    assert_eq!(p.rating, Some(4.5));
    assert_eq!(p.telephone, None);
}

#[test]
fn test_array_scalar() {
    let c = make();
    let arr: Vec<i64> = c.get("arr").unwrap();

    assert_eq!(arr.len(), 10);
    assert_eq!(arr[3], 4);
}

#[test]
fn test_struct_array() {
    #[derive(Debug, Deserialize)]
    struct Settings {
        #[serde(rename = "arr")]
        elements: Vec<String>,
    }

    let c = make();
    let s: Settings = c.deserialize().unwrap();

    assert_eq!(s.elements.len(), 10);
    assert_eq!(s.elements[3], "4".to_string());
}
