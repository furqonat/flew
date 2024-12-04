use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::node::DataNode;

pub trait Graph<T> {
    fn add_node(&mut self, node_name: String) -> String;
    fn remove_node(&mut self, node_name: &str);
    fn get_node(&self, node_name: &str) -> Option<&Vec<DataNode<T>>>;
    fn get_node_mut(&mut self, node_name: &str) -> Option<&mut Vec<DataNode<T>>>;
    fn add_edge(&mut self, source: &str, target: &str);
    fn remove_edge(&mut self, source: &str, target: &str);
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Collection<T> {
    pub nodes: HashMap<String, Vec<DataNode<T>>>,
    pub edges: HashMap<String, HashSet<String>>,
}

impl<T> Collection<T> {
    pub fn new() -> Self {
        Collection {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn from(
        nodes: HashMap<String, Vec<DataNode<T>>>,
        edges: HashMap<String, HashSet<String>>,
    ) -> Self {
        Collection { nodes, edges }
    }
}

impl<T> Graph<T> for Collection<T>
where
    T: Clone + for<'de> Deserialize<'de> + Serialize,
{
    fn add_node(&mut self, node_name: String) -> String {
        self.nodes.entry(node_name.clone()).or_insert_with(Vec::new);
        node_name
    }

    fn remove_node(&mut self, node_name: &str) {
        self.nodes.remove(node_name);
        self.edges.remove(node_name);

        for (_, targets) in self.edges.iter_mut() {
            targets.remove(node_name);
        }
    }

    fn add_edge(&mut self, source: &str, target: &str) {
        if self.nodes.contains_key(source) && self.nodes.contains_key(target) {
            self.edges
                .entry(source.to_string())
                .or_insert_with(HashSet::new)
                .insert(target.to_string());
        }
    }

    fn remove_edge(&mut self, source: &str, target: &str) {
        if let Some(targets) = self.edges.get_mut(source) {
            targets.remove(target);
        }
    }

    fn get_node(&self, node_name: &str) -> Option<&Vec<DataNode<T>>> {
        self.nodes.get(node_name)
    }

    fn get_node_mut(&mut self, node_name: &str) -> Option<&mut Vec<DataNode<T>>> {
        self.nodes.get_mut(node_name)
    }
}
