extern crate config;
use std::env;
use std::collections::HashMap;

fn main() {
    env::set_var("debug_env","true");
    env::set_var("priority_env","1");
    env::set_var("key_env","sdjfdjkfjdkjfkj");

    let mut settings = config::Config::default();
    settings
        // Add in `./Settings.toml`
        .merge(config::File::with_name("Settings")).unwrap()
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .merge(config::Environment::with_prefix("APP")).unwrap();

    // Print out our settings (as a HashMap)
    println!("{:?}",
             settings.try_into::<HashMap<String, String>>().unwrap());
    env::remove_var("debug_env");
    env::remove_var("priority_env");
    env::remove_var("key_env");
}
