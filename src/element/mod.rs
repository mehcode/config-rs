use std::collections::HashMap;

use crate::{accessor::{AccessType, Accessor}, object::ConfigObjectAccessError};

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

impl<'a> ConfigElement<'a> {
    pub(crate) fn get(&self, accessor: &mut Accessor) -> Result<Option<&ConfigElement<'a>>, ConfigObjectAccessError> {
        match (accessor.current(), &self) {
            (Some(AccessType::Key(k)), ConfigElement::Null) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnNull(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::Bool(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnBool(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::I8(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnI8(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::I16(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnI16(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::I32(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnI32(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::I64(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnI64(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::U8(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnU8(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::U16(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnU16(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::U32(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnU32(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::U64(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnU64(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::F32(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnF32(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::F64(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnF64(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::Str(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnStr(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::List(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnList(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::Map(hm)) => Ok(hm.get(k.as_str())),

            (Some(AccessType::Index(u)), ConfigElement::Null) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnNull(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::Bool(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnBool(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::I8(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnI8(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::I16(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnI16(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::I32(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnI32(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::I64(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnI64(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::U8(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnU8(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::U16(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnU16(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::U32(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnU32(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::U64(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnU64(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::F32(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnF32(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::F64(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnF64(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::Str(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnStr(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::List(v)) => Ok(v.get(*u)),
            (Some(AccessType::Index(u)), ConfigElement::Map(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnMap(*u))
            }

            (None, _) => Err(ConfigObjectAccessError::NoAccessor),
        }
    }
}

pub trait AsConfigElement {
    type Error: std::error::Error;

    fn as_config_element<'a>(&'a self) -> Result<ConfigElement<'a>, Self::Error>;
}

#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "toml")]
pub mod toml;

