use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct PostMeta {
    pub title: String,
    pub date: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Post {
    pub meta: PostMeta,
    pub content: String,
}
