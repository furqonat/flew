use flew_core::{
    flew::{EmbeddedFlew, Flew},
    graph::Collection,
    node::{DataNode, Node},
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
    let db: EmbeddedFlew<JsonStore> = EmbeddedFlew::new(store);
    let post = Post {
        title: "Hello, world!".to_string(),
        body: "This is a test post.".to_string(),
    };
    let post_node = DataNode::new(Entity::Post(post));
    let update_post = Post {
        title: "Hello, world!".to_string(),
        body: "This is an updated test post.".to_string(),
    };
    let update_post_node = DataNode::new(Entity::Post(update_post));
    // let x = db
    //     .node("posts")
    //     .delete("64a23758-a52f-40d3-b881-9094f597a686");

    let mut m: Collection<Entity, JsonStore> = db.node("posts");

    let x = m.get("64a23758-a52f-40d3-b881-9094f597a686");
    println!("{:?}", x);

    let user = User {
        name: "John Doe".to_string(),
        age: 30,
    };
    let user_node = DataNode::new(Entity::User(user));
    db.node("users").add(user_node);
    db.node("posts").add(post_node);
}
