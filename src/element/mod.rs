use std::collections::HashMap;

#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub enum ConfigElement<'a> {
    Null,
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Str(&'a str),
    List(Vec<ConfigElement<'a>>),
    Map(HashMap<&'a str, ConfigElement<'a>>),
}

pub trait AsConfigElement {
    type Error: std::error::Error;

    fn as_config_element<'a>(&'a self) -> Result<ConfigElement<'a>, Self::Error>;
}

#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "toml")]
pub mod toml;

