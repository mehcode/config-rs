use std::env;
use config::{ConfigError, Config, File, Environment};

#[derive(Debug, Deserialize)]
struct Database {
    url: String,
}

#[derive(Debug, Deserialize)]
struct Sparkpost {
    key: String,
    token: String,
    url: String,
    version: u8,
}

#[derive(Debug, Deserialize)]
struct Twitter {
    consumer_token: String,
    consumer_secret: String,
}

#[derive(Debug, Deserialize)]
struct Braintree {
    merchant_id: String,
    public_key: String,
    private_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    debug: bool,
    database: Database,
    sparkpost: Sparkpost,
    twitter: Twitter,
    braintree: Braintree,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/default"))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let env = env::var("RUN_MODE").unwrap_or("development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("config/local").required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app"))?;

        // You may also programmatically change settings
        s.set("database.url", "postgres://")?;

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", s.get_bool("debug"));
        println!("database: {:?}", s.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
