use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use crate::{graph::Collection, store::Store};

pub trait Flew<T: Store<T>> {
    fn node<U>(&self, node_name: &str) -> Collection<U, T>
    where
        U: Clone + for<'de> Deserialize<'de> + Serialize;
}

#[derive(Debug, Clone)]
pub struct EmbeddedFlew<T> {
    store: Arc<RwLock<T>>,
}

impl<T> EmbeddedFlew<T>
where
    T: Store<T>,
{
    pub fn new(store: T) -> Self {
        EmbeddedFlew {
            store: Arc::new(RwLock::new(store)),
        }
    }
}

impl<T> Flew<T> for EmbeddedFlew<T>
where
    T: Store<T> + Default,
{
    fn node<U>(&self, node_name: &str) -> Collection<U, T>
    where
        U: Clone + for<'de> Deserialize<'de> + Serialize,
    {
        let store_read = self.store.read().unwrap();
        if let Ok(node) = store_read.read::<Collection<U, T>>() {
            Collection::from(
                node.nodes,
                node.edges,
                Some(node_name.to_string()),
                Some(self.store.clone()),
            )
        } else {
            Collection::new(Some(node_name.to_string()), Some(self.store.clone()))
        }
    }
}
