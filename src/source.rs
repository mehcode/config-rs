use crate::object::ConfigObject;

pub trait ConfigSource: std::fmt::Debug {
    type Error: std::error::Error;

    fn load<'a>(&'a self) -> Result<ConfigObject<'a>, Self::Error>;
}
