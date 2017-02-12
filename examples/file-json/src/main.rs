extern crate config;

fn main() {
    let mut c = config::Config::new();

    // Read configuration from "Settings.json"
    c.merge(config::File::new("Settings", config::FileFormat::Json)).unwrap();

    println!("debug  = {:?}", c.get("debug"));
    println!("pi     = {:?}", c.get("pi"));
    println!("weight = {:?}", c.get("weight"));
}
