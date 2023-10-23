# config-rs

![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)
[![Build Status](https://travis-ci.org/mehcode/config-rs.svg?branch=master)](https://travis-ci.org/mehcode/config-rs)
[![Crates.io](https://img.shields.io/crates/d/config.svg)](https://crates.io/crates/config)
[![Docs.rs](https://docs.rs/config/badge.svg)](https://docs.rs/config)

> Layered configuration system for Rust applications (with strong support for [12-factor] applications).

[12-factor]: https://12factor.net/config

 - Set defaults
 - Set explicit values (to programmatically override)
 - Read from [JSON], [TOML], [YAML], [INI], [RON], [JSON5] files
 - Read from environment
 - Loosely typed — Configuration values may be read in any supported type, as long as there exists a reasonable conversion
 - Access nested fields using a formatted path — Uses a subset of JSONPath; currently supports the child ( `redis.port` ) and subscript operators ( `databases[0].name` )

[JSON]: https://github.com/serde-rs/json
[TOML]: https://github.com/toml-lang/toml
[YAML]: https://github.com/chyh1990/yaml-rust
[INI]: https://github.com/zonyitoo/rust-ini
[RON]: https://github.com/ron-rs/ron
[JSON5]: https://github.com/callum-oakley/json5-rs

Please note that this library can not be used to write changed configuration
values back to the configuration file(s)!

## Usage

```toml
[dependencies]
config = "0.13.1"
```

### Feature flags

 - `ini` - Adds support for reading INI files
 - `json` - Adds support for reading JSON files
 - `yaml` - Adds support for reading YAML files
 - `toml` - Adds support for reading TOML files
 - `ron` - Adds support for reading RON files
 - `json5` - Adds support for reading JSON5 files

### Support for custom formats

Library provides out of the box support for most renowned data formats such as JSON or Yaml. Nonetheless, it contains an extensibility point - a `Format` trait that, once implemented, allows seamless integration with library's APIs using custom, less popular or proprietary data formats.

See [custom_format](https://github.com/mehcode/config-rs/tree/master/examples/custom_format) example for more information.

### More

See the [documentation](https://docs.rs/config) or [examples](https://github.com/mehcode/config-rs/tree/master/examples) for
more usage information.


## MSRV

We currently support Rust 1.61.0 and newer.


## License

config-rs is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
