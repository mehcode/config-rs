extern crate config;

fn main() {
    // Read configuration from "Settings.json"
    config::merge(config::File::new("Settings", config::FileFormat::Json)).unwrap();

    println!("debug  = {:?}", config::get("debug"));
    println!("pi     = {:?}", config::get("pi"));
    println!("weight = {:?}", config::get("weight"));
}
