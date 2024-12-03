use flew_core::{
    flew::{self, Node},
    store::{JsonStore, Store},
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
    // let mut db = flew::Flew::new("");
    // let user = User {
    //     name: "John".to_string(),
    //     age: 30,
    // };
    // let user1 = User {
    //     name: "Marry".to_string(),
    //     age: 20,
    // };

    // let post = Post {
    //     title: "Hello World".to_string(),
    //     body: "This is my first post".to_string(),
    // };
    // let data = Node::new(Entity::User(user));
    // let data1 = Node::new(Entity::User(user1));
    // let posts = Node::new(Entity::Post(post));

    // db.add_node("users".to_string(), vec![data]);
    // db.add_node("users".to_string(), vec![data1]);
    // db.add_node("posts".to_string(), vec![posts]);

    // let data = db.node("users").unwrap();

    // println!("{:?}", data);

    // let strore = JsonStore::new("graph.json");
    // strore.save(db);
    // // strore.save(dbPost);
}
