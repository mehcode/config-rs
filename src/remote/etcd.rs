use std::sync::{Arc, Mutex};
use etcd::{kv, Client, BasicAuth};
use hyper::client::{Connect, HttpConnector};
use std::collections::HashMap;
use tokio_core::reactor::{Core, Remote};
use std::fmt::Debug;
use source::Source;
use value::Value;
use thread_local::ThreadLocal;
use error::Result;
use std::iter::Iterator;
use std::iter::FromIterator;

#[derive(Clone, Debug)]
pub struct Etcd
// where
    // C: Clone + Connect + Debug
{
    path: String,
    endpoints: Vec<String>,
    basic_auth: Option<BasicAuth>,
}

// impl Etcd<HttpConnector>  {
impl Etcd {
    pub fn new(endpoints: &[&str], basic_auth: Option<BasicAuth>) -> Self {
        return Etcd {
            path: "/".into(),
            endpoints: Vec::from_iter(endpoints.iter().map(|e| e.to_string())),
            basic_auth,
        }
    }
}

impl Etcd {
// impl<C> Etcd<C> where C: Clone + Connect + Debug {
    pub fn with_path(mut self, path: &str) -> Self {
        self.path = path.into();
        self
    }
}

// impl<C> Source for Etcd<C> where C: Clone + Connect + Debug + Send + Sync {
impl Source for Etcd {
    fn clone_into_box(&self) -> Box<Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<HashMap<String, Value>> {
        let mut core = Core::new()?;
        let endpoints = self.endpoints.iter()
            .map(|e| e.as_ref())
            .collect::<Vec<_>>();

        let client = Client::new(&core.handle(), endpoints.as_slice(),
            self.basic_auth.clone())?;

        let promise = kv::get(&client, &self.path, kv::GetOptions {
            strong_consistency: false,
            sort: false,
            recursive: true,
        }).then(move |response| {
            println!("{:?}", response.data);
        });

        Ok(HashMap::new())
    }
}
