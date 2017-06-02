use std::str::FromStr;
use nom::ErrorKind;
use error::*;
use value::{Value, ValueKind};

mod parser;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Expression {
    Identifier(String),
    Child(Box<Expression>, String),
    Subscript(Box<Expression>, i32),
}

impl FromStr for Expression {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Expression> {
        parser::from_str(s).map_err(|kind| ConfigError::PathParse(kind))
    }
}

impl Expression {
    pub fn get<'a>(self, root: &'a Value) -> Option<&'a Value> {
        match self {
            Expression::Identifier(id) => {
                match root.kind {
                    // `x` access on a table is equivalent to: map[x]
                    ValueKind::Table(ref map) => map.get(&id),

                    // all other variants return None
                    _ => None,
                }
            }

            _ => {
                unimplemented!();
            }
        }
    }
}
