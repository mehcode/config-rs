#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[macro_use]
extern crate serde;

extern crate nom;

#[cfg(feature = "toml")]
extern crate toml;

mod error;
mod value;
mod de;
mod path;
mod source;
mod config;
mod file;

pub use config::Config;
pub use error::ConfigError;
pub use value::Value;
pub use source::Source;
pub use file::{File, FileFormat};
