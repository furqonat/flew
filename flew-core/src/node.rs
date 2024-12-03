use std::collections::HashMap;

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
pub struct NodeData<T> {
    pub id: String,
    pub data: HashMap<String, T>,
}

impl<T> Node<T> for NodeData<T>
where
    T: Clone + for<'de> Deserialize<'de> + Serialize,
{
    fn insert(&mut self, data: T) {
        let id = Uuid::new_v4().to_string();
        self.data.insert(id, data);
    }

    fn remove(&mut self, id: &str) {
        self.data.remove(id);
    }

    fn get(&self, id: &str) -> Option<T> {
        return self.data.get(id).cloned();
    }

    fn get_mut(&mut self, id: &str) -> Option<&mut T> {
        return self.data.get_mut(id);
    }

    fn len(&self) -> usize {
        return self.data.len();
    }

    fn is_empty(&self) -> bool {
        return self.data.is_empty();
    }

    fn clear(&mut self) {
        self.data.clear();
    }
}
