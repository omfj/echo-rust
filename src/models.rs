use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RSSToml {
    pub title: String,
    pub posts: Vec<Post>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
}
