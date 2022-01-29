#![allow(deprecated)]
use config::Config;
use lazy_static::lazy_static;
use std::error::Error;
use std::sync::RwLock;

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

fn try_main() -> Result<(), Box<dyn Error>> {
    // Set property
    SETTINGS.write()?.set("property", 42)?;

    // Get property
    println!("property: {}", SETTINGS.read()?.get::<i32>("property")?);

    Ok(())
}

fn main() {
    try_main().unwrap();
}
