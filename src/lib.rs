#![feature(drop_types_in_const)]
#![allow(unknown_lints)]

extern crate toml;

mod value;
mod source;
mod config;

use std::error::Error;
use std::borrow::Cow;
use std::sync::{Once, ONCE_INIT};

pub use source::Source;
pub use source::File;

pub use value::Value;

pub use config::Config;

// Global configuration
static mut CONFIG: Option<Config> = None;
static CONFIG_INIT: Once = ONCE_INIT;

// Get the global configuration instance
fn global() -> &'static mut Config {
    unsafe {
        CONFIG_INIT.call_once(|| {
            CONFIG = Some(Default::default());
        });

        CONFIG.as_mut().unwrap()
    }
}

pub fn merge<T>(source: T) -> Result<(), Box<Error>>
    where T: Source
{
    global().merge(source)
}

pub fn set_env_prefix(prefix: &str) {
    global().set_env_prefix(prefix)
}

pub fn set_default<T>(key: &str, value: T)
    where T: Into<Value>
{
    global().set_default(key, value)
}

pub fn set<T>(key: &str, value: T)
    where T: Into<Value>
{
    global().set(key, value)
}

pub fn get<'a>(key: &str) -> Option<&'a Value> {
    global().get(key)
}

pub fn get_str<'a>(key: &str) -> Option<Cow<'a, str>> {
    global().get_str(key)
}

pub fn get_int(key: &str) -> Option<i64> {
    global().get_int(key)
}

pub fn get_float(key: &str) -> Option<f64> {
    global().get_float(key)
}

pub fn get_bool(key: &str) -> Option<bool> {
    global().get_bool(key)
}
