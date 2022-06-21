use crate::accessor::Accessor;
use crate::description::ConfigSourceDescription;
use crate::element::ConfigElement;

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
        accessor: &mut Accessor,
    ) -> Result<Option<&ConfigElement<'a>>, ConfigObjectAccessError> {
        self.element.get(accessor)
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
