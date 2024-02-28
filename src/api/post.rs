use crate::types::post::Post;
use crate::types::post::PostMeta;
use gray_matter::engine::TOML;
use gray_matter::Matter;
use leptos::*;
use std::fs;
use std::path::Path;

pub fn get_posts_list() -> Vec<PostMeta> {
    let path = match Path::new("./content/blog/").canonicalize() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to canonicalize path: {}", e);
            return Vec::new();
        }
    };

    let paths = match fs::read_dir(path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to read directory: {}", e);
            return Vec::new();
        }
    };

    paths
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
            let post = get_post_by_slug(slug).ok()?; // Skip if post is not valid
            Some(post.meta) // Return the successfully parsed Post
        })
        .collect()
}

pub fn get_post(slug: &str) -> Result<Post, ServerFnError> {
    match get_post_by_slug(slug) {
        Ok(p) => Ok(p),
        Err(_) => Err(ServerFnError::ServerError(String::from(
            "Failed to get post by slug",
        ))),
    }
}

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
