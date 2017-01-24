#![feature(try_from)]
#![feature(drop_types_in_const)]

extern crate toml;

mod value;
mod source;
mod config;

use std::error::Error;
use std::convert::TryFrom;
use std::sync::{Once, ONCE_INIT};

pub use source::Source;
pub use source::File;

use value::Value;

pub use config::Config;

// Global configuration
static mut CONFIG: Option<Config> = None;
static CONFIG_INIT: Once = ONCE_INIT;

// Get the global configuration instance
fn global() -> Option<&'static mut Config> {
    unsafe {
        CONFIG_INIT.call_once(|| {
            CONFIG = Some(Default::default());
        });

        // TODO(@rust): One-line this if possible
        if let Some(ref mut c) = CONFIG {
            return Some(c);
        }

        None
    }
}

pub fn merge<T>(source: T) -> Result<(), Box<Error>>
    where T: Source
{
    global().unwrap().merge(source)
}

pub fn set_env_prefix(prefix: &str) {
    global().unwrap().set_env_prefix(prefix)
}

pub fn set_default<T>(key: &str, value: T)
    where T: Into<Value>
{
    global().unwrap().set_default(key, value)
}

pub fn set<T>(key: &str, value: T)
    where T: Into<Value>
{
    global().unwrap().set(key, value)
}

pub fn get<'a, T>(key: &str) -> Option<T>
    where T: TryFrom<&'a mut Value>,
          T: Default
{
    global().unwrap().get(key)
}

pub fn get_str<'a>(key: &str) -> Option<&'a str> {
    global().unwrap().get_str(key)
}

pub fn get_int(key: &str) -> Option<i64> {
    global().unwrap().get_int(key)
}

pub fn get_float(key: &str) -> Option<f64> {
    global().unwrap().get_float(key)
}

pub fn get_bool(key: &str) -> Option<bool> {
    global().unwrap().get_bool(key)
}
