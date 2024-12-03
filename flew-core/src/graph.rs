use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::node;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node<T> {
    pub id: String,
    pub data: T,
}
pub trait Graph<T> {
    fn item(&self, id: &str) -> Option<Vec<Node<T>>>;
    fn add_node(&mut self, id: String, node: Vec<Node<T>>) -> String;
    fn add_edge(&mut self, source: &str, target: &str);
    fn remove_node(&mut self, id: &str);
    fn remove_edge(&mut self, source: &str, target: &str);
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Flew<T> {
    nodes: Option<HashMap<String, Vec<Node<T>>>>,
    edges: Option<HashMap<String, HashSet<String>>>,
    node_name: String,
}

impl<T> Flew<T> {
    pub fn new(node_name: String) -> Self {
        Flew {
            nodes: None,
            edges: None,
            node_name,
        }
    }
}

impl<T> Graph<T> for Flew<T>
where
    T: Clone,
{
    fn add_edge(&mut self, source: &str, target: &str) {
        todo!()
    }

    fn remove_node(&mut self, id: &str) {
        todo!()
    }

    fn remove_edge(&mut self, source: &str, target: &str) {
        todo!()
    }

    fn item(&self, id: &str) -> Option<Vec<Node<T>>> {
        match Some(self.nodes.clone()) {
            None => self.nodes.as_ref().unwrap().get(&self.node_name).cloned(),
            Some(nodes) => nodes.unwrap().get(&self.node_name).cloned(),
        }
    }

    fn add_node(&mut self, id: String, node: Vec<Node<T>>) -> String {
        todo!()
    }
}
