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

            _ => {
                // Unhandled
            }
        }
    }
}

impl Source for File {
    fn build(&mut self) -> Result<HashMap<String, Value>, Box<Error>> {
        // Find file
        // TODO: Use a nearest algorithm rather than strictly CWD
        let cwd = env::current_dir()?;
        let filename = cwd.join(self.name.clone() + ".toml");

        // Read contents from file
        let mut file = fs::File::open(filename)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        // Parse
        let mut parser = toml::Parser::new(&buffer);
        // TODO: Get a solution to make this return an Error-able
        let document = parser.parse().unwrap();

        // Iterate through document and fill content
        let mut content = HashMap::new();
        collect(&mut content, &document, None);

        Ok(content)
    }
}
