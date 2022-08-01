#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Accessor parser error")]
    AccessorParseError(#[from] crate::accessor::AccessorParseError),

    #[error("Config object access error")]
    ConfigObjectAccessError(#[from] crate::object::ConfigObjectAccessError),

    #[error("Error loading Source")]
    SourceError(#[from] crate::source::SourceError),

    #[error("RwLock poisoned")]
    InternalRwLockPoisioned,

    #[error("Configuration is not loaded")]
    NotLoaded,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigBuilderError<E> {
    Wrapped(E),
}
