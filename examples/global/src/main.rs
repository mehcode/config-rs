#[macro_use]
extern crate lazy_static;

extern crate config;

use std::error::Error;
use std::sync::RwLock;
use config::Config;

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
	try_main().unwrap()
}
