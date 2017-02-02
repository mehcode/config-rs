use std::collections::HashMap;

use source::Source;
use value::Value;

// Nil source that does nothing to easily allow for optional files
pub struct Nil {}

impl Source for Nil {
    fn collect(&self) -> HashMap<String, Value> {
        HashMap::new()
    }
}
