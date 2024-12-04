use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

pub trait Graph<T> {
    fn add_node(&mut self, node_name: String) -> String;
    fn remove_node(&mut self, node_name: &str);
    fn get_node(&self, node_name: &str) -> Vec<T>;
    fn add_edge(&mut self, source: &str, target: &str);
    fn remove_edge(&mut self, source: &str, target: &str);
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Collection<T> {
    pub nodes: HashMap<String, Vec<T>>,
    pub edges: HashMap<String, HashSet<String>>,
}

impl<T> Collection<T> {
    pub fn from(nodes: HashMap<String, Vec<T>>, edges: HashMap<String, HashSet<String>>) -> Self {
        Collection { nodes, edges }
    }
}

impl<T> Graph<T> for Collection<T>
where
    T: Clone + for<'de> Deserialize<'de> + Serialize,
{
    fn add_node(&mut self, node_name: String) -> String {
        self.nodes.insert(node_name.clone(), Vec::new());
        node_name
    }

    fn remove_node(&mut self, node_name: &str) {
        self.nodes.remove(node_name);
    }

    fn add_edge(&mut self, source: &str, target: &str) {
        if !self.nodes.contains_key(source) || !self.nodes.contains_key(target) {
            return;
        }

        if !self.edges.contains_key(source) {
            self.edges.insert(source.to_string(), HashSet::new());
        }
        self.edges
            .get_mut(source)
            .unwrap()
            .insert(target.to_string());
    }

    fn remove_edge(&mut self, source: &str, target: &str) {
        if !self.edges.contains_key(source) {
            return;
        }
        self.edges.get_mut(source).unwrap().remove(target);
    }

    fn get_node(&self, node_name: &str) -> Vec<T> {
        self.nodes.get(node_name).cloned().unwrap_or_else(Vec::new)
    }
}
