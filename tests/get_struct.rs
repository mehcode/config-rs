extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

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
fn test_file_struct() {
    let c = make();

    // Deserialize the entire file as single struct
    let s: Settings = c.deserialize().unwrap();

    assert_eq!(s.debug, 1.0);
    assert_eq!(s.production, Some("false".to_string()));
    assert_eq!(s.place.name, "Torre di Pisa");
    assert_eq!(s.place.longitude, 43.7224985);
    assert_eq!(s.place.latitude, 10.3970522);
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
    assert_eq!(p.longitude, 43.7224985);
    assert_eq!(p.latitude, 10.3970522);
    assert_eq!(p.favorite, false);
    assert_eq!(p.reviews, 3866);
    assert_eq!(p.rating, Some(4.5));
    assert_eq!(p.telephone, None);
}
