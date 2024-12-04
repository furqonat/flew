use serde::{Deserialize, Serialize};

use crate::{graph::Collection, store::Store};

pub trait Flew {
    fn node<U>(&self) -> Collection<U>
    where
        U: for<'de> Deserialize<'de> + Serialize;
}

#[derive(Debug, Default)]
pub struct EmbeddedFlew<T> {
    store: T,
}

impl<T> EmbeddedFlew<T>
where
    T: Store<T>,
{
    pub fn new(store: T) -> Self {
        EmbeddedFlew { store }
    }
}

impl<T> Flew for EmbeddedFlew<T>
where
    T: Store<T>,
{
    fn node<U>(&self) -> Collection<U>
    where
        U: for<'de> Deserialize<'de> + Serialize,
    {
        let node = self.store.load();
        Collection::from(node.nodes, node.edges)
    }
}
