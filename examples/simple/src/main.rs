use std::collections::MapImpl;

fn main() {
    let mut settings = config::Config::default();
    settings
        // Add in `./Settings.toml`
        .merge(config::File::with_name("Settings")).unwrap()
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .merge(config::Environment::with_prefix("APP")).unwrap();

    // Print out our settings (as a MapImpl)
    println!("{:?}",
             settings.try_into::<MapImpl<String, String>>().unwrap());
}
