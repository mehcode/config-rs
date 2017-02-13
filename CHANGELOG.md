# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## 0.4.0 - 2017-02-12
 - Remove global ( `config::get` ) API — It's now required to create a local configuration instance with `config::Config::new()` first

## 0.3.0 - 2017-02-08
 - YAML from [@tmccombs](https://github.com/tmccombs)
 - Nested field retrieval
 - Deep merging of sources (was shallow)
 - `config::File::from_str` to parse and merge a file from a string
 - Support for retrieval of maps and slices — `config::get_table` and `config::get_array`

## 0.2.0 - 2017-01-29
Initial release.
