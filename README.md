# config-rs
[![Crates.io](https://img.shields.io/crates/v/config.svg)](https://crates.io/crates/config)
[![Crates.io](https://img.shields.io/crates/dv/config.svg)](https://crates.io/crates/config)
[![Build Status](https://travis-ci.org/mehcode/config-rs.svg?branch=master)](https://travis-ci.org/mehcode/config-rs)
> Layered configuration system for Rust applications (with strong support for [12-factor] applications).

[12-factor]: https://12factor.net/config
 
 - Set defaults
 - Set explicit values (to programmatically override)
 - Read from [JSON] and [TOML] files
 - Read from environment
 - Loosely typed — Configuration values may be read in any supported type, as long as there exists a reasonable conversion

[JSON]: https://github.com/serde-rs/json
[TOML]: https://github.com/toml-lang/toml

## Install

```toml
[dependencies]
config = "0.2"
```

#### Features

 - **json** - Adds support for reading JSON files
 - **toml** - Adds support for reading TOML files (included by default)

## Usage

Configuration is gathered by building a `Source` and then merging that source into the 
current state of the configuration.

```rust
fn main() {
    // Add environment variables that begin with RUST_
    config::merge(config::Environment::new("RUST"));

    // Add 'Settings.json'
    config::merge(config::File::new("Settings", config::FileFormat::Json));

    // Add 'Settings.$(RUST_ENV).json`
    let name = format!("Settings.{}", config::get_str("env").unwrap());
    config::merge(config::File::new(&name, config::FileFormat::Json));
}
```

Note that in the above example the calls to `config::merge` could have 
been re-ordered to influence the priority as each successive merge 
is evaluated on top of the previous.

Configuration values can be retrieved with a call to `config::get` and then 
coerced into a type with `as_*`.

```toml
# Settings.toml
debug = 1
```

```rust
fn main() {
    // Add 'Settings.toml' (from above)
    config::merge(config::File::new("Settings", config::FileFormat::Toml));

    // Get 'debug' and coerce to a boolean
    assert_eq!(config::get("debug").unwrap().as_bool(), Some(true));

    // You can use a type suffix on `config::get` to simplify
    assert_eq!(config::get_bool("debug"), Some(true));
    assert_eq!(config::get_str("debug"), Some("true"));
}
```

See the [examples](https://github.com/mehcode/config-rs/tree/master/examples) for 
more usage information.

## Roadmap
 - [ ] Read from remote source — [etcd](https://github.com/jimmycuadra/rust-etcd) and [consul](https://github.com/stusmall/consul-rust)
 - [ ] Read from YAML files
 - [ ] Read from Libconfig files
 
All suggestions are welcome. Please make an issue.

## License

config-rs is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
