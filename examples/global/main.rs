#[macro_use]
extern crate lazy_static;

extern crate config;

use config::{builder::DefaultState, ConfigBuilder, Config};
use std::error::Error;
use std::sync::RwLock;

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

fn try_main() -> Result<(), Box<dyn Error>> {
    // Set property
    {
        let mut settings = SETTINGS.write()?;
        *settings = ConfigBuilder::<DefaultState>::default()
            .add_source(std::mem::take(&mut *settings))
            .set_override("property", 42)?
            .build()?;
    }

    // Get property
    println!("property: {}", SETTINGS.read()?.get::<i32>("property")?);

    Ok(())
}

fn main() {
    try_main().unwrap()
}
