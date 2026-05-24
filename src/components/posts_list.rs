use crate::api::post::get_posts_list;
use leptos::*;
use leptos_router::*;

#[component]
pub fn PostsList() -> impl IntoView {
    let posts = create_resource(|| (), |_| async { get_posts_list().await });

    view! {
        <ul class="page-blog-index">
            <Suspense fallback=move || view! { <li class="blog-list-status">"Loading posts..."</li> }>
                {move || {
                    posts.get().map(|posts| {
                        match posts {
                            Ok(posts) if posts.is_empty() => {
                                view! { <li class="blog-list-status">"No posts yet."</li> }.into_view()
                            }
                            Ok(posts) => posts
                                .into_iter()
                                .map(|post| {
                                    let title = post.title.clone();
                                    let description = post.description.unwrap_or_default();
                                    let image_url = post.image_url.unwrap_or_default();
                                    let date = post.date.clone();

                                    view! {
                                        <li class="blog-card">
                                            <A class="blog-card-link" href={format!("/blog/{}", post.slug)}>
                                                <div class="blog-card-image">
                                                    <img src={image_url} alt={title.clone()}/>
                                                </div>
                                                <div class="blog-card-body">
                                                    <time>{date}</time>
                                                    <h2>{title}</h2>
                                                    <p>{description}</p>
                                                </div>
                                            </A>
                                        </li>
                                    }
                                })
                                .collect_view(),
                            Err(_) => view! { <li class="blog-list-status">"Could not load posts."</li> }.into_view(),
                        }
                    })
                }}
            </Suspense>
        </ul>
    }
}
