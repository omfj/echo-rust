use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TOMLPost {
    pub id: String,
    pub title: String,
    pub description: String,
    pub date: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TOMLConfig {
    pub title: String,
    pub posts: Vec<TOMLPost>,
}
