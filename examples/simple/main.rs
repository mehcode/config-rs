use config::{builder::DefaultState, ConfigBuilder, Environment, File};
use std::collections::HashMap;

fn main() {
    let settings = ConfigBuilder::<DefaultState>::default()
        // Add in `./Settings.toml`
        .add_source(File::with_name("Settings"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(Environment::with_prefix("APP"))
        .build()
        .unwrap();

    // Print out our settings (as a HashMap)
    println!(
        "{:?}",
        settings.try_into::<HashMap<String, String>>().unwrap()
    );
}
