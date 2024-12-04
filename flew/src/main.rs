use flew_core::{
    flew::{EmbeddedFlew, Flew},
    graph::{Collection, Graph},
    node::DataNode,
    store::JsonStore,
};
use flew_macros::{flew_main, Entity};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
struct User {
    name: String,
    age: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
struct Post {
    title: String,
    body: String,
}

#[flew_main]
fn main() {
    let store = JsonStore::new("graph.json");
    let flew = EmbeddedFlew::new(store.clone());
    let data: Collection<DataNode<Entity>> = flew.node();
    let mut users = data.get_node("users");
    let user = User {
        name: "John Doe".to_string(),
        age: 30,
    };
    users.push(DataNode::new(Entity::User(user)));
    println!("Data: {:#?}", data);
}
