use std::borrow::Cow;

use source::Source;
use value::Value;

// Nil source that does nothing to easily allow for optional files
pub struct Nil {}

impl Source for Nil {
    fn get<'a>(&self, _: &str) -> Option<Cow<'a, Value>> {
        None
    }
}
