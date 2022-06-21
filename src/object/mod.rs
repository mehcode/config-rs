use crate::accessor::Accessor;
use crate::description::ConfigSourceDescription;
use crate::element::ConfigElement;
use crate::accessor::AccessType;

#[derive(Debug)]
pub struct ConfigObject<'a> {
    element: ConfigElement<'a>,
    source: ConfigSourceDescription,
}

impl<'a> ConfigObject<'a> {
    pub(crate) fn new(element: ConfigElement<'a>, source: ConfigSourceDescription) -> Self {
        Self { element, source }
    }

    pub(crate) fn get(
        &self,
        accessor: &Accessor,
    ) -> Result<Option<&ConfigElement<'a>>, ConfigObjectAccessError> {
        match (accessor.head(), &self.element) {
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

#[derive(Debug, thiserror::Error)]
pub enum ConfigObjectAccessError {
    #[error("Tried to access with no accessor")]
    NoAccessor,

    #[error("Accessed Null with key '{0}'")]
    AccessWithKeyOnNull(String),
    #[error("Accessed Bool with key '{0}'")]
    AccessWithKeyOnBool(String),
    #[error("Accessed i8 with key '{0}'")]
    AccessWithKeyOnI8(String),
    #[error("Accessed i16 with key '{0}'")]
    AccessWithKeyOnI16(String),
    #[error("Accessed i32 with key '{0}'")]
    AccessWithKeyOnI32(String),
    #[error("Accessed i64 with key '{0}'")]
    AccessWithKeyOnI64(String),
    #[error("Accessed u8 with key '{0}'")]
    AccessWithKeyOnU8(String),
    #[error("Accessed u16 with key '{0}'")]
    AccessWithKeyOnU16(String),
    #[error("Accessed u32 with key '{0}'")]
    AccessWithKeyOnU32(String),
    #[error("Accessed u64 with key '{0}'")]
    AccessWithKeyOnU64(String),
    #[error("Accessed f32 with key '{0}'")]
    AccessWithKeyOnF32(String),
    #[error("Accessed f64 with key '{0}'")]
    AccessWithKeyOnF64(String),
    #[error("Accessed String with key '{0}'")]
    AccessWithKeyOnStr(String),
    #[error("Accessed List with key '{0}'")]
    AccessWithKeyOnList(String),

    #[error("Accessed Null with index '{0}'")]
    AccessWithIndexOnNull(usize),
    #[error("Accessed Bool with index '{0}'")]
    AccessWithIndexOnBool(usize),
    #[error("Accessed i8 with index '{0}'")]
    AccessWithIndexOnI8(usize),
    #[error("Accessed i16 with index '{0}'")]
    AccessWithIndexOnI16(usize),
    #[error("Accessed i32 with index '{0}'")]
    AccessWithIndexOnI32(usize),
    #[error("Accessed i64 with index '{0}'")]
    AccessWithIndexOnI64(usize),
    #[error("Accessed u8 with index '{0}'")]
    AccessWithIndexOnU8(usize),
    #[error("Accessed u16 with index '{0}'")]
    AccessWithIndexOnU16(usize),
    #[error("Accessed u32 with index '{0}'")]
    AccessWithIndexOnU32(usize),
    #[error("Accessed u64 with index '{0}'")]
    AccessWithIndexOnU64(usize),
    #[error("Accessed f32 with index '{0}'")]
    AccessWithIndexOnF32(usize),
    #[error("Accessed f64 with index '{0}'")]
    AccessWithIndexOnF64(usize),
    #[error("Accessed usize with index '{0}'")]
    AccessWithIndexOnStr(usize),
    #[error("Accessed Map with index '{0}'")]
    AccessWithIndexOnMap(usize),
}
