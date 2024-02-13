use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct BlogPost {
    pub title: String,
    pub date: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}
