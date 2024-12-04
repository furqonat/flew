use flew_core::{
    flew::{EmbeddedFlew, Flew},
    graph::Graph,
    node::DataNode,
    store::BinaryStore,
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
    let store = BinaryStore::new("graph.bin");
    let db = EmbeddedFlew::new(store.clone());
    let mut collection = db.collection();
    collection.add_node("posts".to_string());
    let post = Post {
        title: "Hello, world!".to_string(),
        body: "This is a test post.".to_string(),
    };
    let post_node = DataNode::new(Entity::Post(post));
    if let Some(posts) = collection.get_node_mut("posts") {
        posts.push(post_node);
    }
    println!("Data: {:#?}", collection);
    db.sync(collection);
}
