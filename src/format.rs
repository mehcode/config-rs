use std::{collections::HashMap, error::Error};

use crate::value::Value;

pub trait Format {
    fn parse(
        self,
        uri: Option<&String>,
        text: &str,
    ) -> Result<HashMap<String, Value>, Box<dyn Error + Send + Sync>>;
}

impl<F> Format for F 
where
    F: Fn(Option<&String>, &str) -> Result<HashMap<String, Value>, Box<dyn Error + Send + Sync>>
{
    fn parse(
        self,
        uri: Option<&String>,
        text: &str,
    ) -> Result<HashMap<String, Value>, Box<dyn Error + Send + Sync>> {
        self(uri, text)
    }
}