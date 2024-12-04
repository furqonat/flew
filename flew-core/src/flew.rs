use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use crate::{graph::Collection, store::Store};

pub trait Flew {
    fn collection<U>(&self) -> Collection<U>
    where
        U: for<'de> Deserialize<'de> + Serialize;
    fn sync<U>(&self, collection: Collection<U>)
    where
        U: for<'de> Deserialize<'de> + Serialize;
}

#[derive(Debug, Default)]
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

impl<T> Flew for EmbeddedFlew<T>
where
    T: Store<T>,
{
    fn collection<U>(&self) -> Collection<U>
    where
        U: for<'de> Deserialize<'de> + Serialize,
    {
        let node = self.store.read().unwrap().read();
        match node {
            Ok(node) => Collection::from(node.nodes, node.edges),
            Err(_e) => Collection::new(),
        }
    }

    fn sync<U>(&self, collection: Collection<U>)
    where
        U: for<'de> Deserialize<'de> + Serialize,
    {
        let store = self.store.write().unwrap();
        let result = store.write(collection);
        match result {
            Ok(_) => {}
            Err(e) => println!("Failed to write to store: {}", e),
        }
    }
}
