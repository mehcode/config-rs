use std::convert::{From, TryFrom};

// Variant for a configuration Value
// The additional Option<String> is used for the textual representation of the
// underlying type (to cache the string generation) but only if requested.
pub enum Value {
    String(String),
    Integer(i64, Option<String>),
    Float(f64, Option<String>),
    Boolean(bool, Option<String>),
}

// Conversion from type into variant
impl From<String> for Value {
    fn from(value: String) -> Value {
        Value::String(value)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(value: &'a str) -> Value {
        Value::String(value.into())
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Value {
        Value::Integer(value, None)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Value {
        Value::Float(value, None)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Value {
        Value::Boolean(value, None)
    }
}

// Conversion from variant into type
impl<'a> TryFrom<&'a mut Value> for &'a str {
    type Err = ();

    fn try_from(value: &mut Value) -> Result<&str, ()> {
        // When converting a non-string value into a string;
        // cache the conversion and return a reference

        if let Value::String(ref value) = *value {
            Ok(value)
        } else if let Value::Integer(value, ref mut text) = *value {
            if let Some(ref text) = *text {
                Ok(text)
            } else {
                *text = Some(value.to_string());

                Ok(text.as_ref().unwrap())
            }
        } else if let Value::Float(value, ref mut text) = *value {
            if let Some(ref text) = *text {
                Ok(text)
            } else {
                *text = Some(value.to_string());

                Ok(text.as_ref().unwrap())
            }
        } else if let Value::Boolean(value, ref mut text) = *value {
            if let Some(ref text) = *text {
                Ok(text)
            } else {
                *text = Some(value.to_string());

                Ok(text.as_ref().unwrap())
            }
        } else {
            Err(())
        }
    }
}

impl<'a> TryFrom<&'a mut Value> for i64 {
    type Err = ();

    fn try_from(value: &mut Value) -> Result<i64, ()> {
        if let Value::Integer(value, ..) = *value {
            Ok(value)
        } else if let Value::String(ref value) = *value {
            value.parse().map_err(|_| {
                // Drop specific error
            })
        } else if let Value::Boolean(value, ..) = *value {
            Ok(if value { 1 } else { 0 })
        } else if let Value::Float(value, ..) = *value {
            Ok(value.round() as i64)
        } else {
            Err(())
        }
    }
}

impl<'a> TryFrom<&'a mut Value> for f64 {
    type Err = ();

    fn try_from(value: &mut Value) -> Result<f64, ()> {
        if let Value::Float(value, ..) = *value {
            Ok(value)
        } else if let Value::String(ref value) = *value {
            value.parse().map_err(|_| {
                // Drop specific error
            })
        } else if let Value::Integer(value, ..) = *value {
            Ok(value as f64)
        } else if let Value::Boolean(value, ..) = *value {
            Ok(if value { 1.0 } else { 0.0 })
        } else {
            Err(())
        }
    }
}

impl<'a> TryFrom<&'a mut Value> for bool {
    type Err = ();

    fn try_from(value: &mut Value) -> Result<bool, ()> {
        if let Value::Boolean(value, ..) = *value {
            Ok(value)
        } else if let Value::String(ref value) = *value {
            Ok(match value.to_lowercase().as_ref() {
                "1" | "true" | "on" | "yes" => true,
                _ => false,
            })
        } else if let Value::Integer(value, ..) = *value {
            Ok(value != 0)
        } else if let Value::Float(value, ..) = *value {
            Ok(value != 0.0)
        } else {
            Err(())
        }
    }
}
