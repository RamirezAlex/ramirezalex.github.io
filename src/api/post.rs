use crate::types::post::Post;
use crate::types::post::PostMeta;
#[cfg(feature = "ssr")]
use gray_matter::engine::TOML;
#[cfg(feature = "ssr")]
use gray_matter::Matter;
use leptos::*;
#[cfg(feature = "ssr")]
use std::fs;
#[cfg(feature = "ssr")]
use std::path::Path;

#[server]
pub async fn get_posts_list() -> Result<Vec<PostMeta>, ServerFnError> {
    get_posts_list_from_fs().map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server]
pub async fn get_post(slug: String) -> Result<Post, ServerFnError> {
    get_post_by_slug(&slug).map_err(|_| {
        ServerFnError::ServerError(String::from("Failed to get post by slug"))
    })
}

#[cfg(feature = "ssr")]
fn get_posts_list_from_fs() -> Result<Vec<PostMeta>, Box<dyn std::error::Error>> {
    let path = match Path::new("./content/blog/").canonicalize() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to canonicalize path: {}", e);
            return Ok(Vec::new());
        }
    };

    let paths = match fs::read_dir(path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to read directory: {}", e);
            return Ok(Vec::new());
        }
    };

    let posts = paths
        .filter_map(|entry| {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("Failed to read entry: {}", e);
                    return None; // Skip this entry
                }
            };
            let path_str = entry.path().to_str()?.to_string(); // Skip if path is not valid UTF-8
            let slug = path_str.split('/').last()?.split('.').next()?; // Skip if path is not valid UTF-8
            let mut post = get_post_by_slug(slug).ok()?; // Skip if post is not valid
            post.meta.slug = slug.to_string();
            Some(post.meta) // Return the successfully parsed Post
        })
        .collect();

    Ok(posts)
}

#[cfg(feature = "ssr")]
fn get_post_by_slug(slug: &str) -> Result<Post, Box<dyn std::error::Error>> {
    let matter = Matter::<TOML>::new();

    let file_path = Path::new("./content/blog/")
        .canonicalize()
        .map_err(|e| format!("Failed to canonicalize path: {}", e))?;

    let file_path_str = file_path
        .to_str()
        .ok_or("Failed to convert path to string")?;

    let full_file_path = format!("{}/{}.md", file_path_str, slug);

    let file_content = fs::read_to_string(&full_file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", full_file_path, e))?;

    let post = matter
        .parse_with_struct::<PostMeta>(&file_content)
        .ok_or_else(|| "Failed to parse post".to_string())?;

    Ok(Post {
        meta: post.data,
        content: post.content,
    })
}
