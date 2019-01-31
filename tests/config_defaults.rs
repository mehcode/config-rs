extern crate config;

#[macro_use]
extern crate serde_derive;

use config::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub db_host: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            db_host: String::from("default"),
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.set_defaults(Settings::default());
        s.try_into()
    }
}

#[test]
fn config_with_defaults(){
    let mut s = Settings::new().unwrap();
    assert_eq!(s.db_host, "default");
}
