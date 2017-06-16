extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod settings;

use settings::Settings;

fn main() {
    let settings = Settings::new();

    // Print out our settings
    println!("{:?}", settings);
}
