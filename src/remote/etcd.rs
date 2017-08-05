use std::sync::{Arc, Mutex};
use etcd::{self, kv, BasicAuth, Client};
use hyper::client::{Connect, HttpConnector};
use std::collections::HashMap;
use tokio_core::reactor::Core;
use std::fmt::Debug;
use source::Source;
use value::{Table, Value};
use thread_local::ThreadLocal;
use error::*;
use std::iter::Iterator;
use std::iter::FromIterator;
use futures::Future;
use remote::Remote;

// TODO: uri!

#[derive(Clone, Debug)]
pub struct Etcd {
    prefix: String,
    paths: Option<Vec<String>>,
    endpoints: Vec<String>,
    basic_auth: Option<BasicAuth>,
}

// Joins 2 etcd paths together
fn etcd_path_join(a: &str, b: &str) -> String {
    let mut result = String::new();

    if !a.starts_with('/') {
        result.push('/');
    }

    result.push_str(&a);

    if a.ends_with('/') {
        if b.starts_with('/') {
            result.push_str(&b[1..]);
        } else {
            result.push_str(&b);
        }
    } else {
        if !b.starts_with('/') {
            result.push('/');
        }

        result.push_str(&b);
    }

    result
}

impl Etcd {
    pub fn new(endpoints: &[&str]) -> Self {
        return Etcd {
            prefix: "/".into(),
            paths: None,
            endpoints: Vec::from_iter(endpoints.iter().map(|e| e.to_string())),
            basic_auth: None,
        };
    }

    pub fn with_basic_auth(mut self, basic_auth: BasicAuth) -> Self {
        self.basic_auth = basic_auth.into();
        self
    }

    pub fn with_prefix(mut self, prefix: &str) -> Self {
        self.prefix = prefix.into();
        self
    }

    pub fn with_paths(mut self, paths: &[&str]) -> Self {
        self.paths = Some(Vec::from_iter(paths.iter().map(|e| e.to_string())));
        self
    }

    pub(crate) fn get_kv(&self, path: Option<&str>) -> Result<kv::Node> {
        let mut core = Core::new()?;
        let endpoints = self.endpoints
            .iter()
            .map(|e| e.as_ref())
            .collect::<Vec<_>>();

        let client = Client::new(
            &core.handle(),
            endpoints.as_slice(),
            self.basic_auth.clone(),
        )?;

        let mut key = self.prefix.clone();
        if let Some(path) = path {
            key = etcd_path_join(&key, path);
        }

        let work = kv::get(
            &client,
            &key,
            kv::GetOptions {
                strong_consistency: false,
                sort: false,
                recursive: true,
            },
        );

        let response = core.run(work)?;

        return Ok(response.data.node);
    }
}

fn etcd_key_to_config_property(key: &str, prefix: &str, include_parents: bool) -> String {
    if include_parents {
        // Replace `/` with `.`
        let mut result = key[prefix.len()..].replace('/', ".");

        if result.starts_with(".") {
            result = result[1..].into();
        }

        result
    } else {
        // Keep only the final bit
        key.split('/').last().unwrap_or_default().into()
    }
}

fn read_node_into_table(
    table: &mut Table,
    node: &kv::Node,
    uri: &String,
    key_prefix: &str,
    include_parents_in_key: bool,
) {
    let mut value: Option<Value> = None;

    // FIXME: What do we do if this neither has nodes nor a value?
    // FIXME: What do we do if this does not have a key?

    if let Some(ref nodes) = node.nodes {
        // This has children
        let mut child = HashMap::<String, Value>::new();
        for node in nodes {
            read_node_into_table(&mut child, node, uri, key_prefix, include_parents_in_key);
        }

        value = Some(Value::new(Some(uri), child));
    } else if let Some(ref node_value) = node.value {
        value = Some(Value::new(Some(uri), node_value.clone()));
    }

    if let Some(value) = value {
        if let Some(ref key) = node.key {
            table.insert(
                etcd_key_to_config_property(key, key_prefix, include_parents_in_key),
                value,
            );
        }
    }
}

impl Source for Etcd {
    fn clone_into_box(&self) -> Box<Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<HashMap<String, Value>> {
        let mut result = HashMap::new();
        let uri = self.uri();

        if let Some(ref paths) = self.paths {
            for path in paths {
                let node = self.get_kv(Some(&path))?;

                read_node_into_table(&mut result, &node, &uri, &self.prefix, true);
            }
        } else {
            let root = self.get_kv(None)?;

            if let Some(ref nodes) = root.nodes {
                for node in nodes {
                    read_node_into_table(&mut result, node, &uri, &self.prefix, false);
                }
            } else {
                return Err(ConfigError::Message(
                    format!("etcd path {:?} is not a directory", self.prefix),
                ));
            }
        }

        Ok(result)
    }
}

impl Remote for Etcd {
    fn clone_into_box(&self) -> Box<Remote + Send + Sync> {
        Box::new((*self).clone())
    }

    fn uri(&self) -> String {
        return format!("etcd{:?}", self.endpoints);
    }

    fn get(&self, key: &str) -> Result<String> {
        self.get_kv(Some(key)).and_then(|node| {
            if let Some(value) = node.value {
                Ok(value)
            } else {
                // TODO: When can this happen??
                Err(ConfigError::Message(
                    format!("remote key {:?} has no value", key),
                ))
            }
        })
    }
}
