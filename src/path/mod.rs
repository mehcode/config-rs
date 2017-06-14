use std::str::FromStr;
use std::collections::HashMap;
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
        parser::from_str(s).map_err(ConfigError::PathParse)
    }
}

impl Expression {
    pub fn get(self, root: &Value) -> Option<&Value> {
        match self {
            Expression::Identifier(id) => {
                match root.kind {
                    // `x` access on a table is equivalent to: map[x]
                    ValueKind::Table(ref map) => map.get(&id),

                    // all other variants return None
                    _ => None,
                }
            }

            Expression::Child(expr, key) => {
                match expr.get(root) {
                    Some(value) => {
                        match value.kind {
                            // Access on a table is identical to Identifier, it just forwards
                            ValueKind::Table(ref map) => map.get(&key),

                            // all other variants return None
                            _ => None,
                        }
                    }

                    _ => None,
                }
            }

            _ => {
                unimplemented!();
            }
        }
    }

    pub fn get_mut<'a>(&self, root: &'a mut Value) -> Option<&'a mut Value> {
        match *self {
            Expression::Identifier(ref id) => {
                match root.kind {
                    ValueKind::Table(ref mut map) => {
                        Some(map.entry(id.clone()).or_insert_with(|| Value::new(None, ValueKind::Nil)))
                    }

                    _ => None,
                }
            }

            Expression::Child(ref expr, ref key) => {
                match expr.get_mut(root) {
                    Some(value) => {
                        match value.kind {
                            ValueKind::Table(ref mut map) => {
                                Some(map.entry(key.clone()).or_insert_with(|| Value::new(None, ValueKind::Nil)))
                            }

                            _ => {
                                *value = HashMap::<String, Value>::new().into();

                                if let ValueKind::Table(ref mut map) = value.kind {
                                    Some(map.entry(key.clone()).or_insert_with(|| Value::new(None, ValueKind::Nil)))
                                } else {
                                    println!("WHAT THE FUCK?");

                                    unreachable!();
                                }
                            }
                        }
                    }

                    _ => None,
                }
            }

            _ => {
                unimplemented!();
            }
        }
    }

    pub fn set<'a>(&self, root: &'a mut Value, value: Value) {
        match *self {
            Expression::Identifier(ref id) => {
                // Ensure that root is a table
                match root.kind {
                    ValueKind::Table(_) => { }

                    _ => {
                        *root = HashMap::<String, Value>::new().into();
                    }
                }

                match value.kind {
                    ValueKind::Table(ref incoming_map) => {
                        // Pull out another table
                        let mut target = if let ValueKind::Table(ref mut map) = root.kind {
                            map.entry(id.clone()).or_insert_with(|| HashMap::<String, Value>::new().into())
                        } else {
                            unreachable!();
                        };

                        // Continue the deep merge
                        for (key, val) in incoming_map {
                            Expression::Identifier(key.clone()).set(&mut target, val.clone());
                        }
                    }

                    _ => {
                        if let ValueKind::Table(ref mut map) = root.kind {
                            // Just do a simple set
                            map.insert(id.clone(), value);
                        }
                    }
                }
            }

            Expression::Child(ref expr, ref key) => {
                if let Some(parent) = expr.get_mut(root) {
                    match parent.kind {
                        ValueKind::Table(_) => {
                            Expression::Identifier(key.clone()).set(parent, value);
                        }

                        _ => {
                            // Didn't find a table. Oh well. Make a table and do this anyway
                            *parent = HashMap::<String, Value>::new().into();

                            Expression::Identifier(key.clone()).set(parent, value);
                        }
                    }
                }
            }

            _ => {
                unimplemented!();
            }
        }
    }
}
