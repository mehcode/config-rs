//! Configuration is gathered by building a `Source` and then merging that source into the
//! current state of the configuration.
//!
//! ```rust
//! extern crate config;
//!
//! use std::env;
//! use config::{Config, File, FileFormat, Environment};
//!
//! fn main() {
//!     // Create a new local configuration
//!     let mut c = Config::new();
//!
//!     // Add 'Settings.toml'
//!     c.merge(File::new("Settings", FileFormat::Toml).required(false)).unwrap();
//!
//!     // Add 'Settings.$(RUST_ENV).toml`
//!     let name = format!("Settings.{}", env::var("env").unwrap_or("development".into()));
//!     c.merge(File::new(&name, FileFormat::Toml).required(false)).unwrap();
//!
//!     // Add environment variables that begin with APP_
//!     c.merge(Environment::new("APP")).unwrap();
//! }
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
//! # extern crate config;
//! #
//! # use std::env;
//! # use config::{Config, File, FileFormat, Environment};
//! #
//! # fn main() {
//! #    // Create a new local configuration
//! #    let mut c = Config::new();
//! #
//! #    // Add 'Settings.toml'
//! #    c.merge(File::new("Settings", FileFormat::Toml).required(false)).unwrap();
//! #
//! #    // Add 'Settings.$(RUST_ENV).toml`
//! #    let name = format!("Settings.{}", env::var("env").unwrap_or("development".into()));
//! #    c.merge(File::new(&name, FileFormat::Toml).required(false)).unwrap();
//! #
//! #    // Add environment variables that begin with APP_
//! #    c.merge(Environment::new("APP")).unwrap();
//! // Get 'debug' and coerce to a boolean
//! if let Some(value) = c.get("debug") {
//!     println!("{:?}", value.into_bool());
//! }
//!
//! // You can use a type suffix
//! println!("{:?}", c.get_bool("debug"));
//! println!("{:?}", c.get_str("debug"));
//! # }
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

#[cfg(feature = "yaml")]
extern crate yaml_rust;

mod value;
mod source;
mod file;
mod env;
mod path;
mod config;

pub use source::{Source, SourceBuilder};
pub use file::{File, FileFormat};
pub use env::Environment;
pub use value::Value;
pub use config::Config;
