# Flew Nosql Database

Flew is an open source nosql database written in rust.

## Example

```rust
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
    let _ = db
        .node("posts")
        .update("ba2e5ac8-ce9a-4e21-84f8-2a6c6e64d602", update_post_node);

    // get a node data by id
    let m: Collection<Entity, JsonStore> = db.node("posts");
    let x = m.get("10ae169e-2ccf-4ad1-bf54-e2bc68911b78");
    println!("data is {:?}", x);

    let user = User {
        name: "John Doe".to_string(),
        age: 30,
    };
    let user_node = DataNode::new(Entity::User(user));

    // create a node and add some data
    let _ = db.node("users").add(user_node);
    let _ = db.node("posts").add(post_node);

    // delete a node
    let mut j: Collection<Entity, JsonStore> = db.node("posts");
    let _ = j.delete("64a23758-a52f-40d3-b881-9094f597a686");
}

```
