//! Config organizes hierarchical or layered configurations for Rust applications.
//!
//! Config lets you set a set of default parameters and then extend them via merging in
//! configuration from a variety of sources:
//!
//!  - Environment variables
//!  - Another Config instance
//!  - Remote configuration: etcd, Consul
//!  - Files: JSON, YAML, TOML, HJSON
//!  - Manual, programmatic override (via a `.set` method on the Config instance)
//!
//! Additionally, Config supports:
//!
//!  - Live watching and re-reading of configuration files
//!  - Deep access into the merged configuration via a path syntax
//!  - Deserialization via `serde` of the configuration or any subset defined via a path
//!
//! See the [examples](https://github.com/mehcode/config-rs/tree/master/examples) for
//! general usage information.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unknown_lints)]
// #![warn(missing_docs)]

#[macro_use]
extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;

extern crate nom;

#[macro_use]
extern crate lazy_static;

#[cfg(feature = "toml")]
extern crate toml;

#[cfg(feature = "json")]
extern crate serde_json;

#[cfg(feature = "yaml")]
extern crate yaml_rust;

#[cfg(feature = "hjson")]
extern crate serde_hjson;

mod error;
mod value;
mod de;
mod ser;
mod path;
mod source;
mod config;
mod file;
mod env;

pub use config::Config;
pub use error::ConfigError;
pub use value::Value;
pub use source::Source;
pub use file::{File, FileFormat};
pub use env::Environment;
