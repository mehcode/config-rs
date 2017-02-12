extern crate config;

fn main() {
    let mut c = config::Config::new();

    // Read configuration from "Settings.yaml"
    c.merge(config::File::new("Settings", config::FileFormat::Yaml)).unwrap();

    println!("debug  = {:?}", c.get("debug"));
    println!("pi     = {:?}", c.get("pi"));
    println!("weight = {:?}", c.get("weight"));
}
