pub trait ParsableAccessor {
    fn parse(&self) -> Result<Accessor, AccessorParseError>;
}

impl ParsableAccessor for &str {
    fn parse(&self) -> Result<Accessor, AccessorParseError> {
        use std::str::FromStr;

        // TODO: Make this non-trivial and bulletproof

        let accessor = self.split('.')
            .map(|s| {
                match usize::from_str(s) {
                    Ok(u) => AccessType::Index(u),
                    Err(_) => AccessType::Key(s.to_string())
                }
            })
            .collect();

        Ok(Accessor(accessor))
    }
}

impl ParsableAccessor for String {
    fn parse(&self) -> Result<Accessor, AccessorParseError> {
        let s: &str = self;
        ParsableAccessor::parse(&s)
    }
}

pub struct Accessor(Vec<AccessType>);

pub(crate) enum AccessType {
    Key(String),
    Index(usize),
}

impl Accessor {
    pub(crate) fn head(&self) -> Option<&AccessType> {
        self.0.get(0)
    }

    pub fn pop(mut self) -> Accessor {
        let _ = self.0.remove(0);
        Accessor(self.0)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AccessorParseError {
}

