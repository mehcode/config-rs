extern crate config;

fn main() {
    let mut c = config::Config::new();

    // Read configuration from "Settings.toml"
    c.merge(config::File::new("Settings", config::FileFormat::Toml)).unwrap();

    println!("debug  = {:?}", c.get("debug"));
    println!("pi     = {:?}", c.get("pi"));
    println!("weight = {:?}", c.get("weight"));
}
