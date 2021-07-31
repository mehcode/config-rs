use std::{collections::HashMap, error::Error};

use crate::value::Value;
use crate::map::Map;

pub trait Format {
    fn parse(
        &self,
        uri: Option<&String>,
        text: &str,
    ) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>>;
}
