use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PostMeta {
    #[serde(default)]
    pub slug: String,
    pub title: String,
    pub date: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Post {
    pub meta: PostMeta,
    pub content: String,
}
