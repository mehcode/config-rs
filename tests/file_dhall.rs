#![cfg(feature = "dhall")]

extern crate config;
extern crate float_cmp;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

use config::*;
use float_cmp::ApproxEqUlps;

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
}

fn make() -> Config {
    Config::builder()
        .add_source(File::new("tests/Settings", FileFormat::Dhall))
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
    assert_eq!(
        s.place.creator["name"].clone().into_string().unwrap(),
        "John Smith".to_string()
    );
}

#[test]
fn test_dhall_vec() {
    let c = Config::builder()
        .add_source(File::from_str(
            r#"
            {
              WASTE = ["example_dir1", "example_dir2"]
            }
            "#,
            FileFormat::Dhall,
        ))
        .build()
        .unwrap();

    let v = c.get_array("WASTE").unwrap();
    let mut vi = v.into_iter();
    assert_eq!(vi.next().unwrap().into_string().unwrap(), "example_dir1");
    assert_eq!(vi.next().unwrap().into_string().unwrap(), "example_dir2");
    assert!(vi.next().is_none());
}
