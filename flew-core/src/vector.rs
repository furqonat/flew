use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::node::DataNode;

#[derive(Debug, Clone)]
pub struct Vector<T>
where
    T: Clone,
{
    inner: Vec<DataNode<T>>,
}

impl<T> Serialize for Vector<T>
where
    T: Serialize + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Vector<T>
where
    T: Deserialize<'de> + Clone,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner = Vec::<DataNode<T>>::deserialize(deserializer)?;
        Ok(Vector { inner })
    }
}

impl<T: Clone> Vector<T> {
    pub fn new() -> Self {
        Vector { inner: Vec::new() }
    }

    pub fn insert(&mut self, value: DataNode<T>) {
        self.inner.push(value);
    }

    pub fn get(&mut self, id: &str) -> Option<&mut DataNode<T>> {
        self.inner.iter_mut().find(|n| n.id == id)
    }

    pub fn delete(&mut self, id: &str) {
        self.inner.retain(|n| n.id != id);
    }

    pub fn update(&mut self, id: &str, value: DataNode<T>) {
        self.get(id).map(|n| *n = value);
    }
}
