# config-rs
> Layered configuration system for Rust applications (with strong support for [12-factor]).

[12-factor]: https://12factor.net/config

## Install

```toml
[dependencies]
config = { git = "https://github.com/mehcode/config-rs.git" }
```

## Usage

Configuration is collected in a series of layers, in order from lowest to highest priority.

1. Explicit Default — `config::set_default`
2. Source — File
3. Environment
4. Explicit Set — `config::set`

```rust
// Set explicit defaults. This is optional but can be used
// to ensure a key always has a value.
config::set_default("port", 80);

assert_eq!(config::get_int("port"), 80);

// Merge in a configuration file
//
// ---
// [development]
// host = "::1"
// factor = 5.321
//
// [development.redis]
// port = 80
// ---
config::merge(config::File::with_name("Settings").namespace("development"));

assert_eq!(config::get_str("host"), Some("::1"));
assert_eq!(config::get_int("factor"), Some(5));
assert_eq!(config::get_str("redis.port"), Some("80"));

// Keep your environment unique and predictable by
// namespacing environment variable usage
config::set_env_prefix("rust");

// Environment variables would normally be set outside of the application
std::env::set_var("RUST_PORT", "80");
std::env::set_var("RUST_HOST", "::0");
std::env::set_var("RUST_DEBUG", "false");

assert_eq!(config::get_int("port"), Some(80));
assert_eq!(config::get_str("host"), Some("::0"));
assert_eq!(config::get_bool("debug"), Some(false));

// Set an explicit override of a key
confing::set("debug", true);

assert_eq!(config::get_bool("debug"), Some(true));
```
