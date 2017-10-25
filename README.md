# config-rs
![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)
[![Build Status](https://travis-ci.org/mehcode/config-rs.svg?branch=master)](https://travis-ci.org/mehcode/config-rs)
[![Crates.io](https://img.shields.io/crates/d/config.svg)](https://crates.io/crates/config)
[![Docs.rs](https://docs.rs/config/badge.svg)](https://docs.rs/config)
[![IRC](https://img.shields.io/badge/chat-%23config-yellow.svg)](https://kiwiirc.com/client/irc.mozilla.org/#config)
> Layered configuration system for Rust applications (with strong support for [12-factor] applications).

[12-factor]: https://12factor.net/config

 - Set defaults
 - Set explicit values (to programmatically override)
 - Read from [JSON], [TOML], [YAML] and [HJSON] files
 - Read from environment
 - Loosely typed — Configuration values may be read in any supported type, as long as there exists a reasonable conversion
 - Access nested fields using a formatted path — Uses a subset of JSONPath; currently supports the child ( `redis.port` ) and subscript operators ( `databases[0].name` )

[JSON]: https://github.com/serde-rs/json
[TOML]: https://github.com/toml-lang/toml
[YAML]: https://github.com/chyh1990/yaml-rust
[HJSON]: https://github.com/hjson/hjson-rust

## Usage

```toml
[dependencies]
config = "0.7"
```

 - `json` - Adds support for reading JSON files
 - `hjson` - Adds support for reading HJSON files
 - `yaml` - Adds support for reading YAML files
 - `toml` - Adds support for reading TOML files (included by default)

See the [documentation](https://docs.rs/config) or [examples](https://github.com/mehcode/config-rs/tree/master/examples) for
more usage information.

## License

config-rs is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
