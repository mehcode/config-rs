# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## Unreleased

## 0.13.4 - 2023-11-22

- Relaxed the MSRV to 1.56.0 for this release. Please have a look at
  [#495] for details
- Backport of the following patches from PR [#465]:
  - [aa63d2dbbcc13fbdfa846185d54d87d7822e2509]
  - [831102fe0ffd5c7fe475efe5f379c710d201f165]
  - [147e6c7275b65b6a74eaec9c05b317673e61084e]
  - [ed6a3c9882fbc43eae9313ab1801610e49af863f]
  to fix nested arrays (see
  [the related bug report](https://github.com/mehcode/config-rs/issues/464)
  for details).

[#495]: https://github.com/mehcode/config-rs/pull/495
[#465]: https://github.com/mehcode/config-rs/pull/465
[aa63d2dbbcc13fbdfa846185d54d87d7822e2509]: https://github.com/mehcode/config-rs/commit/aa63d2dbbcc13fbdfa846185d54d87d7822e2509
[831102fe0ffd5c7fe475efe5f379c710d201f165]: https://github.com/mehcode/config-rs/commit/831102fe0ffd5c7fe475efe5f379c710d201f165
[147e6c7275b65b6a74eaec9c05b317673e61084e]: https://github.com/mehcode/config-rs/commit/147e6c7275b65b6a74eaec9c05b317673e61084e
[ed6a3c9882fbc43eae9313ab1801610e49af863f]: https://github.com/mehcode/config-rs/commit/ed6a3c9882fbc43eae9313ab1801610e49af863f

## 0.13.3 - 2022-12-04

Please note that we had to update the MSRV for this crate from 1.56.0 to 1.59.0
for this patch release being possible, because a transitive dependency did
update its MSRV.

 - Backport of commit [d54986c54091e4620c199d3dfadde80b82958bb3] from [#362] for
   using float_cmp for testing floats
 - Backport of [#379] adding `Clone` trait derive to builder states

[d54986c54091e4620c199d3dfadde80b82958bb3]: https://github.com/mehcode/config-rs/commit/d54986c54091e4620c199d3dfadde80b82958bb3
[#362]: https://github.com/mehcode/config-rs/pull/362
[#379]: https://github.com/mehcode/config-rs/pull/379

## 0.13.2 - 2022-08-02

 - Backport of [#316] to be testing with temp_env. The backport was necessary to
   be able to backport the next change. This change shouldn't be user-visible.
 - Backport of [#353] to use TryInto for more permissive deserialization of
   integers
 - Backport of commit [518a3cafa1e62ba7405709e5c508247e328e0a18] from [#362] to
   fix tests

[#316]: https://github.com/mehcode/config-rs/pull/316
[#353]: https://github.com/mehcode/config-rs/pull/353
[518a3cafa1e62ba7405709e5c508247e328e0a18]: https://github.com/mehcode/config-rs/commit/518a3cafa1e62ba7405709e5c508247e328e0a18
[#362]: https://github.com/mehcode/config-rs/pull/362

## 0.13.1 - 2022-04-13

 - typo in doc comment for ConfigBuilder [#299]
 - dot in config file name handling fixed [#306]

[#299]: https://github.com/mehcode/config-rs/pull/299
[#306]: https://github.com/mehcode/config-rs/pull/306

## 0.13.0 - 2022-04-03

 - Prefix-Seperator support was added [#292]
 - Environment lists can now be parsed [#255]
 - Setting an overwrite from an Option was added [#303]
 - Option to keep the prefix from an environment variable was added [#298]
 - Some small doc/CI fixes [#307], [#309]
 - MSRV was updated to 1.56.0 [#304]
 - Dependencies were updated [#289], [#301]

[#292]: https://github.com/mehcode/config-rs/pull/292
[#255]: https://github.com/mehcode/config-rs/pull/255
[#303]: https://github.com/mehcode/config-rs/pull/303
[#298]: https://github.com/mehcode/config-rs/pull/298
[#307]: https://github.com/mehcode/config-rs/pull/307
[#309]: https://github.com/mehcode/config-rs/pull/309
[#304]: https://github.com/mehcode/config-rs/pull/304
[#289]: https://github.com/mehcode/config-rs/pull/289
[#301]: https://github.com/mehcode/config-rs/pull/301

## 0.12.0 - 2022-02-10

### Format support changes in this version

 - HJSON support was removed [#230]
 - JSON5 format support [#206]
 - RON format support [#202]

### Other noteworthy changes

 - A new ConfigBuilder interface for building configuration objects [#196]
 - Asynchronous sources [#207]
 - Custom ENV separators are now supported [#185]
 - Loads of dependency updates and bugfixes of course
 - Preserved map order [#217]
 - Support for parsing numbers from the environment [#137]
 - Support for unsigned integers [#178]
 - `Format` trait for (custom) file formats [#219]

### Deprecated

 - `Environment::new()` - see [#235]
 - Large parts of the `Config` interface - see [#196]
     - `Config::merge()`
     - `Config::with_merged()`
     - `Config::refresh()`
     - `Config::set_default()`
     - `Config::set()`
     - `Config::set_once()`
     - `Config::deserialize()`

[#137]: https://github.com/mehcode/config-rs/pull/137
[#178]: https://github.com/mehcode/config-rs/pull/178
[#185]: https://github.com/mehcode/config-rs/pull/185
[#196]: https://github.com/mehcode/config-rs/pull/196
[#202]: https://github.com/mehcode/config-rs/pull/202
[#206]: https://github.com/mehcode/config-rs/pull/206
[#207]: https://github.com/mehcode/config-rs/pull/207
[#217]: https://github.com/mehcode/config-rs/pull/217
[#219]: https://github.com/mehcode/config-rs/pull/219
[#230]: https://github.com/mehcode/config-rs/pull/230
[#235]: https://github.com/mehcode/config-rs/pull/235

## 0.11.0 - 2021-03-17
 - The `Config` type got a builder-pattern `with_merged()` method [#166].
 - A `Config::set_once()` function was added, to set an value that can be
   overwritten by `Config::merge`ing another configuration [#172]
 - serde_hjson is, if enabled, pulled in without default features.
   This is due to a bug in serde_hjson, see [#169] for more information.
 - Testing is done on github actions [#175]

[#166]: https://github.com/mehcode/config-rs/pull/166
[#172]: https://github.com/mehcode/config-rs/pull/172
[#169]: https://github.com/mehcode/config-rs/pull/169
[#175]: https://github.com/mehcode/config-rs/pull/169

## 0.10.1 - 2019-12-07
 - Allow enums as configuration keys [#119]

[#119]: https://github.com/mehcode/config-rs/pull/119

## 0.10.0 - 2019-12-07
 - Remove lowercasing of keys (unless the key is coming from an environment variable).
 - Update nom to 5.x

## 0.9.3 - 2019-05-09
 - Support deserializing to a struct with `#[serde(default)]` [#106]

[#106]: https://github.com/mehcode/config-rs/pull/106

## 0.9.2 - 2019-01-03
 - Support reading `enum`s from configuration. [#85]
 - Improvements to error path (attempting to propagate path). [#89]
 - Fix UB in monomorphic expansion. We weren't re-exporting dependent types. [#91]

[#85]: https://github.com/mehcode/config-rs/pull/85
[#89]: https://github.com/mehcode/config-rs/pull/89
[#91]: https://github.com/mehcode/config-rs/issues/91

## 0.9.1 - 2018-09-25
 - Allow Environment variable collection to ignore empty values. [#78]
   ```rust
   // Empty env variables will not be collected
   Environment::with_prefix("APP").ignore_empty(true)
   ```

[#78]: https://github.com/mehcode/config-rs/pull/78

## 0.9.0 - 2018-07-02
 - **Breaking Change:** Environment does not declare a separator by default.
    ```rust
    // 0.8.0
    Environment::with_prefix("APP")

    // 0.9.0
    Environment::with_prefix("APP").separator("_")
    ```

 - Add support for INI. [#72]
 - Add support for newtype structs. [#71]
 - Fix bug with array set by path. [#69]
 - Update to nom 4. [#63]

[#72]: https://github.com/mehcode/config-rs/pull/72
[#71]: https://github.com/mehcode/config-rs/pull/71
[#69]: https://github.com/mehcode/config-rs/pull/69
[#63]: https://github.com/mehcode/config-rs/pull/63

## 0.8.0 - 2018-01-26
 - Update lazy_static and yaml_rust

## 0.7.1 - 2018-01-26
 - Be compatible with nom's verbose_errors feature (#50)[https://github.com/mehcode/config-rs/pull/50]
 - Add `derive(PartialEq)` for Value (#54)[https://github.com/mehcode/config-rs/pull/54]

## 0.7.0 - 2017-08-05
 - Fix conflict with `serde_yaml`. [#39]

[#39]: https://github.com/mehcode/config-rs/issues/39

 - Implement `Source` for `Config`.
 - Implement `serde::de::Deserializer` for `Config`. `my_config.deserialize` may now be called as either `Deserialize::deserialize(my_config)` or `my_config.try_into()`.
 - Remove `ConfigResult`. The builder pattern requires either `.try_into` as the final step _or_ the initial `Config::new()` to be bound to a slot. Errors must also be handled on each call instead of at the end of the chain.


    ```rust
    let mut c = Config::new();
    c
        .merge(File::with_name("Settings")).unwrap()
        .merge(Environment::with_prefix("APP")).unwrap();
    ```

    ```rust
    let c = Config::new()
        .merge(File::with_name("Settings")).unwrap()
        .merge(Environment::with_prefix("APP")).unwrap()
        // LLVM should be smart enough to remove the actual clone operation
        // as you are cloning a temporary that is dropped at the same time
        .clone();
    ```

    ```rust
    let mut s: Settings = Config::new()
        .merge(File::with_name("Settings")).unwrap()
        .merge(Environment::with_prefix("APP")).unwrap()
        .try_into();
    ```

## 0.6.0 – 2017-06-22
  - Implement `Source` for `Vec<T: Source>` and `Vec<Box<Source>>`

    ```rust
    Config::new()
        .merge(vec![
            File::with_name("config/default"),
            File::with_name(&format!("config/{}", run_mode)),
        ])
    ```

  - Implement `From<&Path>` and `From<PathBuf>` for `File`

  - Remove `namespace` option for File
  - Add builder pattern to condense configuration

    ```rust
    Config::new()
        .merge(File::with_name("Settings"))
        .merge(Environment::with_prefix("APP"))
        .unwrap()
    ```

 - Parsing errors even for non required files – [@Anthony25] ( [#33] )

[@Anthony25]: https://github.com/Anthony25
[#33]: https://github.com/mehcode/config-rs/pull/33

## 0.5.1 – 2017-06-16
 - Added config category to Cargo.toml

## 0.5.0 – 2017-06-16
 - `config.get` has been changed to take a type parameter and to deserialize into that type using serde. Old behavior (get a value variant) can be used by passing `config::Value` as the type parameter: `my_config.get::<config::Value>("..")`. Some great help here from [@impowski] in [#25].
 - Propagate parse and type errors through the deep merge (remembering filename, line, etc.)
 - Remove directory traversal on `File`. This is likely temporary. I do _want_ this behavior but I can see how it should be optional. See [#35]
 - Add `File::with_name` to get automatic file format detection instead of manual `FileFormat::*` – [@JordiPolo]
 - Case normalization [#26]
 - Remove many possible panics [#8]
 - `my_config.refresh()` will do a full re-read from the source so live configuration is possible with some work to watch the file

[#8]: https://github.com/mehcode/config-rs/issues/8
[#35]: https://github.com/mehcode/config-rs/pull/35
[#26]: https://github.com/mehcode/config-rs/pull/26
[#25]: https://github.com/mehcode/config-rs/pull/25

[@impowski]: https://github.com/impowski
[@JordiPolo]: https://github.com/JordiPolo

## 0.4.0 - 2017-02-12
 - Remove global ( `config::get` ) API — It's now required to create a local configuration instance with `config::Config::new()` first.

   If you'd like to have a global configuration instance, use `lazy_static!` as follows:

   ```rust
   use std::sync::RwLock;
   use config::Config;

   lazy_static! {
       static ref CONFIG: RwLock<Config> = Default::default();
   }
   ```

## 0.3.0 - 2017-02-08
 - YAML from [@tmccombs](https://github.com/tmccombs)
 - Nested field retrieval
 - Deep merging of sources (was shallow)
 - `config::File::from_str` to parse and merge a file from a string
 - Support for retrieval of maps and slices — `config::get_table` and `config::get_array`

## 0.2.0 - 2017-01-29
Initial release.
