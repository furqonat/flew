use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node<T> {
    pub id: String,
    pub data: T,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        let id = Uuid::new_v4().to_string();
        Node { id, data }
    }

    pub fn new_with_id(id: String, data: T) -> Self {
        Node { id, data }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Flew<T> {
    nodes: HashMap<String, Vec<Node<T>>>,
    edges: HashMap<String, HashSet<String>>,
}

impl<T> Flew<T>
where
    T: Clone,
{
    pub fn new(node_name: String) -> Self {
        Flew {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn node(&self, node_name: &str) -> Option<&Vec<Node<T>>> {
        return self.nodes.get(node_name);
    }

    pub fn add_node(&mut self, node_name: String, node: Vec<Node<T>>) -> String {
        if self.nodes.contains_key(&node_name) {
            self.nodes.get_mut(&node_name).unwrap().extend(node);
            return node_name;
        } else {
            self.nodes.insert(node_name.to_string().clone(), node);
            return node_name;
        }
    }

    pub fn add_edge(&mut self, source: &str, target: &str) {
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

    pub fn remove_node(&mut self, identifier: &str) {
        if !self.nodes.contains_key(identifier) {
            return;
        }
    }

    pub fn remove_edge(&mut self, source: &str, target: &str) {
        if !self.edges.contains_key(source) {
            return;
        }
        self.edges.get_mut(source).unwrap().remove(target);
    }
}
