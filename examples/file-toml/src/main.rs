extern crate config;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Deserialize)]
struct Point { x: i64, y: i64 }

fn main() {
    let mut c = config::Config::default();

    // Read configuration from "Settings.toml"
    c.merge(config::File::new("Settings", config::FileFormat::Toml)).unwrap();

    // Simple key access to values
    println!("debug       = {}", c.get::<bool>("debug").unwrap());
    println!("pi          = {}", c.get::<f64>("pi").unwrap());
    println!("weight      = {}", c.get::<i64>("weight").unwrap());
    println!("location    = {:?}", c.get::<Point>("location").unwrap());
    // println!("location.x  = {}", c.get::<Point>("location.x").unwrap());
    // println!("location.y  = {}", c.get::<Point>("location.y").unwrap());
}
