# config-rs
> Application Configuration for Rust

config-rs is a layered configuration system for Rust applications (including [12-factor]).

[12-factor]: https://12factor.net/config

## Install

```toml
[dependencies]
config = { git = "https://github.com/mehcode/config-rs.git" }
```

## Usage

### Setting Values

Configuration is collected in a series of layers, in order from lowest to highest priority.

1. Explicit Default — `config::set_default`
2. Source — File, Remote (ETCD, Consul, etc.)
3. Environment
4. Explicit Set — `config::set`

#### Defaults

By default, `None` is returned when requesting a configuration value
that does not exist elsewhere.

Defaults may be established in code.

```rust
config::set_default("port", 80);
```

#### Environment

```rust
// Keep your environment unique and predictable
config::set_env_prefix("rust");

// Enable environment
// config::bind_env("port");
// config::bind_env_to("port", "port");
config::bind_env_all();

// Environment variables are typically set outside of the application
std::env::set_var("RUST_PORT", "80");
std::env::set_var("RUST_PORT2", "602");

config::get_int("port");  //= Some(80)
config::get_int("port2"); //= Some(602)
```

#### Source

##### File

Read `${CWD}/Settings.toml` and populate configuration.
 - `prefix` is used to only pull keys nested under the named key
 - `required(true)` (default) will cause an error to be returned if the file failed to be read/parsed/etc.

```rust
config::merge(
    config::source::File::with_name("Settings")
        .required(false)
        .prefix("development")
).unwrap();
```

## Getting Values

Values will attempt to convert from their underlying type (from when they were set) when accessed.

 - `config::get::<T>(key: &str) -> T`
 - `config::get_str(key: &str) -> &str`
 - `config::get_int(key: &str) -> i64`
 - `config::get_float(key: &str) -> float`
 - `config::get_bool(key: &str) -> bool`

```rust
if config::get("debug").unwrap() {
    println!("address: {:?}", config::get_int("port"));
}
```
