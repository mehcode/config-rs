#![feature(drop_types_in_const)]

#![allow(unknown_lints)]
#![feature(trace_macros)]

//! Configuration is gathered by building a `Source` and then merging that source into the
//! current state of the configuration.
//!
//! ```rust
//! // Add environment variables that begin with RUST_
//! config::merge(config::Environment::new("RUST")).unwrap();
//!
//! // Add 'Settings.toml'
//! config::merge(config::File::new("Settings", config::FileFormat::Toml)
//!     .required(false)).unwrap();
//!
//! // Add 'Settings.$(RUST_ENV).toml`
//! let name = format!("Settings.{}", config::get_str("env").unwrap_or("development".into()));
//! config::merge(config::File::new(&name, config::FileFormat::Toml)
//!     .required(false)).unwrap();
//! ```
//!
//! Note that in the above example the calls to `config::merge` could have
//! been re-ordered to influence the priority as each successive merge
//! is evaluated on top of the previous.
//!
//! Configuration values can be retrieved with a call to `config::get` and then
//! coerced into a type with `as_*`.
//!
//! ```rust
//! // Get 'debug' and coerce to a boolean
//! if let Some(value) = config::get("debug") {
//!     println!("{:?}", value.as_bool());
//! }
//!
//! // You can use a type suffix
//! println!("{:?}", config::get_bool("debug"));
//! println!("{:?}", config::get_str("debug"));
//! ```
//!
//! See the [examples](https://github.com/mehcode/config-rs/tree/master/examples) for
//! more usage information.

#[macro_use]
extern crate nom;

#[cfg(feature = "toml")]
extern crate toml;

#[cfg(feature = "json")]
extern crate serde_json;

mod value;
mod source;
mod file;
mod env;
mod path;
mod config;

use std::error::Error;
use std::sync::{Once, ONCE_INIT, RwLock};
use std::borrow::Cow;

pub use source::{Source, SourceBuilder};
pub use file::{File, FileFormat};
pub use env::Environment;

pub use value::Value;

pub use config::Config;

// Global configuration
static mut CONFIG: Option<RwLock<Config>> = None;
static CONFIG_INIT: Once = ONCE_INIT;

// Get the global configuration instance
pub fn global() -> &'static mut RwLock<Config> {
    unsafe {
        CONFIG_INIT.call_once(|| {
            CONFIG = Some(Default::default());
        });

        CONFIG.as_mut().unwrap()
    }
}

pub fn merge<T>(source: T) -> Result<(), Box<Error>>
    where T: SourceBuilder
{
    global().write().unwrap().merge(source)
}

pub fn set_default<T>(key: &str, value: T) -> Result<(), Box<Error>>
    where T: Into<Value>
{
    global().write().unwrap().set_default(key, value)
}

pub fn set<T>(key: &str, value: T) -> Result<(), Box<Error>>
    where T: Into<Value>
{
    global().write().unwrap().set(key, value)
}

pub fn get<'a>(key: &str) -> Option<&'a Value> {
    // TODO(~): Should this panic! or return None with an error message?
    //          Make an issue if you think it should be an error message.
    let r = global().read().unwrap();

    let c = &*r;

    // TODO(@rust): Figure out how to not to use unsafe here
    unsafe {
        let c: &'static Config = std::mem::transmute(c);
        c.get(key)
    }
}

pub fn get_str<'a>(key: &str) -> Option<Cow<'a, str>> {
    let r = global().read().unwrap();

    unsafe {
        let c: &'static Config = std::mem::transmute(&*r);
        c.get_str(key)
    }
}

pub fn get_int(key: &str) -> Option<i64> {
    let r = global().read().unwrap();

    unsafe {
        let c: &'static Config = std::mem::transmute(&*r);
        c.get_int(key)
    }
}

pub fn get_float(key: &str) -> Option<f64> {
    let r = global().read().unwrap();

    unsafe {
        let c: &'static Config = std::mem::transmute(&*r);
        c.get_float(key)
    }
}

pub fn get_bool(key: &str) -> Option<bool> {
    let r = global().read().unwrap();

    unsafe {
        let c: &'static Config = std::mem::transmute(&*r);
        c.get_bool(key)
    }
}
