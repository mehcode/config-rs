use crate::element::ConfigElement;
use crate::description::ConfigSourceDescription;
use crate::accessor::Accessor;

#[derive(Debug)]
pub struct ConfigObject<'a> {
    element: ConfigElement<'a>,
    source: ConfigSourceDescription,
}

impl<'a> ConfigObject<'a> {
    pub(crate) fn get(&self, accessor: &Accessor) -> Result<Option<ConfigElement<'a>>, ConfigObjectAccessError> {
        unimplemented!()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigObjectAccessError {
}
