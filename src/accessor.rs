pub trait ParsableAccessor {
    fn parse(&self) -> Result<Accessor, AccessorParseError>;
}

impl ParsableAccessor for &str {
    fn parse(&self) -> Result<Accessor, AccessorParseError> {
        unimplemented!()
    }
}

impl ParsableAccessor for String {
    fn parse(&self) -> Result<Accessor, AccessorParseError> {
        unimplemented!()
    }
}

pub struct Accessor;

#[derive(Debug, thiserror::Error)]
pub enum AccessorParseError {
}

