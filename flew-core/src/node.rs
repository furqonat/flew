use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait Node<T>
where
    T: Clone + for<'de> Deserialize<'de> + Serialize,
{
    fn add(&mut self, node_name: DataNode<T>) -> String;
    fn get(&mut self, id: &str) -> Option<&mut DataNode<T>>;
    fn delete(&mut self, id: &str);
    fn update(&mut self, id: &str, data: DataNode<T>) -> &mut Self;
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
