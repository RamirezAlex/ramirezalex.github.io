use crate::api::post::get_posts_list;
use leptos::*;
use leptos_router::*;

#[component]
pub fn PostsList() -> impl IntoView {
    let posts = get_posts_list();

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
