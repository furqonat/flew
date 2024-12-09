use std::io::Error;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait Node<T>
where
    T: Clone + for<'de> Deserialize<'de> + Serialize,
{
    fn add(&mut self, node_name: DataNode<T>) -> Result<String, Error>;
    fn get(&self, id: &str) -> Option<&DataNode<T>>;
    fn delete(&mut self, id: &str) -> Result<(), Error>;
    fn update(&mut self, id: &str, data: DataNode<T>) -> Result<(), Error>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataNode<T>
where
    T: Clone,
{
    pub id: String,
    pub data: T,
}

impl<T> DataNode<T>
where
    T: Clone + for<'de> Deserialize<'de> + Serialize,
{
    pub fn new(data: T) -> Self {
        let id = Uuid::new_v4().to_string();
        DataNode { id, data }
    }
}
