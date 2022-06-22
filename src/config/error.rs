#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Accessor parser error")]
    AccessorParseError(#[from] crate::accessor::AccessorParseError),

    #[error("Config object access error")]
    ConfigObjectAccessError(#[from] crate::object::ConfigObjectAccessError),
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigBuilderError<E> {
    Wrapped(E),
}
