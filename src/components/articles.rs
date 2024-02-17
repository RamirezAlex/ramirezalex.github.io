use crate::types::blog_post::BlogPost;
use konfiguration::Konfiguration;
use leptos::*;
use leptos_router::*;
use std::fs;
use std::path::Path;

fn get_blog_post_list() -> Vec<BlogPost> {
    let directory = "./content/blog/";
    let path = match Path::new(directory).canonicalize() {
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
            let conf = Konfiguration::from_file(format!("{}/index.toml", path_str))
                .parse::<BlogPost>()
                .ok()?; // Skip if parsing fails
            Some(conf) // Return the successfully parsed BlogPost
        })
        .collect()
}

#[component]
pub fn Articles() -> impl IntoView {
    let posts = get_blog_post_list(); // Assuming this returns Vec<BlogPost>

    // let posts: Vec<BlogPost> = (0..16)
    //     .flat_map(|_| posts.clone()) // Clone `posts` for each iteration
    //     .collect();

    view! {
        <ul class="page-blog-index">
            {
                posts.iter().map(|post| {
                    let blog_url = "/blog";
                    let title = post.title.clone();
                    let description = post.description.clone();
                    let image_url = post.image_url.clone();
                    let title_slug = title.to_lowercase()
                        .replace(' ', "-")
                        .chars()
                        .filter(|c| c.is_alphanumeric() || *c == '-')
                        .collect::<String>();

                    view! {
                        <A href={format!("{}/{}", blog_url, title_slug)}>
                            <li style="display: inline-block">
                                <div>
                                    <img src={image_url.clone()} alt={title.clone()}/>
                                    <h2>{title}</h2>
                                    <p>{description.clone()}</p>
                                </div>
                            </li>
                        </A>
                    }
                }).collect::<Vec<_>>()
            }
        </ul>
    }
}
