use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait Node<T> {
    fn insert(&mut self, data: T);
    fn remove(&mut self, id: &str);
    fn get(&self, id: &str) -> Option<T>;
    fn get_mut(&mut self, id: &str) -> Option<&mut T>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn clear(&mut self);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataNode<T> {
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
