use std::fs;
use std::env;
use std::error::Error;
use std::io::Read;
use std::collections::HashMap;

use toml;

use value::Value;

pub trait Source {
    fn build(&mut self) -> Result<HashMap<String, Value>, Box<Error>>;
}

#[derive(Default)]
pub struct File {
    // Basename of configuration file
    name: String,

    // Namespace to restrict configuration from the file
    namespace: Option<String>,

    // A required File will error if it cannot be found
    required: bool,
}

impl File {
    pub fn with_name(name: &str) -> File {
        File {
            name: name.into(),
            required: true,

            ..Default::default()
        }
    }

    pub fn namespace(&mut self, namespace: &str) -> &mut File {
        self.namespace = Some(namespace.into());
        self
    }

    pub fn required(&mut self, required: bool) -> &mut File {
        self.required = required;
        self
    }
}

fn collect(content: &mut HashMap<String, Value>, table: &toml::Table, prefix: Option<String>) {
    for (key, value) in table {
        // Construct full key from prefix
        let key = if let Some(ref prefix) = prefix {
            prefix.clone() + "." + key
        } else {
            key.clone()
        };

        match *value {
            // Recurse into nested table
            toml::Value::Table(ref table) => collect(content, table, Some(key)),

            toml::Value::String(ref value) => {
                content.insert(key, value.clone().into());
            }

            toml::Value::Integer(value) => {
                content.insert(key, value.into());
            }

            toml::Value::Float(value) => {
                content.insert(key, value.into());
            }

            toml::Value::Boolean(value) => {
                content.insert(key, value.into());
            }

            _ => {
                // Unhandled
            }
        }
    }
}

impl Source for File {
    fn build(&mut self) -> Result<HashMap<String, Value>, Box<Error>> {
        let mut content = HashMap::new();

        // Find file
        // TODO: Use a nearest algorithm rather than strictly CWD
        let cwd = match env::current_dir() {
            Ok(cwd) => cwd,
            Err(err) => {
                if self.required {
                    return Err(From::from(err));
                } else {
                    return Ok(content);
                }
            }
        };

        let filename = cwd.join(self.name.clone() + ".toml");

        // Read contents from file
        let mut file = match fs::File::open(filename) {
            Ok(file) => file,
            Err(err) => {
                if self.required {
                    return Err(From::from(err));
                } else {
                    return Ok(content);
                }
            }
        };

        let mut buffer = String::new();
        let res = file.read_to_string(&mut buffer);
        if res.is_err() {
            if self.required {
                return Err(From::from(res.err().unwrap()));
            } else {
                return Ok(content);
            }
        }

        // Parse
        let mut parser = toml::Parser::new(&buffer);
        // TODO: Get a solution to make this return an Error-able
        let document = parser.parse().unwrap();

        // Iterate through document and fill content
        collect(&mut content, &document, None);

        Ok(content)
    }
}
