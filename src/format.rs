use std::{collections::HashMap, error::Error};

use crate::value::Value;

pub trait Format {
    fn parse(
        &self,
        uri: Option<&String>,
        text: &str,
    ) -> Result<HashMap<String, Value>, Box<dyn Error + Send + Sync>>;
}
