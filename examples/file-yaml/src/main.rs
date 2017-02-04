extern crate config;

fn main() {
    // Read configuration from "Settings.yaml"
    config::merge(config::File::new("Settings", config::FileFormat::Yaml)).unwrap();

    println!("debug  = {:?}", config::get("debug"));
    println!("pi     = {:?}", config::get("pi"));
    println!("weight = {:?}", config::get("weight"));
}
