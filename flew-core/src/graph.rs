use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    io::{Error, ErrorKind},
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};

use crate::{
    node::{DataNode, Node},
    store::Store,
    vector::Vector,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection<N: Clone, T> {
    pub nodes: HashMap<String, Vector<N>>,
    pub edges: HashMap<String, HashSet<String>>,
    #[serde(skip_serializing, skip_deserializing)]
    node: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    db: Option<Arc<RwLock<T>>>,
}

impl<N, T> Node<N> for Collection<N, T>
where
    N: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
    T: Store<T> + Default,
{
    fn add(&mut self, data: DataNode<N>) -> Result<String, Error> {
        if let Some(node) = self.node.as_ref() {
            match self.nodes.get_mut(node) {
                Some(vector) => {
                    vector.insert(data.clone());
                    self.sync();
                    Ok(data.id)
                }
                None => {
                    self.nodes.insert(node.clone(), Vector::new());
                    self.nodes.get_mut(node).unwrap().insert(data.clone());
                    self.sync();
                    Ok(data.id)
                }
            }
        } else {
            Err(Error::new(ErrorKind::Other, "No node selected"))
        }
    }

    fn get(&self, id: &str) -> Option<&DataNode<N>> {
        if let Some(node) = self.node.as_ref() {
            if let Some(vector) = self.nodes.get(node) {
                return vector.get(id);
            }
            return None;
        }
        None
    }

    fn delete(&mut self, id: &str) -> Result<(), Error> {
        if let Some(node) = self.node.as_ref() {
            match self.nodes.get_mut(node) {
                Some(vector) => {
                    vector.delete(id);
                    self.sync();
                    Ok(())
                }
                None => Err(Error::new(ErrorKind::Other, "Node not found")),
            }
        } else {
            Err(Error::new(ErrorKind::Other, "No node selected"))
        }
    }

    fn update(&mut self, id: &str, data: DataNode<N>) -> Result<(), Error> {
        if let Some(node) = self.node.as_ref() {
            self.nodes.get_mut(node).unwrap().update(id, data);
            self.sync();
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "No node selected"))
        }
    }
}

impl<N, T> Collection<N, T>
where
    N: Clone + Serialize + for<'de> Deserialize<'de>,
    T: Store<T> + Default,
{
    pub(crate) fn new(node_name: Option<String>, db: Option<Arc<RwLock<T>>>) -> Self {
        Collection {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            node: node_name,
            db,
        }
    }

    pub fn from(
        nodes: HashMap<String, Vector<N>>,
        edges: HashMap<String, HashSet<String>>,
        node_name: Option<String>,
        db: Option<Arc<RwLock<T>>>,
    ) -> Self {
        Collection {
            nodes,
            edges,
            node: node_name,
            db,
        }
    }

    pub fn as_mut(&mut self) -> &mut Self {
        self
    }

    fn sync(&self) {
        if let Some(db) = self.db.as_ref() {
            let store: std::sync::RwLockWriteGuard<'_, T> = db.write().unwrap();
            let col = Collection::from(
                self.nodes.clone(),
                self.edges.clone(),
                self.node.clone(),
                self.db.clone(),
            );
            let result = store.write::<Collection<N, T>>(col);
            match result {
                Ok(_) => {}
                Err(e) => println!("Failed to write to store: {}", e),
            }
        }
    }
}

impl<N: Clone, T> Default for Collection<N, T> {
    fn default() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Default::default(),
            node: Default::default(),
            db: Default::default(),
        }
    }
}
