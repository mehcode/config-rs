//! Config organizes hierarchical or layered configurations for Rust applications.
//!
//! Config lets you set a set of default parameters and then extend them via merging in
//! configuration from a variety of sources:
//!
//!  - Environment variables
//!  - String literals in well-known formats
//!  - Another Config instance
//!  - Files: TOML, JSON, YAML, INI, RON, JSON5 and custom ones defined with Format trait
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
#![allow(unused_variables)]
#![allow(unknown_lints)]
// #![warn(missing_docs)]

extern crate serde;

#[cfg(test)]
extern crate serde_derive;

extern crate nom;

extern crate lazy_static;

#[cfg(feature = "toml")]
extern crate toml;

#[cfg(feature = "json")]
extern crate serde_json;

#[cfg(feature = "yaml")]
extern crate yaml_rust;

#[cfg(feature = "ini")]
extern crate ini;

#[cfg(feature = "ron")]
extern crate ron;

#[cfg(feature = "json5")]
extern crate json5_rs;

pub mod builder;
mod config;
mod de;
mod env;
mod error;
mod file;
mod format;
mod map;
mod path;
mod ser;
mod source;
mod value;

pub use crate::builder::{AsyncConfigBuilder, ConfigBuilder};
pub use crate::config::Config;
pub use crate::env::Environment;
pub use crate::error::ConfigError;
pub use crate::file::{File, FileFormat, FileSourceFile, FileSourceString, FileStoredFormat};
pub use crate::format::Format;
pub use crate::map::Map;
pub use crate::source::{AsyncSource, Source};
pub use crate::value::{Value, ValueKind};
